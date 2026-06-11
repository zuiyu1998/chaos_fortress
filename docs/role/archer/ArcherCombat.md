# ArcherCombat

`ArcherCombat` 是一个标记组件（Marker Component），通过 gearbox 的 [`StateComponent`] 机制自动插入/移除到弓箭手（状态机根）实体上，表示实体当前处于**战斗（Combat）**状态。

## 设计动机

当弓箭手的状态机进入 `Combat` 子状态时，gearbox 的 `StateComponent` 机制将 `ArcherCombat` 自动插入到状态机根实体（弓箭手本体）上；离开 `Combat` 时自动移除。这使得外部系统可以直接通过 `(With<Archer>, With<ArcherCombat>)` 查询当前战斗中的弓箭手。

## 定义

```rust
/// 标记弓箭手处于 Combat（战斗）状态。
///
/// 由 gearbox 的 StateComponent 自动管理：进入 Combat 子状态时
/// 插入到根实体上，离开 Combat 子状态时移除。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ArcherCombat;
```

## 注册

在 `ArcherPlugin` 中注册反射和 gearbox 状态组件：

```rust
impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ArcherCombat>();
        app.register_state_component::<ArcherCombat>();
    }
}
```

- `register_type` — 启用 Bevy 反射支持
- `register_state_component` — 注册 gearbox 自动插入/移除行为

## 在 setup_state_machine 中使用

在 `setup_state_machine` 中，`StateComponent(ArcherCombat)` 作为组件附加到 `Combat` 子状态实体上：

```rust
pub fn setup_state_machine(machine: Entity, commands: &mut Commands) {
    let idle = commands
        .spawn_substate(machine, (Name::new("Idle"), StateComponent(ArcherIdle)))
        .id();
    let combat = commands
        .spawn_substate(machine, (Name::new("Combat"), StateComponent(ArcherCombat)))
        .id();

    commands.spawn_transition::<Idle2Combat>(idle, combat);
    commands.entity(machine).init_state_machine(idle);
}
```

进入 Combat 时 `ArcherCombat` 被插入到 `machine`（弓箭手本体）上，离开时移除。

## 查询示例

```rust
/// 查找处于 Combat 状态的弓箭手。
fn combat_behavior_system(
    combat_archers: Query<(Entity, &EnemyTarget), (With<Archer>, With<ArcherCombat>)>,
) {
    for (entity, target) in &combat_archers {
        // entity 是弓箭手，target 是当前锁定的目标
    }
}
```

## 与现有模块的关系

| 模块 | 关系 |
|---|---|
| [`ArcherRoleBuilder`](ArcherRoleBuilder.md) | 在 `setup_state_machine` 中将 `StateComponent(ArcherCombat)` 附加到 Combat 子状态 |
| [`ArcherPlugin`](ArcherPlugin.md) | 负责注册反射和 gearbox 状态组件 |
| [`ArcherIdle`](ArcherIdle.md) | 与 `ArcherCombat` 互斥，分别标记 Idle 和 Combat 状态 |
| [`Idle2Combat`](Idle2Combat.md) | 触发 Idle → Combat 转换的消息，转换后 `ArcherCombat` 自动插入 |

## 组件生命周期

```
状态机进入 Combat
    │
    ▼
ArcherCombat 插入到弓箭手（根）实体  ◄── 可通过 (With<Archer>, With<ArcherCombat>) 查询
    │
    ▼
状态机离开 Combat（例如切换到 Idle）
    │
    ▼
ArcherCombat 从弓箭手实体移除
```

## 设计说明

- `ArcherCombat` 是一个纯粹的空标记组件，不含运行时数据。
- 生命周期由 gearbox 的 `StateComponent` 自动管理，无需手动操作。
- 与 `ArcherIdle` 互斥：同一时刻弓箭手只能处于 Idle 或 Combat 中一种状态。
- 与 `Archer` 配合使用：`Archer` 表示角色类型，`ArcherCombat` 表示当前处于战斗状态。
