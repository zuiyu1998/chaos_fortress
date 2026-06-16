//! Role asset definitions.
//!
//! Defines [`RoleAssets`], a resource/asset that holds handles to role-related
//! assets loaded from the filesystem, following the same pattern as
//! [`LevelAssets`](crate::level::LevelAssets).

use bevy::prelude::*;

use crate::attribute::AttributeTemplate;
use crate::skill::SkillDefinition;

/// Collection of role-related asset handles.
///
/// Registered via [`asset_tracking::LoadResource`] so the resource is only
/// inserted once all dependencies have finished loading.
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RoleAssets {
    /// Loaded attribute template for archer entities, sourced from
    /// `assets/attribute/archer.attribute_template.csv`.
    #[dependency]
    pub archer_attributes: Handle<AttributeTemplate>,
    /// Loaded skill definition for archer entities, sourced from
    /// `assets/skill/archer.skill.toml`.
    #[dependency]
    pub archer_skill: Handle<SkillDefinition>,
}

impl FromWorld for RoleAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            archer_attributes: assets.load("attribute/archer.attribute_template.csv"),
            archer_skill: assets.load("skill/archer.skill.toml"),
        }
    }
}
