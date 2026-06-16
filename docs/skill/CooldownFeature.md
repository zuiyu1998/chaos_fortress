# CooldownFeature

`CooldownFeature` 是一个结构体，用于表示技能的**冷却特征**数据，包含冷却时长。它实现了 [`FromSkillFeatureDefinition`] 和 [`IntoSkillFeatureDefinition`] trait。

## 用途

- 从 [`SkillFeatureDefinition`] 的 `features` 字典中提取冷却时长参数。
- 将冷却时长编码回 [`SkillFeatureDefinition`]，用于程序化构建 [`SkillDefinition`] 资产。
- 方便地在技能特征定义和运行时结构体之间进行双向转换。

## 定义

```rust
/// 冷却特征数据。
#[derive(Debug, Clone, PartialEq)]
pub struct CooldownFeature {
    /// 冷却时长（秒），默认值 1.0。
    pub cooldown_duration: f32,
}

impl CooldownFeature {
    /// 创建一个新的 CooldownFeature，使用默认冷却时长 1.0 秒。
    pub fn new() -> Self {
        Self {
            cooldown_duration: 1.0,
        }
    }
}

impl Default for CooldownFeature {
    fn default() -> Self {
        Self::new()
    }
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `cooldown_duration` | `f32` | 冷却时长（秒），默认值为 `1.0`。 |

## Trait 实现

### FromSkillFeatureDefinition

从 [`SkillFeatureDefinition`] 的数值字典中读取 `"cooldown_duration"` 键，缺失时使用默认值 `1.0`。

```rust
impl FromSkillFeatureDefinition for CooldownFeature {
    fn from_feature(def: &SkillFeatureDefinition) -> Option<Self> {
        Some(Self {
            cooldown_duration: def.get("cooldown_duration").unwrap_or(1.0),
        })
    }
}

// 使用方式
if let Some(feature) = skill_def.get_feature("cooldown") {
    if let Some(cooldown) = CooldownFeature::from_feature(feature) {
        // 使用 cooldown.cooldown_duration
    }
}
```

### IntoSkillFeatureDefinition

将 `CooldownFeature` 编码为 [`SkillFeatureDefinition`]，`features` 字典中包含 `"cooldown_duration"` 键。

```rust
impl IntoSkillFeatureDefinition for CooldownFeature {
    fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition {
        let mut def = SkillFeatureDefinition::new(id);
        def.set("cooldown_duration", self.cooldown_duration);
        def
    }
}

// 使用方式
let cooldown = CooldownFeature { cooldown_duration: 2.5 };
let feature = cooldown.into_feature("cooldown");
// feature.features == { "cooldown_duration": 2.5 }
```

## 与现有模块的关系

- **[`SkillFeatureDefinition`]**：`CooldownFeature` 通过 `FromSkillFeatureDefinition` 从 [`SkillFeatureDefinition`] 构建，通过 `IntoSkillFeatureDefinition` 转换回 [`SkillFeatureDefinition`]。
- **[`SkillDefinition`]**：通过 [`SkillDefinition::get_feature`] 获取 `"cooldown"` 特征，然后转换为 `CooldownFeature`；或通过 `into_feature` 将冷却数据添加到 [`SkillDefinition`] 中。
- **[`SkillInstance`]**：`CooldownFeature.cooldown_duration` 可作为创建 [`SkillInstance`] 时的冷却参数输入。
- **[`FromSkillFeatureDefinition`]**：`CooldownFeature` 实现了该 trait，提供从特征字典到结构体的转换能力。
- **[`IntoSkillFeatureDefinition`]**：`CooldownFeature` 实现了该 trait，提供从结构体到特征字典的转换能力。

[`FromSkillFeatureDefinition`]: ./FromSkillFeatureDefinition.md
[`IntoSkillFeatureDefinition`]: ./IntoSkillFeatureDefinition.md
[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillDefinition::get_feature`]: ./SkillDefinition.md#方法
[`SkillInstance`]: ./SkillInstance.md
