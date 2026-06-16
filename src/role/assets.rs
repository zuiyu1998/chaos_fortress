//! Role asset definitions.
//!
//! Defines [`RoleAssets`], a resource/asset that holds handles to role-related
//! assets loaded from the filesystem, following the same pattern as
//! [`LevelAssets`](crate::level::LevelAssets).

use bevy::prelude::*;

use crate::attribute::AttributeTemplate;

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
}

impl FromWorld for RoleAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            archer_attributes: assets.load("attribute/archer.attribute_template.csv"),
        }
    }
}
