# SkillInstance

`SkillInstance` 是一个组件（Component），用于将技能模板绑定到实体上，表示该实体**拥有**此技能。

## 用途

- 持有 `SkillInstance` 组件的实体代表该技能在该实体上**已就绪**，可被技能系统调度施放。
- 同一实体可持有多个 `SkillInstance` 组件（每个技能一个实例），通过不同的 `Handle<SkillDefinition>` 区分。
- 与 [`SkillDefinition`] 资源配合使用：`SkillDefinition` 定义技能的完整模板（标识、名称、数值特征），`SkillInstance` 仅持有对模板的引用。

## 定义

```rust
/// 技能运行时实例，持有对技能模板的引用。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct SkillInstance {
    /// 引用的技能模板句柄。
    pub skill: Handle<SkillDefinition>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `skill` | `Handle<SkillDefinition>` | 引用 [`SkillDefinition`] 资源的句柄，用于在施放时查找技能模板数据。 |

## 方法

`SkillInstance` 为纯数据组件，不提供方法。运行时状态（如充能、冷却等）由其他专用组件管理。

## 与现有模块的关系

- **[`SkillDefinition`]**：`skill` 字段持有 `Handle<SkillDefinition>`，技能系统通过该句柄在 `Assets<SkillDefinition>` 中查找技能模板数据（`id`、`name`、`features`）。
- **`run_skill` 系统**：弓箭手等角色的技能驱动系统应查询 `SkillInstance` 组件，通过 `skill` 句柄获取 [`SkillDefinition`] 模板，再执行技能效果逻辑。
- **敌人模块**（`enemy`）：敌人实体携带的 `skills: [Skill]` 在生成时可转化为多个 `SkillInstance` 组件附加到实体上，供 AI 系统调度。
- **[`BattleState`]**：施放技能时，通过 `SkillDefinition` 模板的 `execute` 逻辑计算伤害，再调用 `BattleState::take_damage` 对目标造成伤害。

### 典型使用流程

```
1. 实体生成时，为每个技能创建一个 SkillInstance 组件并附加。
2. run_skill 系统查询 SkillInstance 组件。
3. 通过 skill 句柄在 Assets<SkillDefinition> 中查找技能模板。
4. 根据 SkillDefinition 的 features 数据计算效果。
5. 效果应用于目标（伤害、治疗、Buff 等）。
```

[`SkillDefinition`]: ./SkillDefinition.md
[`BattleState`]: ../battle/BattleState.md
