# SkillDefinition

`SkillDefinition` 是一个资源（Asset），用于定义技能的**完整模板**，包含技能标识、名称、数值特征定义列表以及效果定义列表。可以从外部文件加载，也可以程序化构建。

## 用途

- 持有 `SkillDefinition` 资源的系统代表拥有一个**技能模板库**，定义了技能的标识、显示名称、所有数值参数和效果行为。
- 提供技能的基础数据定义，供 [`SkillInstance`] 在运行时通过 `Handle<SkillDefinition>` 引用。
- 通过内嵌多个 [`SkillFeatureDefinition`] 实现技能数值参数的结构化组织（如伤害、范围、冷却等各维度独立定义）。
- 通过内嵌多个 [`SkillEffectDefinition`] 实现技能运行时效果的定义（如发射子弹、治疗、范围伤害）。
- 可作为 [`Asset`](bevy::asset::Asset) 从外部文件（如 JSON/CSV）加载，也可在代码中程序化创建。

## 定义

```rust
use std::collections::HashMap;

/// 技能模板定义。
#[derive(Asset, Debug, Clone, TypePath)]
pub struct SkillDefinition {
    /// 技能唯一标识（如 "archer_shot"、"enemy_charge"）。
    pub id: String,
    /// 技能名称（用于 UI 显示）。
    pub name: String,
    /// 技能数值特征定义列表。
    pub features: Vec<SkillFeatureDefinition>,
    /// 技能效果定义列表。
    pub effects: Vec<SkillEffectDefinition>,
}

impl SkillDefinition {
    /// 创建一个新的 SkillDefinition。
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self { ... }
    /// 按特征 id 查找对应的 SkillFeatureDefinition。
    pub fn get_feature(&self, feature_id: &str) -> Option<&SkillFeatureDefinition> { ... }
    /// 添加一个 SkillFeatureDefinition。
    pub fn add_feature(&mut self, feature: SkillFeatureDefinition) { ... }
    /// 按效果 id 查找对应的 SkillEffectDefinition。
    pub fn get_effect(&self, effect_id: &str) -> Option<&SkillEffectDefinition> { ... }
    /// 添加一个 SkillEffectDefinition。
    pub fn add_effect(&mut self, effect: SkillEffectDefinition) { ... }
}
```

> **注意**：[`SkillFeatureDefinition`] 和 [`SkillEffectDefinition`] 在此处是普通结构体（不实现 `Component` 或 `Asset`），作为 `SkillDefinition` 的内嵌数据存在。

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `String` | 技能唯一标识，用于被 [`SkillInstance`] 引用（通过 `Handle<SkillDefinition>` 匹配）。 |
| `name` | `String` | 技能显示名称，用于 UI 和日志。 |
| `features` | `Vec<SkillFeatureDefinition>` | 技能包含的数值特征定义列表，每个元素代表一个特征维度（如伤害、范围、持续时间）。 |
| `effects` | `Vec<SkillEffectDefinition>` | 技能包含的效果定义列表，每个元素代表一个运行时效果（如发射子弹、治疗、范围伤害）。 |

## 方法

| 方法 | 说明 |
|------|------|
| `new(id, name)` | 创建一个指定 `id` 和 `name` 的 `SkillDefinition`，`features` 和 `effects` 初始为空列表。 |
| `get_feature(feature_id) -> Option<&SkillFeatureDefinition>` | 按 `feature_id` 在 `features` 列表中查找对应的 [`SkillFeatureDefinition`]，不存在时返回 `None`。 |
| `add_feature(feature)` | 向 `features` 列表中添加一个 [`SkillFeatureDefinition`]。 |
| `get_effect(effect_id) -> Option<&SkillEffectDefinition>` | 按 `effect_id` 在 `effects` 列表中查找对应的 [`SkillEffectDefinition`]，不存在时返回 `None`。 |
| `add_effect(effect)` | 向 `effects` 列表中添加一个 [`SkillEffectDefinition`]。 |

## Asset 加载

`SkillDefinition` 实现了 [`Asset`](bevy::asset::Asset) trait，可通过 Bevy 的资产系统从外部文件加载。需要配合一个实现了 [`AssetLoader`] 的加载器（如 JSON/CSV 加载器）。

### 注册方式

```rust
app.init_asset::<SkillDefinition>();
// 如果有自定义加载器：
// app.register_asset_loader(SkillDefinitionLoader);
```

### 使用方式

```rust
// 从文件加载
let skill_handle: Handle<SkillDefinition> = asset_server.load("skills/archer_shot.skill.json");

// 在系统中访问
fn system(skills: Res<Assets<SkillDefinition>>) {
    if let Some(skill) = skills.get(&handle) {
        // 使用 skill.id, skill.name, skill.features, skill.effects
    }
}
```

## 与现有模块的关系

- **[`SkillInstance`]**：[`SkillInstance.skill`] 持有 `Handle<SkillDefinition>`，运行时通过该句柄在 `Assets<SkillDefinition>` 资源中查找对应的 `SkillDefinition` 资产获取技能模板数据。
- **[`SkillFeatureDefinition`]**：`SkillDefinition` 通过 `features: Vec<SkillFeatureDefinition>` 包含多个数值特征定义，`get_feature` 方法提供按 `id` 查找的能力。`SkillFeatureDefinition` 是普通结构体（非 Component，非 Asset），作为 `SkillDefinition` 的一部分存在。
- **[`SkillEffectDefinition`]**：`SkillDefinition` 通过 `effects: Vec<SkillEffectDefinition>` 包含多个效果定义，`get_effect` 方法提供按 `id` 查找的能力。`SkillEffectDefinition` 同样是普通结构体，作为 `SkillDefinition` 的一部分存在。
- **`run_skill` 系统**：在施放技能时，通过 `SkillInstance.skill` 在 `Assets<SkillDefinition>` 中查询 `SkillDefinition`，再通过 `get_feature` 获取具体数值参与计算。
- **敌人模块**（`enemy`）：敌人生成时可通过 `Assets<SkillDefinition>` 资源查找技能模板，再创建对应的 [`SkillInstance`] 附加到实体上。
- **加载管线**：`SkillDefinition` 作为 `Asset` 可通过 `AssetServer` 从文件加载，由 `AssetLoader` 解析并存入 `Assets<SkillDefinition>` 资源中。

### 典型使用流程

```
1. 设计阶段定义 SkillDefinition 资产文件（JSON/CSV），包含 id, name, features, effects。
2. 游戏启动时通过 AssetServer 加载 SkillDefinition 资产。
3. 实体生成时，从 Assets<SkillDefinition> 获取技能模板，创建 SkillInstance 作为运行时状态。
4. 战斗中通过 SkillInstance.skill 句柄在 Assets<SkillDefinition> 中查找 SkillDefinition。
5. 通过 get_feature 获取数值参数，参与伤害计算和效果判定。
6. 通过 get_effect 获取效果定义，传递给 [`SkillEffectBuilder`] 在场景中生效果实体。
```

[`SkillInstance`]: ./SkillInstance.md
[`SkillInstance.skill`]: ./SkillInstance.md#字段说明
[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`SkillEffectDefinition`]: ./SkillEffectDefinition.md
[`SkillEffectBuilder`]: ./SkillEffectBuilder.md
