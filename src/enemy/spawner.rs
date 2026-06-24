//! Enemy spawner module.
//!
//! Provides the [`EnemySpawner`] resource and its supporting types
//! [`SpawnArea`] and [`SpawnEntry`] for configuring enemy spawning
//! within a fixed rectangular area.
//!
//! # Overview
//!
//! [`EnemySpawner`] holds a [`SpawnArea`] (the rectangle bounds), a list
//! of [`SpawnEntry`] (each specifying a builder name and interval), and a
//! `spawn_timer`. The driving system [`tick_enemy_spawner`] lives in
//! [`crate::level`] and is registered by [`crate::level::LevelPlugin`].
//!
//! # Usage
//!
//! Insert the resource in your level setup:
//!
//! ```rust
//! app.insert_resource(EnemySpawner {
//!     spawn_area: SpawnArea {
//!         col_min: 2, col_max: 6,
//!         row_min: 0, row_max: 4,
//!     },
//!     entries: vec![
//!         SpawnEntry { builder_name: "soldier".into(), count: 10, interval: 0.6 },
//!     ],
//!     spawn_timer: 0.0,
//! });
//! ```

use bevy::prelude::*;

/// 生成区域：敌人在此区域内随机选取格子坐标生成。
#[derive(Clone, Copy, Debug, Default)]
pub struct SpawnArea {
    /// 生成区域的最小列坐标（含）。
    pub col_min: u32,
    /// 生成区域的最大列坐标（含）。
    pub col_max: u32,
    /// 生成区域的最小行坐标（含）。
    pub row_min: u32,
    /// 生成区域的最大行坐标（含）。
    pub row_max: u32,
}

/// 单个敌人的生成配置。
#[derive(Clone, Debug, Default)]
pub struct SpawnEntry {
    /// 注册在 EnemyBuilderContainer 中的名称。
    pub builder_name: String,
    /// 该类型生成的总数量（当前保留字段，供扩展用）。
    #[allow(dead_code)]
    pub count: u32,
    /// 连续生成之间的间隔（秒）。
    pub interval: f32,
}

/// 管理敌人生成的 Bevy 资源。
///
/// 在固定区域 [`spawn_area`](EnemySpawner::spawn_area) 内随机选取坐标，
/// 从 [`entries`](EnemySpawner::entries) 中随机选取一个类型进行生成，
/// 每生成一个敌人后重置 [`spawn_timer`](EnemySpawner::spawn_timer) 为
/// 对应 entry 的 [`interval`](SpawnEntry::interval)。
#[derive(Resource, Default)]
pub struct EnemySpawner {
    /// 生成区域（在此矩形区域内随机选取坐标）。
    pub spawn_area: SpawnArea,
    /// 要生成的敌人配置。
    pub entries: Vec<SpawnEntry>,
    /// 生成倒计时（秒），归零时生成下一个敌人。
    pub spawn_timer: f32,
}
