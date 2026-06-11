# ArcherRoleBuilder

`ArcherRoleBuilder` 是一个实现了 [`RoleBuilder`](../RoleBuilder.md) trait 的结构体，用于**生成弓箭手角色实体**。

## 设计动机

弓箭手是一种特殊类型的角色，在基础角色组件 `Role` 之外还需携带 `Archer` 标记组件以及远程攻击相关的属性组件。通过为弓箭手实现 `RoleBuilder`，可以将其构建逻辑集中管理，并通过 `RoleBuilderContainer` 按名称动态创建。

## 定义

```rust
use crate::role::{Archer, Role, RoleBuilder, RoleBuilderContext};

/// 构造弓箭手角色的构建器。
pub struct ArcherRoleBuilder {
    /// 弓箭手的名称。
    pub name: String,
    /// 攻击范围（像素）。
    pub attack_range: f32,
    /// 攻击间隔（秒）。
    pub attack_speed: f32,
    /// 投射物伤害值。
    pub projectile_damage: f32,
}
```

## 实现 RoleBuilder

```rust
impl RoleBuilder for ArcherRoleBuilder {
    fn build<'w, 's>(&self, commands: &'w mut Commands<'w, 's>, ctx: RoleBuilderContext) -> Entity {
        let (col, row) = ctx.position;
        let cell_size = 64.0;
        let mut entity = commands.spawn((
            Name::new(format!("Archer ({col},{row})")),
            Role,              // 基础角色标记
            Archer,            // 弓箭手标记
            Sprite::from_color(Color::srgb(0.0, 0.8, 0.2), Vec2::splat(cell_size)),
            Transform::from_xyz(col as f32 * cell_size, -(row as f32 * cell_size), 1.0),
            Visibility::default(),
            RigidBody::Kinematic,
            Collider::circle(cell_size / 2.0),
            GamePhysicsLayer::character_layers(),
            AttackRange(self.attack_range),
            AttackSpeed(self.attack_speed),
            ProjectileDamage(self.projectile_damage),
        ));

        if let Some(parent) = ctx.parent {
            entity.set_parent(parent);
        }

        entity.id()
    }
}
```

> **说明**
>
> - 弓箭手使用绿色精灵（`Color::srgb(0.0, 0.8, 0.2)`）以区别于其他角色类型。
> - 除 `Role` 和 `Archer` 标记组件外，还附加了 `AttackRange`、`AttackSpeed`、`ProjectileDamage` 三个属性组件。
> - 物理组件与基础角色一致（`RigidBody::Kinematic`、圆形碰撞体、角色碰撞层）。

## 预注册到 RoleBuilderContainer

`ArcherRoleBuilder` 已在 `RoleBuilderContainer::new()` 中预注册，默认配置如下：

| 参数 | 默认值 |
|---|---|
| `name` | `"Archer"` |
| `attack_range` | `300.0` |
| `attack_speed` | `0.8` |
| `projectile_damage` | `15.0` |

因此 `"archer"` 构建器无需额外注册即可使用。如需不同配置，可再次调用 `register` 覆盖：

```rust
fn setup_custom_archer(mut container: ResMut<RoleBuilderContainer>) {
    container.register("elite_archer", ArcherRoleBuilder {
        name: "Elite Archer".into(),
        attack_range: 400.0,
        attack_speed: 1.2,
        projectile_damage: 30.0,
    });
}
```

## 使用示例

```rust
// 在系统或 exclusive system 中使用
fn spawn_archer(world: &mut World) {
    let mut commands = Commands::new(world);
    let container = world.resource::<RoleBuilderContainer>();
    let ctx = RoleBuilderContext {
        position: (3, 4),
        parent: None,
    };
    container.build("archer", &mut commands, ctx);
}
```

## 建议的属性组件

以下为 `ArcherRoleBuilder::build` 中使用的属性组件定义示例：

```rust
/// 攻击范围（像素）。
#[derive(Component, Debug, Clone, Copy)]
pub struct AttackRange(pub f32);

/// 攻击间隔（秒）。
#[derive(Component, Debug, Clone, Copy)]
pub struct AttackSpeed(pub f32);

/// 投射物伤害值。
#[derive(Component, Debug, Clone, Copy)]
pub struct ProjectileDamage(pub f32);
```

> 以上组件定义仅作示例，实际实现时可放在 `src/role/archer.rs` 或 `src/role/` 下的独立文件中。

## setup_state_machine

在弓箭手实体创建后，可通过 `setup_state_machine` 为其附加状态机组件，管理**静止**和**战斗**两种核心状态的切换：

```rust
use bevy_gearbox::prelude::*;
use bevy::prelude::*;

/// 为弓箭手实体附加状态机组件。
///
/// 调用此函数后，实体会获得 `StateMachine` 组件，并添加
/// Idle / Combat 两个子状态实体及两者间的自动过渡。
/// 状态机起始状态为 `Idle`。
pub fn setup_state_machine(machine: Entity, commands: &mut Commands) {
    // 子状态实体
    let idle = commands
        .spawn_substate(machine, Name::new("Idle"))
        .id();
    let combat = commands
        .spawn_substate(machine, Name::new("Combat"))
        .id();

    // 自动过渡：Combat → Idle（条件满足后返回静止状态）
    commands.spawn_transition_always(combat, idle);

    // 初始化状态机，起始状态为 Idle
    commands.entity(machine).init_state_machine(idle);
}
```

### 状态说明

| 状态实体 | 含义 |
|---|---|
| `Idle` | 静止状态，待机巡逻或等待目标 |
| `Combat` | 战斗状态，执行攻击行为 |

过渡使用 `AlwaysEdge`（由 `spawn_transition_always` 创建），从 Combat 回到 Idle，条件由响应系统驱动。Idle → Combat 的切换由外部系统通过发送消息触发。

> 状态机并非 `ArcherRoleBuilder::build` 的默认行为——`setup_state_machine` 旨在作为**可选的初始化步骤**，可在 `build` 返回实体后独立调用，或在专用的 spawn 系统中组合使用。

```
ArcherRoleBuilder
    │  (实现 RoleBuilder trait)
    ▼
commands.spawn((Role, Archer, AttackRange, AttackSpeed, ProjectileDamage, ...))
    │
    ├── Role       — 标记为角色
    ├── Archer     — 标记为弓箭手（文档见 [Archer.md](Archer.md)）
    └── 属性组件   — 攻击范围、攻速、伤害等运行时数据
```

`Archer` 是一个纯粹的标记组件，弓箭手的实际行为由查询 `(With<Role>, With<Archer>)` 的系统驱动，而 `ArcherRoleBuilder` 仅在构建阶段负责组装实体。
