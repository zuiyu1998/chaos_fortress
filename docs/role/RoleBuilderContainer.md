# RoleBuilderContainer

`RoleBuilderContainer` 是一个**按名称存储和查找 builder 的 Bevy `Resource`**，用于在运行时根据角色名称查找对应的构建器并生成角色实体。

## 设计动机

在游戏运行时，经常需要根据外部数据（如关卡配置、玩家选择、网络消息）动态决定创建哪种角色。如果调用方直接依赖具体的 `PlayerBuilder`、`NpcBuilder` 等类型，则每次引入新角色类型都需修改创建逻辑。

`RoleBuilderContainer` 将「名称 → 构建器」的映射关系集中管理，调用方只需传入角色名称即可生成对应实体，实现了构建逻辑的完全解耦。

## 定义

```rust
use std::collections::HashMap;

/// 按名称存储 builder 的 Bevy 资源。
#[derive(Resource)]
pub struct RoleBuilderContainer {
    builders: HashMap<
        String,
        Box<dyn for<'w, 's> Fn(&'w mut Commands<'w, 's>, RoleBuilderContext) -> Entity + Send + Sync>,
    >,
}

impl RoleBuilderContainer {
    pub fn new() -> Self { /* ... */ }

    /// 从实现了 `RoleBuilder` 的类型注册。
    pub fn register(&mut self, name: impl Into<String>, builder: impl RoleBuilder + 'static) { /* ... */ }

    /// 直接注册一个闭包构建器。
    pub fn register_fn<F>(&mut self, name: impl Into<String>, builder: F)
    where F: for<'w, 's> Fn(&'w mut Commands<'w, 's>, RoleBuilderContext) -> Entity + Send + Sync + 'static
    { /* ... */ }

    /// 按名称查找并执行构建。
    pub fn build<'w, 's>(&self, name: &str, commands: &'w mut Commands<'w, 's>, ctx: RoleBuilderContext) -> Option<Entity> { /* ... */ }
}
```

> 内部使用 `Box<dyn for<'w, 's> Fn(&'w mut Commands<'w, 's>, RoleBuilderContext) -> Entity>` 存储。
> `Commands` 直接作为闭包参数，`RoleBuilderContext` 仅包含 `position` 和 `parent` 等纯数据。
> `new()` 创建时已预注册一个名为 `"archer"` 的默认 [`ArcherRoleBuilder`](archer/ArcherRoleBuilder.md)。

## 使用方式

```rust
use bevy::prelude::*;

// 在插件中插入（已在 role::plugin 中自动完成）
fn plugin(app: &mut App) {
    app.insert_resource(RoleBuilderContainer::new())
       .add_systems(Startup, register_builders);
}

// 系统：注册构建器
fn register_builders(mut container: ResMut<RoleBuilderContainer>) {
    // 方式一：传入实现了 RoleBuilder 的类型
    container.register("hero", PlayerBuilder { name: "Hero".into() });

    // 方式二：直接传入闭包
    container.register_fn("npc", |commands, ctx| {
        let (col, row) = ctx.position;
        commands.spawn((
            Name::new(format!("NPC ({col},{row})")),
            Role,
            // ...
        )).id()
    });
}

// 按名称构建
fn spawn_hero(world: &mut World) {
    let mut commands = Commands::new(world);
    let container = world.resource::<RoleBuilderContainer>();
    let ctx = RoleBuilderContext {
        position: (0, 9),
        parent: None,
    };
    container.build("hero", &mut commands, ctx);
}
```

## 方法说明

| 方法 | 签名 | 说明 |
|---|---|---|
| `new` | `fn new() -> Self` | 创建容器并预注册默认的 `"archer"` 构建器。 |
| `register` | `fn register(&mut self, name: impl Into<String>, builder: impl RoleBuilder + 'static)` | 从 `RoleBuilder` 实现注册。 |
| `register_fn` | `fn register_fn<F>(&mut self, name: impl Into<String>, builder: F)` | 直接注册闭包。 |
| `build` | `fn build<'w, 's>(&self, name: &str, commands: &'w mut Commands<'w, 's>, ctx: RoleBuilderContext) -> Option<Entity>` | 按名称查找并执行构建。 |

## 完整数据流

```
关卡配置 / 玩家选择
    │  ("hero", "npc_merchant", ...)
    ▼
RoleBuilderContainer      ← Bevy Resource
    │
    ├── "archer"      → Box<dyn Fn(&mut Commands, RoleBuilderContext) -> Entity>（默认）
    ├── "hero"        → ...
    ├── "npc_merchant" → ...
    └── ...
        │  (按名称查找并调用闭包)
        ▼
    commands.spawn((Role, Sprite, Transform, ...))
        │
        ▼
    生成的角色实体
```

## 注意事项

- `Commands` 由 `build` 调用方传入，builder 内部只使用不持有。
- `RoleBuilderContext` 是纯数据结构，不包含任何生命周期参数。
- 注册时机建议在 `Startup` 系统或 `OnEnter` 系统中完成，避免运行时频繁注册。
