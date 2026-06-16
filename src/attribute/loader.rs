//! CSV-based asset loader for [`AttributeTemplate`].
//!
//! Provides [`AttributeTemplateLoader`] — a Bevy [`AssetLoader`] that reads
//! files with the extension `.attribute_template.csv` and produces an
//! [`AttributeTemplate`] directly (which is both a [`Resource`] and an
//! [`Asset`](bevy::asset::Asset)).
//!
//! # CSV format
//!
//! The CSV file **must** have a header row followed by data rows, each with
//! exactly four columns:
//!
//! ```csv
//! name,base,min,max
//! hp,100,0,100
//! max_hp,100,0,3.40282347e+38
//! armor,30,0,3.40282347e+38
//! ```
//!
//! | Column  | Type  | Description                         |
//! |---------|-------|-------------------------------------|
//! | `name`  | str   | Attribute name (key in the map)     |
//! | `base`  | f32   | Initial base value                  |
//! | `min`   | f32   | Minimum value floor                 |
//! | `max`   | f32   | Maximum value ceiling               |
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

use super::{AttributeDefinition, AttributeTemplate};

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

/// Loads [`AttributeTemplate`] from `.csv` files.
///
/// See the [module-level documentation](self) for the expected CSV format.
#[derive(TypePath)]
pub struct AttributeTemplateLoader;

/// No-op settings for [`AttributeTemplateLoader`].
#[derive(Default, Clone, Serialize, Deserialize, TypePath)]
pub struct AttributeTemplateLoaderSettings;

impl AssetLoader for AttributeTemplateLoader {
    type Asset = AttributeTemplate;
    type Settings = AttributeTemplateLoaderSettings;
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
        &["attribute_template.csv"]
    }
}

// ---------------------------------------------------------------------------
// CSV parser
// ---------------------------------------------------------------------------

/// Parses CSV content into an [`AttributeTemplate`].
///
/// The first line is treated as a header and skipped. Each subsequent
/// non-empty line must contain exactly four comma-separated values:
/// `name,base,min,max`.
fn parse_csv(content: &str) -> Result<AttributeTemplate, Box<dyn std::error::Error + Send + Sync>> {
    let mut definitions = HashMap::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Skip the header row (line 0) and empty lines.
        if trimmed.is_empty() || line_num == 0 {
            continue;
        }

        let parts: Vec<&str> = trimmed.split(',').collect();
        if parts.len() != 4 {
            return Err(format!(
                "line {}: expected 4 columns (name,base,min,max), got {}",
                line_num + 1,
                parts.len(),
            )
            .into());
        }

        let name = parts[0].trim().to_string();
        let base: f32 = parts[1].trim().parse().map_err(|e| {
            format!(
                "line {}: invalid `base` value '{}': {}",
                line_num + 1,
                parts[1].trim(),
                e,
            )
        })?;
        let min: f32 = parts[2].trim().parse().map_err(|e| {
            format!(
                "line {}: invalid `min` value '{}': {}",
                line_num + 1,
                parts[2].trim(),
                e,
            )
        })?;
        let max: f32 = parts[3].trim().parse().map_err(|e| {
            format!(
                "line {}: invalid `max` value '{}': {}",
                line_num + 1,
                parts[3].trim(),
                e,
            )
        })?;

        definitions.insert(
            name.clone(),
            AttributeDefinition { name, base, min, max },
        );
    }

    Ok(AttributeTemplate { definitions })
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
name,base,min,max
hp,100,0,100
max_hp,100,0,3.40282347e+38
armor,30,0,3.40282347e+38
";
        let template = parse_csv(csv).unwrap();
        assert_eq!(template.definitions.len(), 3);

        let hp = template.definitions.get("hp").unwrap();
        assert_eq!(hp.name, "hp");
        assert_eq!(hp.base, 100.0);
        assert_eq!(hp.min, 0.0);
        assert_eq!(hp.max, 100.0);

        let max_hp = template.definitions.get("max_hp").unwrap();
        assert_eq!(max_hp.base, 100.0);
        assert_eq!(max_hp.min, 0.0);
        assert_eq!(max_hp.max, f32::MAX);

        let armor = template.definitions.get("armor").unwrap();
        assert_eq!(armor.base, 30.0);
        assert_eq!(armor.min, 0.0);
        assert_eq!(armor.max, f32::MAX);
    }

    #[test]
    fn parse_csv_header_only() {
        let csv = "name,base,min,max\n";
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
        let csv = "name,base,min,max\nhp,100,0\n";
        let result = parse_csv(csv);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected 4 columns"));
    }

    #[test]
    fn parse_csv_invalid_number() {
        let csv = "name,base,min,max\nhp,abc,0,100\n";
        let result = parse_csv(csv);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid `base` value"));
    }

    #[test]
    fn parse_csv_skips_blank_lines() {
        let csv = "\
name,base,min,max

hp,100,0,100

armor,30,0,3.40282347e+38
";
        let template = parse_csv(csv).unwrap();
        assert_eq!(template.definitions.len(), 2);
    }

    #[test]
    fn parse_csv_trims_whitespace() {
        let csv = "name,base,min,max\n  hp  ,  100  ,  0  ,  100  \n";
        let template = parse_csv(csv).unwrap();
        let hp = template.definitions.get("hp").unwrap();
        assert_eq!(hp.base, 100.0);
    }

    #[test]
    fn parse_csv_duplicate_name_replaces() {
        let csv = "\
name,base,min,max
hp,100,0,100
hp,200,0,500
";
        let template = parse_csv(csv).unwrap();
        assert_eq!(template.definitions.len(), 1);
        let hp = template.definitions.get("hp").unwrap();
        assert_eq!(hp.base, 200.0);
        assert_eq!(hp.max, 500.0);
    }
}
