//! Enemy asset definitions.
//!
//! Defines [`EnemyAssets`], a resource/asset that holds handles to enemy-related
//! assets loaded from the filesystem, following the same pattern as
//! [`RoleAssets`](crate::role::assets::RoleAssets).

use bevy::prelude::*;

use crate::attribute::AttributeTemplate;

/// Collection of enemy-related asset handles.
///
/// Registered via [`asset_tracking::LoadResource`] so the resource is only
/// inserted once all dependencies have finished loading.
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct EnemyAssets {
    /// Loaded attribute template for basic enemies, sourced from
    /// `assets/attribute/basic_enemy.attribute_template.csv`.
    #[dependency]
    pub basic_attributes: Handle<AttributeTemplate>,
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            basic_attributes: assets.load("attribute/basic_enemy.attribute_template.csv"),
        }
    }
}
