//! CSV-based asset loader for [`DroppedItemTemplate`].
//!
//! Provides [`DroppedItemTemplateLoader`] — a Bevy [`AssetLoader`] that reads
//! files with the extension `.dropped_item_template.csv` and produces a
//! [`DroppedItemTemplate`] directly (which is both a [`Resource`] and an
//! [`Asset`](bevy::asset::Asset)).
//!
//! # CSV format
//!
//! The CSV file **must** have a header row followed by data rows, each with
//! exactly five columns:
//!
//! ```csv
//! name,item_type,min_amount,max_amount,weight
//! gold,gold,1,10,100.0
//! exp,exp,5,20,80.0
//! wood,material,1,3,40.0
//! ```
//!
//! | Column       | Type | Description                                      |
//! |--------------|------|--------------------------------------------------|
//! | `name`       | str  | Item identifier (key in the map)                 |
//! | `item_type`  | str  | Item type string (e.g. `"gold"`, `"exp"`)        |
//! | `min_amount` | u32  | Minimum drop amount (inclusive)                  |
//! | `max_amount` | u32  | Maximum drop amount (inclusive)                  |
//! | `weight`     | f32  | Drop weight — higher values mean higher prob.   |
//!
//! Empty lines and the header row are skipped during parsing.

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    ecs::error::BevyError,
    prelude::*,
    tasks::ConditionalSendFuture,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{DroppedItemDefinition, DroppedItemTemplate};

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

/// Loads [`DroppedItemTemplate`] from `.csv` files.
///
/// See the [module-level documentation](self) for the expected CSV format.
#[derive(TypePath)]
pub struct DroppedItemTemplateLoader;

/// No-op settings for [`DroppedItemTemplateLoader`].
#[derive(Default, Clone, Serialize, Deserialize, TypePath)]
pub struct DroppedItemTemplateLoaderSettings;

impl AssetLoader for DroppedItemTemplateLoader {
    type Asset = DroppedItemTemplate;
    type Settings = DroppedItemTemplateLoaderSettings;
    type Error = BevyError;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let content = String::from_utf8(bytes)?;
            let template = parse_csv(&content).map_err(BevyError::from)?;
            Ok(template)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["dropped_item_template.csv"]
    }
}

// ---------------------------------------------------------------------------
// CSV parser
// ---------------------------------------------------------------------------

/// Parses CSV content into a [`DroppedItemTemplate`].
///
/// The first line is treated as a header and skipped. Each subsequent
/// non-empty line must contain exactly five comma-separated values:
/// `name,item_type,min_amount,max_amount,weight`.
fn parse_csv(
    content: &str,
) -> Result<DroppedItemTemplate, Box<dyn std::error::Error + Send + Sync>> {
    let mut definitions = HashMap::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Skip the header row (line 0) and empty lines.
        if trimmed.is_empty() || line_num == 0 {
            continue;
        }

        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() != 5 {
            return Err(format!(
                "line {}: expected 5 columns (name,item_type,min_amount,max_amount,weight), got {}",
                line_num + 1,
                parts.len(),
            )
            .into());
        }

        let name = parts[0].trim().to_string();
        let item_type = parts[1].trim().to_string();
        let min_amount: u32 = parts[2].trim().parse().map_err(|e| {
            format!(
                "line {}: invalid `min_amount` value '{}': {}",
                line_num + 1,
                parts[2].trim(),
                e,
            )
        })?;
        let max_amount: u32 = parts[3].trim().parse().map_err(|e| {
            format!(
                "line {}: invalid `max_amount` value '{}': {}",
                line_num + 1,
                parts[3].trim(),
                e,
            )
        })?;
        let weight: f32 = parts[4].trim().parse().map_err(|e| {
            format!(
                "line {}: invalid `weight` value '{}': {}",
                line_num + 1,
                parts[4].trim(),
                e,
            )
        })?;

        definitions.insert(
            name.clone(),
            DroppedItemDefinition {
                name,
                item_type,
                min_amount,
                max_amount,
                weight,
            },
        );
    }

    Ok(DroppedItemTemplate { definitions })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_csv() {
        let csv = "\
name,item_type,min_amount,max_amount,weight
gold,gold,1,10,100.0
exp,exp,5,20,80.0
wood,material,1,3,40.0
";
        let template = parse_csv(csv).unwrap();
        assert_eq!(template.definitions.len(), 3);

        let gold = template.definitions.get("gold").unwrap();
        assert_eq!(gold.name, "gold");
        assert_eq!(gold.item_type, "gold");
        assert_eq!(gold.min_amount, 1);
        assert_eq!(gold.max_amount, 10);
        assert_eq!(gold.weight, 100.0);

        let exp = template.definitions.get("exp").unwrap();
        assert_eq!(exp.name, "exp");
        assert_eq!(exp.item_type, "exp");
        assert_eq!(exp.min_amount, 5);
        assert_eq!(exp.max_amount, 20);
        assert_eq!(exp.weight, 80.0);

        let wood = template.definitions.get("wood").unwrap();
        assert_eq!(wood.name, "wood");
        assert_eq!(wood.item_type, "material");
        assert_eq!(wood.min_amount, 1);
        assert_eq!(wood.max_amount, 3);
        assert_eq!(wood.weight, 40.0);
    }

    #[test]
    fn parse_csv_header_only() {
        let csv = "name,item_type,min_amount,max_amount,weight\n";
        let template = parse_csv(csv).unwrap();
        assert!(template.definitions.is_empty());
    }

    #[test]
    fn parse_csv_empty_content() {
        let csv = "";
        let template = parse_csv(csv).unwrap();
        assert!(template.definitions.is_empty());
    }

    #[test]
    fn parse_csv_wrong_column_count() {
        let csv = "name,item_type,min_amount,max_amount,weight\ngold,gold,1,10\n";
        let result = parse_csv(csv);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("expected 5 columns"));
    }

    #[test]
    fn parse_csv_invalid_number() {
        let csv = "name,item_type,min_amount,max_amount,weight\ngold,gold,abc,10,100.0\n";
        let result = parse_csv(csv);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("invalid `min_amount` value"));
    }

    #[test]
    fn parse_csv_skips_blank_lines() {
        let csv = "\
name,item_type,min_amount,max_amount,weight

gold,gold,1,10,100.0

wood,material,1,3,40.0
";
        let template = parse_csv(csv).unwrap();
        assert_eq!(template.definitions.len(), 2);
    }

    #[test]
    fn parse_csv_trims_whitespace() {
        let csv =
            "name,item_type,min_amount,max_amount,weight\n  gold  ,  gold  ,  1  ,  10  ,  100.0  \n";
        let template = parse_csv(csv).unwrap();
        let gold = template.definitions.get("gold").unwrap();
        assert_eq!(gold.min_amount, 1);
        assert_eq!(gold.max_amount, 10);
        assert_eq!(gold.weight, 100.0);
    }

    #[test]
    fn parse_csv_duplicate_name_replaces() {
        let csv = "\
name,item_type,min_amount,max_amount,weight
gold,gold,1,10,100.0
gold,gold,5,20,200.0
";
        let template = parse_csv(csv).unwrap();
        assert_eq!(template.definitions.len(), 1);
        let gold = template.definitions.get("gold").unwrap();
        assert_eq!(gold.min_amount, 5);
        assert_eq!(gold.max_amount, 20);
        assert_eq!(gold.weight, 200.0);
    }
}
