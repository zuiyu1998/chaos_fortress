# SkillEffectDefinition

`SkillEffectDefinition` 是一个普通结构体，用于定义技能效果的**效果模板**，包含效果的唯一标识和一组可配置的参数字典。它内嵌于 [`SkillDefinition`] 中，不作为独立组件或资产存在。

## 用途

- `SkillEffectDefinition` 作为 [`SkillDefinition`] 中 `effects: Vec<SkillEffectDefinition>` 列表的元素存在，每个元素代表一个具体的效果（如发射投射物、治疗、范围伤害）。
- 通过唯一 `id` 标识效果类别，通过字典存储具体的数值参数，供技能运行时系统读取并驱动效果执行。
- 不单独实现 `Component` 或 `Asset` trait，始终作为 [`SkillDefinition`] 的一部分使用。

## 定义

```rust
use std::collections::HashMap;

/// 技能效果定义。
#[derive(Debug, Clone, PartialEq)]
pub struct SkillEffectDefinition {
    /// 效果唯一标识（如 "fire_bullet", "heal", "aoe_damage"）。
    pub id: String,
    /// 数值参数字典，键为参数名，值为浮点数值。
    pub params: HashMap<String, f32>,
}

impl SkillEffectDefinition {
    /// 创建一个新的 SkillEffectDefinition。
    pub fn new(id: impl Into<String>) -> Self { ... }
    /// 获取指定参数的值，不存在时返回 None。
    pub fn get(&self, key: &str) -> Option<f32> { ... }
    /// 设置指定参数的值。
    pub fn set(&mut self, key: impl Into<String>, value: f32) { ... }
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `String` | 效果唯一标识，用于区分不同类别的效果（如 `"fire_bullet"`、`"heal"`、`"aoe_damage"`、`"buff"`）。 |
| `params` | `HashMap<String, f32>` | 数值参数字典，键为参数名称（如 `"speed"`、`"damage"`、`"radius"`），值为对应的浮点数。 |

## 方法

| 方法 | 说明 |
|------|------|
| `new(id)` | 创建一个指定 `id` 的 `SkillEffectDefinition`，`params` 初始为空字典。 |
| `get(key) -> Option<f32>` | 根据参数名查询对应的数值，不存在时返回 `None`。 |
| `set(key, value)` | 设置或更新指定参数名的数值。 |

## 与 `SkillFeatureDefinition` 的关系

- **[`SkillFeatureDefinition`]** 描述特征的**数值维度**（如伤害值、范围半径、持续时间），`SkillEffectDefinition` 描述效果的**类型和参数**（如发射子弹、治疗、施加 Buff）。
- 一个 [`SkillDefinition`] 通常包含多个 `SkillFeatureDefinition` 来定义各个特征的数值，同时也包含多个 `SkillEffectDefinition` 来定义技能触发的具体效果。
- `SkillFeatureDefinition` 的 `features` 字段与 `SkillEffectDefinition` 的 `params` 字段结构相同（均为 `HashMap<String, f32>`），语义区别在于前者存放特征的数值参数，后者存放效果的运行参数。

## 与现有模块的关系

- **[`SkillDefinition`]**：`SkillEffectDefinition` 作为 [`SkillDefinition`] 中 `effects` 列表的一部分存在，为技能定义具体的效果行为。
- **[`SkillInstance`]**：运行时通过 [`SkillInstance.skill`] 句柄在 `Assets<SkillDefinition>` 中查找对应的 [`SkillDefinition`]，再通过 `effects` 列表获取 `SkillEffectDefinition` 并驱动对应的效果系统。
- **[`SkillEvent`]**：技能执行完成后通过 [`SkillEvent`] 广播结果，效果系统监听该消息并读取 `SkillEffectDefinition` 中的参数来执行具体效果（如 [`fire_bullet_on_skill`] 读取子弹速度参数）。
- **[`BattlePlugin`]**：`BattlePlugin` 中的 [`fire_bullet_on_skill`] 系统是效果定义的消费者之一，它根据效果定义中的参数生成子弹。

[`SkillDefinition`]: ./SkillDefinition.md
[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`SkillInstance`]: ./SkillInstance.md
[`SkillEvent`]: ./SkillEvent.md
[`fire_bullet_on_skill`]: ../battle/BattlePlugin.md#fire_bullet_on_skill
[`BattlePlugin`]: ../battle/BattlePlugin.md
