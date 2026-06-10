# RoleBuilder

`RoleBuilder` 是一个 trait，用于**生成角色实体**。它将角色实体的构建过程抽象为统一的接口，使得不同角色（玩家角色、NPC、敌人变体等）可以通过同一套流程创建，同时允许各实现自定义自身所需的组件和数据。

## 设计动机

当前 `role::role()` 函数返回一个固定的 tuple bundle，所有角色共享完全相同的组件结构。当需要引入多种角色类型（不同外观、属性、碰撞体积等）时，直接使用函数式构建会导致大量重复代码或参数爆炸。

`RoleBuilder` 解决了这个问题：将「构建角色实体」这一行为抽象为 trait，让每种角色类型实现自己的构建逻辑，而调用方只需依赖 `RoleBuilder` 接口。

## Trait 定义

```rust
/// 生成角色实体的 trait。
///
/// 实现者负责将自身数据转换为 Bevy 的 `Entity`，
/// 可添加任意组件（外观、属性、碰撞体、物理参数等）。
pub trait RoleBuilder {
    /// 使用给定的上下文构建一个角色实体并返回其 `Entity` ID。
    fn build(&self, ctx: RoleBuilderContext) -> Entity;
}
```

> **说明**
>
> - `&self` 为引用传递，实现者不可变地访问自身配置数据，构建后仍可复用。
> - 如果实现者有内部可变需求（如缓存），可使用 `RefCell` 等内部可变性方案。
> - 所有外部参数（`commands`、`position`、`parent` 等）通过 [`RoleBuilderContext`](RoleBuilderContext.md) 统一传入。
> - 返回 `Entity` 使得调用方可以保留引用，后续对该实体施加额外组件或命令。

## 方法详解

| 方法 | 签名 | 说明 |
|---|---|---|
| `build` | `fn build(&self, ctx: RoleBuilderContext) -> Entity` | 从上下文中获取 `commands`、`position`、`parent` 等信息，执行实体创建。 |

## 使用示例

```rust
use crate::role::RoleBuilder;

// 一个简单的玩家角色构造器
struct PlayerBuilder {
    name: String,
}

impl RoleBuilder for PlayerBuilder {
    fn build(&self, mut ctx: RoleBuilderContext) -> Entity {
        let (col, row) = ctx.position;
        let mut entity = ctx.commands.spawn((
            Name::new(format!("{} ({col},{row})", self.name)),
            Role,
            Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(64.0)),
            Transform::from_xyz(col as f32 * 64.0, -(row as f32 * 64.0), 1.0),
            Visibility::default(),
            RigidBody::Kinematic,
            Collider::circle(32.0),
            GamePhysicsLayer::character_layers(),
        ));

        if let Some(parent) = ctx.parent {
            entity.set_parent(parent);
        }

        entity.id()
    }
}

// 使用
fn spawn_player(mut commands: Commands) {
    let ctx = RoleBuilderContext {
        commands: &mut commands,
        position: (0, 9),
        parent: Some(level_entity),
    };
    let builder = PlayerBuilder { name: "Hero".into() };
    builder.build(ctx);
}
```

## 建议实现步骤

1. 在 `src/role/mod.rs` 中定义 `RoleBuilder` trait（或单独放在 `src/role/builder.rs`）。
2. 将现有的 `role::role()` 函数重构为 `RoleBuilder` 的默认实现，或保留为辅助函数。
3. 为每种角色类型创建构造器结构体并实现 `RoleBuilder`。

## 与现有系统的关系

```
RoleBuilder  trait
    │
    ├── PlayerBuilder     — 玩家角色
    ├── NpcBuilder        — 普通 NPC
    └── ...
        │
        ▼
    commands.spawn((Role, Sprite, Transform, ...))
        │
        ▼
    bevy ECS 中的角色实体
```

角色实体仍使用 `Role` 标记组件，`RoleBuilder` 只是构建阶段的抽象，不影响运行时查询。
