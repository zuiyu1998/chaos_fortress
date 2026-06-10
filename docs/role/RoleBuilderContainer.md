# RoleBuilderContainer

`RoleBuilderContainer` 是一个**按名称存储和查找 `dyn RoleBuilder` 的 Bevy `Resource`**，用于在运行时根据角色名称查找对应的构建器并生成角色实体。

## 设计动机

在游戏运行时，经常需要根据外部数据（如关卡配置、玩家选择、网络消息）动态决定创建哪种角色。如果调用方直接依赖具体的 `PlayerBuilder`、`NpcBuilder` 等类型，则每次引入新角色类型都需修改创建逻辑。

`RoleBuilderContainer` 将「名称 → 构建器」的映射关系集中管理，调用方只需传入角色名称即可生成对应实体，实现了构建逻辑的完全解耦。

## 定义

```rust
use std::collections::HashMap;

/// 按名称存储 `dyn RoleBuilder` 的 Bevy 资源。
///
/// 通过 `register` 注册构建器，通过 `build` 按名称查找并执行构建。
/// 使用前需要通过 `app.insert_resource()` 插入 ECS。
#[derive(Resource)]
pub struct RoleBuilderContainer {
    builders: HashMap<String, Box<dyn RoleBuilder>>,
}

impl RoleBuilderContainer {
    /// 创建一个空容器。
    pub fn new() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    /// 注册一个命名构建器。
    ///
    /// - `name`：角色名称，后续根据此名称查找构建器。
    /// - `builder`：实现了 `RoleBuilder` 的构建器实例。
    pub fn register(&mut self, name: impl Into<String>, builder: impl RoleBuilder + 'static) {
        self.builders.insert(name.into(), Box::new(builder));
    }

    /// 根据名称查找并执行构建。
    ///
    /// 如果未找到对应的构建器，返回 `None`。
    pub fn build(&self, name: &str, ctx: RoleBuilderContext) -> Option<Entity> {
        self.builders
            .get(name)
            .map(|builder| builder.build(ctx))
    }
}
```

## 使用方式

`RoleBuilderContainer` 本身是一个 Bevy `Resource`，需在插件中通过 `app.insert_resource()` 注册，然后通过 `Res<T>` / `ResMut<T>` 在系统中访问：

```rust
use bevy::prelude::*;

// 在插件中插入
fn plugin(app: &mut App) {
    app.insert_resource(RoleBuilderContainer::new())
       .add_systems(Startup, register_builders);
}

// 系统：注册所有角色构建器
fn register_builders(mut container: ResMut<RoleBuilderContainer>) {
    container.register("hero", PlayerBuilder { name: "Hero".into() });
    container.register("npc_merchant", NpcBuilder { npc_type: NpcType::Merchant });
}

// 系统：使用时按名称生成
fn spawn_level_roles(container: Res<RoleBuilderContainer>, mut commands: Commands) {
    let ctx = RoleBuilderContext {
        commands: &mut commands,
        position: (0, 9),
        parent: Some(level_entity),
    };
    container.build("hero", ctx);
}
```

## 方法说明

| 方法 | 签名 | 说明 |
|---|---|---|
| `new` | `fn new() -> Self` | 创建空的 `RoleBuilderContainer`。 |
| `register` | `fn register(&mut self, name: impl Into<String>, builder: impl RoleBuilder + 'static)` | 注册一个命名构建器。 |
| `build` | `fn build(&self, name: &str, ctx: RoleBuilderContext) -> Option<Entity>` | 按名称查找构建器并执行构建。未找到时返回 `None`。 |

## 完整数据流

```
关卡配置 / 玩家选择
    │  ("hero", "npc_merchant", ...)
    ▼
RoleBuilderContainer      ← Bevy Resource
    │
    ├── "hero"        → Box<dyn RoleBuilder>  (PlayerBuilder)
    ├── "npc_merchant" → Box<dyn RoleBuilder>  (NpcBuilder)
    └── ...
        │
        ▼ (通过名称查找并调用 build)
    RoleBuilder::build(&self, ctx: RoleBuilderContext)
        │
        ▼
    commands.spawn((Role, Sprite, Transform, ...))
        │
        ▼
    生成的角色实体
```

## 与 RoleBuilder 的关系

```
RoleBuilder (trait)              — 定义构建接口
    ▲
    │ 实现
    ├── PlayerBuilder
    ├── NpcBuilder
    └── ...
    │
RoleBuilderContainer (Resource)  — 管理构建器实例的注册表
    │
    ▼
按名称调用 → RoleBuilder::build(&self, ctx)
```

## 注意事项

- `dyn RoleBuilder` 要求 `RoleBuilder` trait 是对象安全的（object-safe）。当前 `build(&self, ctx)` 签名满足此条件。
- 容器本身不持有生成上下文，上下文由调用方在 `build` 时传入。
- 注册时机建议在 `Startup` 系统或 `OnEnter` 系统中完成，避免运行时频繁注册。
