# FromSkillFeatureDefinition

`FromSkillFeatureDefinition` 是一个 trait，用于从 [`SkillFeatureDefinition`] 的数值字典生成自身实例，将键值对映射为结构体字段。

## 用途

- 实现该 trait 的类型可以从 [`SkillFeatureDefinition`] 的 `features: HashMap<String, f32>` 字典中提取数值并构建自身。
- 常用于将技能数值参数定义（如伤害、范围、冷却）转换为对应的运行时结构体或组件。
- 配合 [`SkillDefinition::get_feature`] 使用，按特征 `id` 获取 [`SkillFeatureDefinition`]，再转换为具体的效果包装类型。

## 定义

```rust
/// 从 SkillFeatureDefinition 的数值字典生成自身实例。
pub trait FromSkillFeatureDefinition: Sized {
    /// 从给定的 SkillFeatureDefinition 构建自身。
    ///
    /// 实现时通过 `definition.get("key")` 提取各字段对应的数值，
    /// 对缺失键使用合理的默认值或返回 `None`。
    fn from_feature(definition: &SkillFeatureDefinition) -> Option<Self>;
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `from_feature(definition) -> Option<Self>` | 从 [`SkillFeatureDefinition`] 的 `features` 字典中提取数值并构建实例。当必需字段缺失时返回 `None`。 |

## 实现示例

### 伤害特征

```rust
struct DamageFeature {
    base_damage: f32,
    ratio: f32,
    crit_bonus: f32,
}

impl FromSkillFeatureDefinition for DamageFeature {
    fn from_feature(def: &SkillFeatureDefinition) -> Option<Self> {
        Some(Self {
            base_damage: def.get("base_damage").unwrap_or(0.0),
            ratio: def.get("ratio").unwrap_or(1.0),
            crit_bonus: def.get("crit_bonus").unwrap_or(0.0),
        })
    }
}

// 使用方式
if let Some(feature) = skill_def.get_feature("damage") {
    if let Some(damage) = DamageFeature::from_feature(feature) {
        // 使用 damage.base_damage, damage.ratio, damage.crit_bonus
    }
}
```

### 范围特征

```rust
struct RangeFeature {
    radius: f32,
    min_range: f32,
    max_range: f32,
}

impl FromSkillFeatureDefinition for RangeFeature {
    fn from_feature(def: &SkillFeatureDefinition) -> Option<Self> {
        Some(Self {
            radius: def.get("radius").unwrap_or(0.0),
            min_range: def.get("min_range").unwrap_or(0.0),
            max_range: def.get("max_range").unwrap_or(0.0),
        })
    }
}
```

### 冷却特征

```rust
struct CooldownFeature {
    cooldown: f32,
    initial_cooldown: f32,
}

impl FromSkillFeatureDefinition for CooldownFeature {
    fn from_feature(def: &SkillFeatureDefinition) -> Option<Self> {
        Some(Self {
            cooldown: def.get("cooldown").unwrap_or(1.0),
            initial_cooldown: def.get("initial_cooldown").unwrap_or(0.0),
        })
    }
}
```

## 与现有模块的关系

- **[`SkillFeatureDefinition`]**：`FromSkillFeatureDefinition` 从 `SkillFeatureDefinition` 的 `features` 字典中读取数值，是该 trait 的唯一输入来源。
- **[`SkillDefinition`]**：通过 [`SkillDefinition::get_feature`] 获取指定 `id` 的 [`SkillFeatureDefinition`]，再调用 `from_feature` 转换为具体类型。
- **[`SkillInstance`]**：运行时可通过 `SkillInstance.skill_id` 在 `Assets<SkillDefinition>` 中查找 [`SkillDefinition`]，然后遍历其 `features` 列表，将每个特征转换为对应的运行时组件。
- **`run_skill` 系统**：在施放技能时，通过 `FromSkillFeatureDefinition` 将技能数值特征解析为具体参数结构体，参与伤害计算和效果判定。

### 典型使用流程

```
1. SkillDefinition 资产加载到 Assets<SkillDefinition> 中。
2. 运行时通过 skill_id 获取 SkillDefinition。
3. 遍历 SkillDefinition.features，按 feature.id 区分类别。
4. 对每个 feature，调用对应的 FromSkillFeatureDefinition 实现。
5. 将转换后的结构体用于战斗计算（伤害、范围、冷却等）。
```

[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillDefinition::get_feature`]: ./SkillDefinition.md#方法
[`SkillInstance`]: ./SkillInstance.md
