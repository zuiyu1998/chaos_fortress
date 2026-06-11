# ArcherIdle

`ArcherIdle` 是一个标记组件（Marker Component），通过 gearbox 的 [`StateComponent`] 机制自动插入/移除到弓箭手（状态机根）实体上，表示实体当前处于**静止（Idle）**状态。

## 设计动机

当弓箭手的状态机进入 `Idle` 子状态时，gearbox 的 `StateComponent` 机制将 `ArcherIdle` 自动插入到状态机根实体（弓箭手本体）上；离开 `Idle` 时自动移除。这使得外部系统可以直接通过 `(With<Archer>, With<ArcherIdle>)` 查询当前静止的弓箭手，无需通过子状态实体间接获取。

## 定义

```rust
/// 标记弓箭手处于 Idle（静止）状态。
///
/// 由 gearbox 的 StateComponent 自动管理：进入 Idle 子状态时
/// 插入到根实体上，离开 Idle 子状态时移除。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ArcherIdle;
```

## 注册

在 `ArcherPlugin` 中注册反射和 gearbox 状态组件：

```rust
impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ArcherIdle>();
        app.register_state_component::<ArcherIdle>();
    }
}
```

- `register_type` — 启用 Bevy 反射支持
- `register_state_component` — 注册 gearbox 自动插入/移除行为

## 在 setup_state_machine 中使用

在 `setup_state_machine` 中，`StateComponent(ArcherIdle)` 作为组件附加到 `Idle` 子状态实体上：

```rust
pub fn setup_state_machine(machine: Entity, commands: &mut Commands) {
    let idle = commands
        .spawn_substate(machine, (Name::new("Idle"), StateComponent(ArcherIdle)))
        .id();
    let combat = commands
        .spawn_substate(machine, Name::new("Combat"))
        .id();

    commands.spawn_transition::<Idle2Combat>(idle, combat);
    commands.spawn_transition_always(combat, idle);
    commands.entity(machine).init_state_machine(idle);
}
```

进入 Idle 时 `ArcherIdle` 被插入到 `machine`（弓箭手本体）上，离开时移除。

## 查询示例

```rust
/// 查找处于 Idle 状态的弓箭手。
fn idle_behavior_system(
    idle_archers: Query<(Entity, &EnemyTarget), (With<Archer>, With<ArcherIdle>)>,
) {
    for (entity, target) in &idle_archers {
        // entity 是弓箭手，target 是当前锁定的目标
    }
}
```

## 与现有模块的关系

| 模块 | 关系 |
|---|---|
| [`ArcherRoleBuilder`](ArcherRoleBuilder.md) | 在 `setup_state_machine` 中将 `StateComponent(ArcherIdle)` 附加到 Idle 子状态 |
| [`ArcherPlugin`](ArcherPlugin.md) | 负责注册反射和 gearbox 状态组件 |
| [`Idle2Combat`](Idle2Combat.md) | 触发 Idle → Combat 转换的消息，转换后 `ArcherIdle` 自动移除 |

## 组件生命周期

```
状态机进入 Idle
    │
    ▼
ArcherIdle 插入到弓箭手（根）实体  ◄── 可通过 (With<Archer>, With<ArcherIdle>) 查询
    │
    │  发送 Idle2Combat
    ▼
ArcherIdle 从弓箭手实体移除
    │
    ▼
状态机进入 Combat
```

## 设计说明

- `ArcherIdle` 是一个纯粹的空标记组件，不含运行时数据。
- 生命周期由 gearbox 的 `StateComponent` 自动管理，无需手动操作。
- 与 `Archer` 配合使用：`Archer` 表示角色类型，`ArcherIdle` 表示当前状态。
