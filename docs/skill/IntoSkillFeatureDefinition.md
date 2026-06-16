# IntoSkillFeatureDefinition

`IntoSkillFeatureDefinition` 是一个 trait，用于将自身转换为一个 [`SkillFeatureDefinition`] 实例，将结构体字段映射为数值键值对。

## 用途

- 实现该 trait 的类型可以将自身的字段打包为 [`SkillFeatureDefinition`] 的 `features: HashMap<String, f32>` 字典。
- 常用于将运行时结构体或组件的数值参数（如伤害、范围、冷却）编码回技能特征定义。
- 配合 [`SkillDefinition::get_feature`] 使用，或用于程序化构建 [`SkillDefinition`] 资产时生成特征定义列表。
- 与 [`FromSkillFeatureDefinition`] 互为逆操作。

## 定义

```rust
/// 将自身转换为 SkillFeatureDefinition。
pub trait IntoSkillFeatureDefinition {
    /// 将自身转换为一个 SkillFeatureDefinition 实例。
    ///
    /// 实现时需要指定特征 `id`，并将各字段编码到 `features` 字典中。
    fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition;
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `into_feature(self, id) -> SkillFeatureDefinition` | 将自身转换为一个 [`SkillFeatureDefinition`]，`id` 为特征标识，各字段写入 `features` 字典。 |

## 实现示例

### 伤害特征

```rust
struct DamageFeature {
    base_damage: f32,
    ratio: f32,
    crit_bonus: f32,
}

impl IntoSkillFeatureDefinition for DamageFeature {
    fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition {
        let mut def = SkillFeatureDefinition::new(id);
        def.set("base_damage", self.base_damage);
        def.set("ratio", self.ratio);
        def.set("crit_bonus", self.crit_bonus);
        def
    }
}

// 使用方式
let damage = DamageFeature { base_damage: 50.0, ratio: 1.5, crit_bonus: 0.2 };
let feature = damage.into_feature("damage");
// feature.id == "damage"
// feature.features == { "base_damage": 50.0, "ratio": 1.5, "crit_bonus": 0.2 }
```

### 范围特征

```rust
struct RangeFeature {
    radius: f32,
    min_range: f32,
    max_range: f32,
}

impl IntoSkillFeatureDefinition for RangeFeature {
    fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition {
        let mut def = SkillFeatureDefinition::new(id);
        def.set("radius", self.radius);
        def.set("min_range", self.min_range);
        def.set("max_range", self.max_range);
        def
    }
}
```

### 冷却特征

```rust
struct CooldownFeature {
    cooldown: f32,
    initial_cooldown: f32,
}

impl IntoSkillFeatureDefinition for CooldownFeature {
    fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition {
        let mut def = SkillFeatureDefinition::new(id);
        def.set("cooldown", self.cooldown);
        def.set("initial_cooldown", self.initial_cooldown);
        def
    }
}
```

## 与现有模块的关系

- **[`SkillFeatureDefinition`]**：`IntoSkillFeatureDefinition` 的每个实现都会生成一个 [`SkillFeatureDefinition`] 实例，是构建特征定义的主要方式。
- **[`SkillDefinition`]**：可为 [`SkillDefinition`] 实现 `IntoSkillFeatureDefinition`，将其整个 `features` 列表展开为多个 [`SkillFeatureDefinition`] 实例，或用于构造完整的 [`SkillDefinition`] 资产。
- **[`FromSkillFeatureDefinition`]**：与 `IntoSkillFeatureDefinition` 互为逆操作。在设计上，对于实现了 `FromSkillFeatureDefinition` 的类型 T，通常也应考虑实现 `IntoSkillFeatureDefinition` 以支持双向转换。
- **资产构建**：在程序化构建 [`SkillDefinition`] 资产时，通过 `IntoSkillFeatureDefinition` 将各个参数结构体转换为 `SkillFeatureDefinition`，然后通过 `add_feature` 组装到 `SkillDefinition` 中。

### 典型使用流程

```
1. 定义参数结构体（如 DamageFeature、RangeFeature）。
2. 为参数结构体实现 IntoSkillFeatureDefinition。
3. 在构建 SkillDefinition 时，创建参数结构体实例。
4. 调用 into_feature(id) 将其转换为 SkillFeatureDefinition。
5. 通过 SkillDefinition::add_feature 组装到技能模板中。
6. 最终 SkillDefinition 以 Asset 形式存入 Assets<SkillDefinition>。
```

[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillDefinition::get_feature`]: ./SkillDefinition.md#方法
[`FromSkillFeatureDefinition`]: ./FromSkillFeatureDefinition.md
