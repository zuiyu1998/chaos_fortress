# EnemyBuilder

`EnemyBuilder` 是一个 trait，用于**生成敌人实体**。它将敌人实体的构建过程抽象为统一的接口，使得不同类型（普通、精英、Boss、召唤物等）可以通过同一套流程创建，同时允许各实现自定义自身所需的组件和数据。

## 设计动机

当前 `enemy::enemy()` 函数返回一个固定的 tuple bundle，所有敌人共享完全相同的组件结构。当需要引入多种敌人类型（不同外观、属性、碰撞体积、AI 模式、掉落表等）时，直接使用函数式构建会导致大量重复代码或参数爆炸。

`EnemyBuilder` 解决了这个问题：将「构建敌人实体」这一行为抽象为 trait，让每种敌人类型实现自己的构建逻辑，而调用方只需依赖 `EnemyBuilder` 接口。

## Trait 定义

```rust
/// 生成敌人实体的 trait。
///
/// 实现者负责将自身数据转换为 Bevy 的 `Entity`，
/// 可添加任意组件（外观、属性、碰撞体、物理参数、AI 模式、掉落数据等）。
pub trait EnemyBuilder: Send + Sync {
    /// 使用给定的 Commands 和上下文构建一个敌人实体并返回其 Entity ID。
    fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: EnemyBuilderContext) -> Entity;
}
```

> **说明**
>
> - `&self` 为引用传递，实现者不可变地访问自身配置数据，构建后仍可复用。
> - `commands` 单独作为参数，不放在 `EnemyBuilderContext` 中，避免生命周期参数污染上下文结构体。
> - `ctx` 中的 `position`、`parent`、`cell_size` 等是与生命周期无关的纯数据字段。
> - `Send + Sync` 约束确保 builder 可安全地跨系统传递。

## 方法详解

| 方法 | 签名 | 说明 |
|---|---|---|
| `build` | `fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: EnemyBuilderContext) -> Entity` | 通过 `commands` 执行实体创建，从 `ctx` 中获取位置、父实体和格子尺寸等信息。 |

## 使用示例

```rust
use crate::enemy::{EnemyBuilder, EnemyBuilderContext};

/// 一个简单的普通敌人构造器。
struct NormalEnemyBuilder {
    name: String,
    max_hp: f32,
    armor: f32,
    speed: f32,
}

impl EnemyBuilder for NormalEnemyBuilder {
    fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: EnemyBuilderContext) -> Entity {
        let (col, row) = ctx.position;
        let cell_size = ctx.cell_size;
        let x = col as f32 * cell_size;
        let y = -(row as f32 * cell_size);

        let mut attrs = AttributeSet::new();
        attrs.insert("hp", Attribute::new(self.max_hp));
        attrs.insert("max_hp", Attribute::new(self.max_hp));
        attrs.insert("armor", Attribute::new(self.armor));

        let mut entity = commands.spawn((
            Name::new(format!("{} ({col},{row})", self.name)),
            Enemy,
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(cell_size)),
            Transform::from_xyz(x, y, VisualDisplayLayer::Character.z_value()),
            Visibility::default(),
            RigidBody::Dynamic,
            Collider::circle(cell_size / 2.0),
            GamePhysicsLayer::enemy_layers(),
            LinearVelocity(Vec2::new(0.0, -self.speed)),
            battle(attrs),
        ));

        if let Some(parent) = ctx.parent {
            entity.set_parent(parent);
        }

        entity.id()
    }
}

// 使用
fn spawn_enemy_system(mut commands: Commands) {
    let ctx = EnemyBuilderContext {
        position: (4, 2),
        cell_size: 64.0,
        parent: Some(level_entity),
    };
    let builder = NormalEnemyBuilder {
        name: "Soldier".into(),
        max_hp: 100.0,
        armor: 10.0,
        speed: 10.0,
    };
    builder.build(&mut commands, ctx);
}
```

## 与 `EnemyBuilderContainer` 的关系

通常不直接调用 `build`，而是将实现了 `EnemyBuilder` 的类型注册到 `EnemyBuilderContainer` 中按名称调用：

```rust
fn setup(mut container: ResMut<EnemyBuilderContainer>) {
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

fn spawn(world: &mut World) {
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

## 与现有系统的关系

```
EnemyBuilder  trait
    │
    ├── NormalEnemyBuilder    — 普通敌人
    ├── EliteEnemyBuilder     — 精英敌人（带技能）
    ├── BossEnemyBuilder      — Boss（多阶段、高属性）
    └── SummonEnemyBuilder    — 召唤物（限时存在）
        │
        ▼
    commands.spawn((Enemy, Sprite, Transform, RigidBody, Collider, LinearVelocity, battle(...), ...))
        │
        ▼
    bevy ECS 中的敌人实体
```

敌人实体仍使用 `Enemy` 标记组件，`EnemyBuilder` 只是构建阶段的抽象，不影响运行时查询。现有的 `enemy::enemy()` 函数可作为默认实现或快速路径继续使用，`EnemyBuilder` 在此基础上提供更灵活的可扩展构建方式。
