//! TOML-based asset loader for [`SkillDefinition`].
//!
//! Provides [`SkillDefinitionLoader`] — a Bevy [`AssetLoader`] that reads
//! files with the extension `.skill.toml` and produces a [`SkillDefinition`]
//! asset.
//!
//! # TOML format
//!
//! ```toml
//! [skill]
//! id = "archer_shot"
//! name = "Archer Shot"
//!
//! [[features]]
//! id = "cooldown"
//! cooldown_duration = 1.0
//!
//! [[features]]
//! id = "damage"
//! base_damage = 50.0
//! ratio = 1.5
//! ```

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    ecs::error::BevyError,
    prelude::*,
    tasks::ConditionalSendFuture,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{SkillDefinition, SkillEffectDefinition, SkillFeatureDefinition};

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

/// Loads [`SkillDefinition`] from `.skill.toml` files.
///
/// See the [module-level documentation](self) for the expected TOML format.
#[derive(TypePath)]
pub struct SkillDefinitionLoader;

/// No-op settings for [`SkillDefinitionLoader`].
#[derive(Default, Clone, Serialize, Deserialize, TypePath)]
pub struct SkillDefinitionLoaderSettings;

impl AssetLoader for SkillDefinitionLoader {
    type Asset = SkillDefinition;
    type Settings = SkillDefinitionLoaderSettings;
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
            let skill = parse_toml(&content).map_err(BevyError::from)?;
            Ok(skill)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["skill.toml"]
    }
}

// ---------------------------------------------------------------------------
// TOML parser
// ---------------------------------------------------------------------------

/// Intermediate deserialization structure matching the TOML format.
#[derive(Deserialize)]
struct SkillDefinitionToml {
    skill: SkillHeader,
    #[serde(default)]
    features: Vec<HashMap<String, toml::Value>>,
    #[serde(default)]
    effects: Vec<HashMap<String, toml::Value>>,
}

#[derive(Deserialize)]
struct SkillHeader {
    id: String,
    name: String,
}

/// Parses TOML content into a [`SkillDefinition`].
fn parse_toml(content: &str) -> Result<SkillDefinition, Box<dyn std::error::Error + Send + Sync>> {
    let raw: SkillDefinitionToml = toml::from_str(content)?;

    let mut definition = SkillDefinition::new(raw.skill.id, raw.skill.name);

    for feature_map in raw.features {
        let feature_id = feature_map
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or("each [[features]] entry must have an `id` field")?
            .to_string();

        let mut feature = SkillFeatureDefinition::new(feature_id);

        for (key, value) in &feature_map {
            if key == "id" {
                continue;
            }
            if let Some(num) = value.as_float().or_else(|| value.as_integer().map(|i| i as f64)) {
                feature.set(key, num as f32);
            }
        }

        definition.add_feature(feature);
    }

    for effect_map in raw.effects {
        let effect_id = effect_map
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or("each [[effects]] entry must have an `id` field")?
            .to_string();

        let mut effect = SkillEffectDefinition::new(effect_id);

        for (key, value) in &effect_map {
            if key == "id" {
                continue;
            }
            if let Some(num) = value.as_float().or_else(|| value.as_integer().map(|i| i as f64)) {
                effect.set(key, num as f32);
            }
        }

        definition.add_effect(effect);
    }

    Ok(definition)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_toml() {
        let toml_str = r#"
[skill]
id = "archer_shot"
name = "Archer Shot"

[[features]]
id = "cooldown"
cooldown_duration = 1.0

[[features]]
id = "damage"
base_damage = 50.0
ratio = 1.5
"#;
        let skill = parse_toml(toml_str).unwrap();
        assert_eq!(skill.id, "archer_shot");
        assert_eq!(skill.name, "Archer Shot");
        assert_eq!(skill.features.len(), 2);

        let cooldown = skill.get_feature("cooldown").unwrap();
        assert!((cooldown.get("cooldown_duration").unwrap() - 1.0).abs() < f32::EPSILON);

        let damage = skill.get_feature("damage").unwrap();
        assert!((damage.get("base_damage").unwrap() - 50.0).abs() < f32::EPSILON);
        assert!((damage.get("ratio").unwrap() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn parse_minimal_toml() {
        let toml_str = r#"
[skill]
id = "empty_skill"
name = "Empty"
"#;
        let skill = parse_toml(toml_str).unwrap();
        assert_eq!(skill.id, "empty_skill");
        assert!(skill.features.is_empty());
    }

    #[test]
    fn parse_missing_id_field() {
        let toml_str = r#"
[skill]
id = "test"
name = "Test"

[[features]]
cooldown_duration = 1.0
"#;
        let result = parse_toml(toml_str);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must have an `id` field"));
    }

    #[test]
    fn parse_integer_as_float() {
        let toml_str = r#"
[skill]
id = "test"
name = "Test"

[[features]]
id = "cooldown"
cooldown_duration = 2
"#;
        let skill = parse_toml(toml_str).unwrap();
        let cooldown = skill.get_feature("cooldown").unwrap();
        assert!((cooldown.get("cooldown_duration").unwrap() - 2.0).abs() < f32::EPSILON);
    }
}
