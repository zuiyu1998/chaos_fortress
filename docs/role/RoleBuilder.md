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
pub trait RoleBuilder: Send + Sync {
    /// 使用给定的 Commands 和上下文构建一个角色实体并返回其 Entity ID。
    fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: RoleBuilderContext) -> Entity;
}
```

> **说明**
>
> - `&self` 为引用传递，实现者不可变地访问自身配置数据，构建后仍可复用。
> - `commands` 单独作为参数，不放在 `RoleBuilderContext` 中，避免生命周期参数污染上下文结构体。
> - `ctx` 中的 `position`、`parent` 等是与生命周期无关的纯数据字段。
> - `Send + Sync` 约束确保 builder 可安全地跨系统传递。

## 方法详解

| 方法 | 签名 | 说明 |
|---|---|---|
| `build` | `fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: RoleBuilderContext) -> Entity` | 通过 `commands` 执行实体创建，从 `ctx` 中获取位置和父实体等信息。 |

## 使用示例

```rust
use crate::role::{RoleBuilder, RoleBuilderContext};

// 一个简单的玩家角色构造器
struct PlayerBuilder {
    name: String,
}

impl RoleBuilder for PlayerBuilder {
    fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: RoleBuilderContext) -> Entity {
        let (col, row) = ctx.position;
        let cell_size = 64.0;
        let mut entity = commands.spawn((
            Name::new(format!("{} ({col},{row})", self.name)),
            Role,
            Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(cell_size)),
            Transform::from_xyz(col as f32 * cell_size, -(row as f32 * cell_size), 1.0),
            Visibility::default(),
            RigidBody::Kinematic,
            Collider::circle(cell_size / 2.0),
            GamePhysicsLayer::character_layers(),
        ));

        if let Some(parent) = ctx.parent {
            entity.set_parent(parent);
        }

        entity.id()
    }
}

// 使用
fn spawn_player_system(mut commands: Commands) {
    let ctx = RoleBuilderContext {
        position: (0, 9),
        parent: Some(level_entity),
    };
    let builder = PlayerBuilder { name: "Hero".into() };
    builder.build(&mut commands, ctx);
}
```

## 与 `RoleBuilderContainer` 的关系

通常不直接调用 `build`，而是将实现了 `RoleBuilder` 的类型注册到 `RoleBuilderContainer` 中按名称调用：

```rust
fn setup(mut container: ResMut<RoleBuilderContainer>) {
    container.register("hero", PlayerBuilder { name: "Hero".into() });
}

fn spawn(world: &mut World) {
    let mut commands = Commands::new(world);
    let container = world.resource::<RoleBuilderContainer>();
    let ctx = RoleBuilderContext {
        position: (0, 9),
        parent: None,
    };
    container.build("hero", &mut commands, ctx);
}
```

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
