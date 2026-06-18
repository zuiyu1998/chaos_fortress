# SkillActive

`SkillActive` 是一个标记组件（Marker Component），用于表示当前含有 [`SkillInstance`] 组件的技能实体**正处于激活/执行状态**。

## 用途

- 当技能实体上附加了 `SkillActive` 组件时，代表该技能正在运行中（例如效果正在执行、引导进行中）。
- 其他系统可通过查询 `SkillActive` 来判断技能是否处于激活状态，从而协同行为（例如禁止重复施放、显示激活指示器）。
- 与 [`CoolingTimer`] 等组件配合使用：`SkillActive` 标记激活状态，`CoolingTimer` 管理冷却状态。

## 定义

```rust
/// 标记组件，表示技能实体当前处于激活/执行状态。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct SkillActive;
```

`SkillActive` 是一个单元结构体（Unit Struct），不包含任何字段，仅作为标记使用。

## 组件说明

| 属性 | 说明 |
|------|------|
| 类型 | 标记组件（Marker Component） |
| 字段 | 无 |
| Trait | `Component`, `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Reflect` |

## 使用示例

```rust
/// 施放技能时附加 SkillActive 标记。
fn activate_skill(
    mut commands: Commands,
    skill_query: Query<Entity, Added<SkillInstance>>,
) {
    for entity in &skill_query {
        commands.entity(entity).insert(SkillActive);
    }
}

/// 技能完成时移除 SkillActive 标记。
fn deactivate_skill(
    mut commands: Commands,
    skill_query: Query<Entity, (With<SkillActive>, With<SkillEvent>)>,
) {
    for entity in &skill_query.iter() {
        commands.entity(entity).remove::<SkillActive>();
    }
}

/// 查询处于激活状态的技能。
fn check_active_skills(query: Query<&SkillInstance, With<SkillActive>>) {
    for instance in &query {
        info!("Skill is active: {:?}", instance.skill);
    }
}
```

## 注册

`SkillActive` 需在 [`SkillPlugin`] 中通过 `app.register_type::<SkillActive>()` 注册，才能被 Bevy 的反射系统识别。

## 与现有模块的关系

- **[`SkillInstance`]**：`SkillActive` 与 `SkillInstance` 附加在同一实体上。`SkillInstance` 标识实体拥有某个技能，`SkillActive` 标识该技能当前处于激活状态。
- **[`SkillEvent`]**：技能执行完成后会发出 `SkillEvent` 消息，此时可移除 `SkillActive` 标记。
- **[`SkillPlugin`]**：`SkillActive` 在 `SkillPlugin` 中通过 `app.register_type::<SkillActive>()` 注册。
- **[`SkillTarget`]**：持有者实体可通过 `SkillTarget` 组件指向技能实体，再查询其上的 `SkillActive` 组件来判断技能是否激活。
- **[`CoolingTimer`]**：技能激活期间可与冷却计时器配合，激活结束后进入冷却状态。

### 典型使用流程

```
1. 技能被施放时，在技能实体上插入 SkillActive 组件。
2. 技能执行过程中，其他系统通过查询 SkillActive 感知技能状态。
3. 技能执行完毕（如所有效果完成或引导中断），移除 SkillActive 组件。
4. 可选：技能进入冷却状态，由 CoolingTimer 管理。
```

[`SkillInstance`]: ./SkillInstance.md
[`SkillEvent`]: ./SkillEvent.md
[`SkillPlugin`]: ./SkillPlugin.md
[`SkillTarget`]: ./SkillTarget.md
[`CoolingTimer`]: ./cooldown/CoolingTimer.md
