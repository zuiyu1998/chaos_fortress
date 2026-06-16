# SkillFeatureDefinition

`SkillFeatureDefinition` 是一个普通结构体，用于定义技能效果的**数值特征**，包含一组可配置的键值对。它内嵌于 [`SkillDefinition`] 中，不作为独立组件或资产存在。

## 用途

- `SkillFeatureDefinition` 作为 [`SkillDefinition`] 中 `features: Vec<SkillFeatureDefinition>` 列表的元素存在，每个元素代表一类特征维度（如伤害、范围、持续时间）。
- 通过唯一 `id` 标识特征类别，通过字典存储具体的数值参数，供技能系统在运行时读取并参与计算。
- 不单独实现 `Component` 或 `Asset` trait，始终作为 [`SkillDefinition`] 的一部分使用。

## 定义

```rust
use std::collections::HashMap;

/// 技能效果数值特征定义。
#[derive(Debug, Clone, PartialEq)]
pub struct SkillFeatureDefinition {
    /// 唯一标识（如 "damage", "range", "duration"）。
    pub id: String,
    /// 数值参数字典，键为参数名，值为浮点数值。
    pub features: HashMap<String, f32>,
}

impl SkillFeatureDefinition {
    /// 创建一个新的 SkillFeatureDefinition。
    pub fn new(id: impl Into<String>) -> Self { ... }
    /// 获取指定参数的值，不存在时返回默认值。
    pub fn get(&self, key: &str) -> Option<f32> { ... }
    /// 设置指定参数的值。
    pub fn set(&mut self, key: impl Into<String>, value: f32) { ... }
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `String` | 特征定义唯一标识，用于区分不同类别的特征（如 `"damage"`、`"range"`、`"duration"`、`"aoe_radius"`）。 |
| `features` | `HashMap<String, f32>` | 数值参数字典，键为参数名称（如 `"base_damage"`、`"ratio"`），值为对应的浮点数。 |

## 方法

| 方法 | 说明 |
|------|------|
| `new(id)` | 创建一个指定 `id` 的 `SkillFeatureDefinition`，`features` 初始为空字典。 |
| `get(key) -> Option<f32>` | 根据参数名查询对应的数值，不存在时返回 `None`。 |
| `set(key, value)` | 设置或更新指定参数名的数值。 |

## 与现有模块的关系

- **[`SkillDefinition`]**：`SkillFeatureDefinition` 作为 [`SkillDefinition`] 中 `features` 列表的一部分存在，为技能效果提供可调节的数值参数。
- **[`SkillInstance`]**：运行时通过 [`SkillInstance.skill_id`] 在 `Assets<SkillDefinition>` 中查找对应的 [`SkillDefinition`]，再通过 `get_feature` 获取 `SkillFeatureDefinition` 中的数值参与计算。
- **`run_skill` 系统**：在施放技能时，从 [`SkillDefinition`] 中读取 `SkillFeatureDefinition` 的数值参数（如伤害倍率、范围半径），参与效果计算。

[`SkillDefinition`]: ./SkillDefinition.md
[`SkillInstance`]: ./SkillInstance.md
