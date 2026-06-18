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
        app.register_type::<ArcherIdle>();

        app.register_transition::<Idle2Combat>();

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
            detect_target_when_idle
                .run_if(in_state(Screen::Gameplay).and(in_state(Pause(false)))),
        );
    }
}
```

## 注册内容

| 注册项 | 类型 | 说明 |
|--------|------|------|
| `Archer` | Component | 标记实体为弓箭手的标记组件 |
| `ArcherIdle` | Component | 标记弓箭手处于 Idle 状态，由 gearbox 的 `StateComponent` 自动管理 |
| `AttackSpeed` | Component | 弓箭手的攻击间隔（秒） |
| `ProjectileDamage` | Component | 弓箭手投射物伤害值 |
| `Idle2Combat` | Message | Gearbox 消息，触发 Idle → Combat 状态转换 |

## 系统

### detect_target_when_idle

`detect_target_when_idle` 系统负责在弓箭手处于 **Idle（静止）** 状态时检测是否已锁定敌人目标。当 `EnemyTarget` 组件有值时，自动发送 `Idle2Combat` 消息，触发状态机从 Idle 切换到 Combat。

```rust
/// 当弓箭手处于 Idle 状态且检测到敌人目标时，发送 Idle2Combat 消息
/// 触发状态机切换到 Combat 状态。
///
/// 查询活跃的 Idle 子状态（`StateComponent<ArcherIdle>` + `Active`），
/// 通过 `Active.machine` 获取弓箭手实体，检查其 `EnemyTarget`。
pub fn detect_target_when_idle(
    idle_states: Query<&Active, (With<StateComponent<ArcherIdle>>, With<Active>)>,
    archers: Query<&EnemyTarget, With<Archer>>,
    mut writer: MessageWriter<Idle2Combat>,
) {
    for active in &idle_states {
        if let Ok(target) = archers.get(active.machine) {
            if target.0.is_some() {
                writer.write(Idle2Combat { machine: active.machine });
            }
        }
    }
}
```

#### 查询条件

| 查询项 | 作用 |
|--------|------|
| `With<StateComponent<ArcherIdle>>` | 筛选 Idle 子状态实体 |
| `With<Active>` | 确保该子状态当前处于激活状态 |
| `&Active` | 读取 `Active.machine` 获取弓箭手（状态机根）实体 |
| `With<Archer>` + `&EnemyTarget` | 在弓箭手实体上读取当前锁定的敌人目标 |

#### 工作流程

1. 弓箭手处于 `Idle` 状态 → Idle 子状态实体获得 `Active`
2. 攻击范围传感器通过碰撞事件检测到敌人，设置弓箭手实体上的 `EnemyTarget.0 = Some(enemy_entity)`
3. `detect_target_when_idle` 每帧查询 `(With<StateComponent<ArcherIdle>>, With<Active>)` 找到活跃的 Idle 子状态
4. 通过 `Active.machine` 获取弓箭手实体，检查其 `EnemyTarget`
5. 若 `EnemyTarget.0` 有值，发送 `Idle2Combat { machine: active.machine }`
6. Gearbox 执行 Idle → Combat 转换，Combat 子状态获得 `Active`

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

1. `ArcherPlugin` 注册 `Archer`、`ArcherIdle` 等组件，注册 gearbox 状态组件和消息，并在 `RoleBuilderContainer` 中注册 `ArcherRoleBuilder`
2. 角色系统通过 `RoleBuilderContainer` 调用 `ArcherRoleBuilder` 构造弓箭手实体，实体附带状态机（起始 Idle）和攻击范围传感器
3. 弓箭手处于 Idle 状态时，Idle 子状态获得 `Active`
4. 攻击范围传感器检测到敌人进入射程，通过碰撞事件更新弓箭手实体的 `EnemyTarget`
5. `detect_target_when_idle` 查询 `(With<StateComponent<ArcherIdle>>, With<Active>)` 找到 Idle 子状态，通过 `Active.machine` 获取弓箭手实体，`EnemyTarget` 有值后发送 `Idle2Combat`
6. Gearbox 执行 Idle → Combat 转换，Combat 子状态获得 `Active`
7. SkillPlugin 中的 `tick_cooldown_timer` 和 `emit_skill_event` 系统自动管理技能冷却与完成事件。
8. 技能完成后系统自动发出 [`SkillEvent`]，[`BattlePlugin`] 中的 [`fire_bullet_on_skill`] 读取该消息，朝锁定敌人生成子弹。

## 注意事项

- `ArcherPlugin` 使用 `pub(super)` 可见性，仅在 `crate` 内部可用
- `Archer` 为标记组件，不含运行时数据；实际属性由 `AttackSpeed`、`ProjectileDamage` 等组件承载
- 后续可根据需要扩展注册更多类型（如箭矢相关的组件、事件等）

[`SkillEvent`]: ../../skill/SkillEvent.md
[`BattlePlugin`]: ../../battle/BattlePlugin.md
[`fire_bullet_on_skill`]: ../../battle/BattlePlugin.md#fire_bullet_on_skill
