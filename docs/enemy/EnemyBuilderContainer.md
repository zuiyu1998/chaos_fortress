# EnemyBuilderContainer

`EnemyBuilderContainer` 是一个**按名称存储和查找 builder 的 Bevy `Resource`**，用于在运行时根据敌人名称查找对应的构建器并生成敌人实体。

## 设计动机

在游戏运行时，经常需要根据外部数据（如关卡配置、波次表、波次系统）动态决定创建哪种敌人。如果调用方直接依赖具体的 `NormalEnemyBuilder`、`EliteEnemyBuilder` 等类型，则每次引入新敌人类型都需修改创建逻辑。

`EnemyBuilderContainer` 将「名称 → 构建器」的映射关系集中管理，调用方只需传入敌人名称即可生成对应实体，实现了构建逻辑的完全解耦。

## 定义

```rust
use std::collections::HashMap;

/// 按名称存储 builder 的 Bevy 资源。
#[derive(Resource)]
pub struct EnemyBuilderContainer {
    builders: HashMap<
        String,
        Box<dyn for<'w, 's> Fn(&'w mut Commands<'w, 's>, EnemyBuilderContext) -> Entity + Send + Sync>,
    >,
}

impl EnemyBuilderContainer {
    pub fn new() -> Self { /* ... */ }

    /// 从实现了 `EnemyBuilder` 的类型注册。
    pub fn register(&mut self, name: impl Into<String>, builder: impl EnemyBuilder + 'static) { /* ... */ }

    /// 按名称查找并执行构建。
    pub fn build<'w, 's>(&self, name: &str, commands: &'w mut Commands<'w, 's>, ctx: EnemyBuilderContext) -> Option<Entity> { /* ... */ }
}
```

> 内部使用 `Box<dyn for<'w, 's> Fn(&'w mut Commands<'w, 's>, EnemyBuilderContext) -> Entity>` 存储。
> `Commands` 直接作为闭包参数，`EnemyBuilderContext` 包含 `position`、`cell_size` 和 `parent` 等纯数据。
> `new()` 创建空容器；各敌人插件（如 `BossPlugin`）在构建时通过 `resource_mut` 注册对应的 builder。

## 使用方式

```rust
use bevy::prelude::*;

// 在插件中插入
fn plugin(app: &mut App) {
    app.insert_resource(EnemyBuilderContainer::new())
       .add_plugins(normal::NormalEnemyPlugin) // 注册 normal 敌人 builder
       .add_systems(Startup, register_builders);
}

// 系统：注册自定义构建器
fn register_builders(mut container: ResMut<EnemyBuilderContainer>) {
    container.register("soldier", NormalEnemyBuilder {
        name: "Soldier".into(),
        max_hp: 100.0,
        armor: 10.0,
        speed: 10.0,
    });
    container.register("elite", EliteEnemyBuilder {
        name: "Elite Soldier".into(),
        max_hp: 300.0,
        armor: 30.0,
        speed: 15.0,
        skill_ids: vec!["power_attack".into()],
    });
}

// 按名称构建
fn spawn_enemies(world: &mut World) {
    let mut commands = Commands::new(world);
    let container = world.resource::<EnemyBuilderContainer>();
    let ctx = EnemyBuilderContext {
        position: (4, 2),
        cell_size: 64.0,
        parent: None,
    };
    container.build("soldier", &mut commands, ctx);
}
```

## 方法说明

| 方法 | 签名 | 说明 |
|---|---|---|
| `new` | `fn new() -> Self` | 创建空容器。Builder 由各敌人插件（如 `NormalEnemyPlugin`）注册。 |
| `register` | `fn register(&mut self, name: impl Into<String>, builder: impl EnemyBuilder + 'static)` | 从 `EnemyBuilder` 实现注册。 |
| `build` | `fn build<'w, 's>(&self, name: &str, commands: &'w mut Commands<'w, 's>, ctx: EnemyBuilderContext) -> Option<Entity>` | 按名称查找并执行构建。 |

## 完整数据流

```
关卡配置 / 波次系统
    │  ("soldier", "elite", ...)
    ▼
EnemyBuilderContainer      ← Bevy Resource（空容器，由各插件填充）
    │
    ├── "soldier"      → NormalEnemyPlugin 注册
    ├── "elite"        → ...
    └── ...
        │  (按名称查找并调用闭包)
        ▼
    commands.spawn((Enemy, Sprite, Transform, RigidBody, Collider, LinearVelocity, battle(...), ...))
        │
        ▼
    生成的敌人实体
```

## 注意事项

- `Commands` 由 `build` 调用方传入，builder 内部只使用不持有。
- `EnemyBuilderContext` 是纯数据结构，不包含任何生命周期参数。
- 注册时机建议在 `Startup` 系统或 `OnEnter` 系统中完成，避免运行时频繁注册。
