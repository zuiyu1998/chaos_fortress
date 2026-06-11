# ArcherPlugin

`ArcherPlugin` 是一个结构体，实现了 Bevy 的 `Plugin` trait，用于注册弓箭手模块所需的核心组件。

## 概述

`ArcherPlugin` 是弓箭手模块的入口插件，负责在 App 中注册与弓箭手相关的 ECS 类型，使 Bevy 的反射系统能够识别和操作它们。

```rust
pub(super) struct ArcherPlugin;

impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Archer>();
        app.register_type::<AttackSpeed>();
        app.register_type::<ProjectileDamage>();
        app.register_type::<CoolingTimer>();

        let mut container = app.world_mut().resource_mut::<RoleBuilderContainer>();
        container.register(
            "archer",
            ArcherRoleBuilder {
                name: "Archer".into(),
                attack_range: 300.0,
                attack_speed: 0.8,
                projectile_damage: 15.0,
            },
        );

        app.add_systems(
            Update,
            run_skill.run_if(in_state(Screen::Gameplay).and(in_state(Pause(false)))),
        );
    }
}
```

## 注册内容

| 注册项 | 类型 | 说明 |
|--------|------|------|
| `Archer` | Component | 标记实体为弓箭手的标记组件 |
| `AttackSpeed` | Component | 弓箭手的攻击间隔（秒） |
| `ProjectileDamage` | Component | 弓箭手投射物伤害值 |
| `CoolingTimer` | Component | 冷却计时器（Bevy `Timer`），用于控制攻击频率 |

## 系统

### run_skill

`run_skill` 是一个系统函数，用于驱动弓箭手自动发射子弹。它检索所有携带 `Archer` 标记的实体，当实体的 `CoolingTimer` 冷却完毕时，生成一颗子弹。

```rust
pub fn run_skill(
    mut commands: Commands,
    mut query: Query<(&mut CoolingTimer, &BulletPositionTarget), With<Archer>>,
    bullet_position_query: Query<&GlobalTransform, With<BulletPosition>>,
) {
    for (mut timer, target) in &mut query {
        if timer.0.just_finished() {
            timer.0.reset();

            if let Ok(transform) = bullet_position_query.get(target.0) {
                let position = transform.translation().truncate();
                commands.spawn(bullet(position, Vec2::new(0.0, 200.0)));
            }
        }
    }
}
```

### 系统运行条件

`run_skill` 应在 `Update` 调度中运行，且仅在 `Gameplay` 状态下且游戏未暂停时执行：

```rust
app.add_systems(
    Update,
    run_skill.run_if(in_state(Screen::Gameplay).and(in_state(Pause(false)))),
);
```

| 组件 | 说明 |
|------|------|
| `Archer` | 标记实体为弓箭手，用于系统查询筛选 |
| `CoolingTimer` | 冷却计时器，冷却完毕时触发攻击。每次攻击后需调用 `.reset()` 重置 |
| `BulletPositionTarget` | 指向 `BulletPosition` 子实体的 Entity 引用，用于快速获取子弹生成坐标 |
| `bullet()` | 生成子弹实体，包含 `Sprite(2×16)`、物理组件、`LinearVelocity` 等 |

## 使用方式

在 `main.rs` 的 `AppPlugin` 中与其他插件一同添加：

```rust
app.add_plugins((
    // ... 其他插件
    role::archer::ArcherPlugin,
));
```

## 与现有模块的关系

### Archer 组件

`ArcherPlugin` 注册的 `Archer` 组件用于标记弓箭手实体，配合 `ArcherRoleBuilder` 在构建时附加：

```rust
// ArcherRoleBuilder::build 中生成的实体包含 Archer 组件
commands.spawn((
    Role,
    Archer,
    AttackRange(self.attack_range),
    AttackSpeed(self.attack_speed),
    ProjectileDamage(self.projectile_damage),
    // ...
));
```

### 属性组件

`AttackSpeed` 和 `ProjectileDamage` 是弓箭手专属的属性组件，在 `archer.rs` 中定义，搭配 `AttackRange`（定义在 `common` 模块）共同描述弓箭手的远程战斗能力。

### 交互流程

1. `ArcherPlugin` 注册 `Archer`、`AttackSpeed`、`ProjectileDamage`、`CoolingTimer` 到 Bevy 世界，并在 `RoleBuilderContainer` 中注册 `ArcherRoleBuilder`
2. 角色系统通过 `RoleBuilderContainer` 调用 `ArcherRoleBuilder` 构造弓箭手实体
3. 弓箭手实体携带 `Archer` 标记、属性组件及 `CoolingTimer` 进入 ECS 世界
4. `tick_all` 系统（位于 `gameplay` 模块）每帧推进所有 `CoolingTimer`
5. `run_skill` 系统每帧检查弓箭手的冷却状态，冷却完毕时通过 `bullet()` 生成子弹

### run_skill 数据流

```
Archer 实体
    ├── Archer               — 标记为弓箭手
    ├── CoolingTimer         — 冷却计时（由 tick_all 推进）
    ├── BulletPositionTarget — 指向 BulletPosition 子实体的引用
    └── Children
        └── BulletPosition   — 提供子弹生成的世界坐标
    
    ▼ (CoolingTimer.just_finished())
    │ 通过 BulletPositionTarget.0 获取子实体 Entity
    │ 查询 GlobalTransform 获得世界坐标
    ▼
commands.spawn(bullet(position, Vec2::new(0.0, 200.0)))
    │
    ▼
Bullet 子弹实体（自动飞行碰撞）
```

## 注意事项

- `ArcherPlugin` 使用 `pub(super)` 可见性，仅在 `crate` 内部可用
- `Archer` 为标记组件，不含运行时数据；实际属性由 `AttackSpeed`、`ProjectileDamage` 等组件承载
- 后续可根据需要扩展注册更多类型（如箭矢相关的组件、事件等）
