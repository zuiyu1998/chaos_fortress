# IntoSkillEffectDefinition

`IntoSkillEffectDefinition` 是一个 trait，用于将自身转换为一个 [`SkillEffectDefinition`] 实例，将结构体字段映射为数值键值对。

## 用途

- 实现该 trait 的类型可以将自身的字段打包为 [`SkillEffectDefinition`] 的 `params: HashMap<String, f32>` 字典。
- 常用于将运行时结构体或组件的数值参数（如子弹速度、伤害倍率）编码回技能效果定义。
- 配合 [`SkillDefinition::get_effect`] 使用，或用于程序化构建 [`SkillDefinition`] 资产时生成效果定义列表。
- 与 [`FromSkillEffectDefinition`] 互为逆操作。

## 定义

```rust
/// 将自身转换为 SkillEffectDefinition。
pub trait IntoSkillEffectDefinition {
    /// 将自身转换为一个 SkillEffectDefinition 实例。
    ///
    /// 实现时需要指定效果 `id`，并将各字段编码到 `params` 字典中。
    fn into_effect(self, id: impl Into<String>) -> SkillEffectDefinition;
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `into_effect(self, id) -> SkillEffectDefinition` | 将自身转换为一个 [`SkillEffectDefinition`]，`id` 为效果标识，各字段写入 `params` 字典。 |

## 实现示例

### 发射子弹效果

```rust
struct FireBulletEffect {
    speed: f32,
    damage: f32,
    count: u32,
}

impl IntoSkillEffectDefinition for FireBulletEffect {
    fn into_effect(self, id: impl Into<String>) -> SkillEffectDefinition {
        let mut def = SkillEffectDefinition::new(id);
        def.set("speed", self.speed);
        def.set("damage", self.damage);
        def.set("count", self.count as f32);
        def
    }
}

// 使用方式
let effect = FireBulletEffect { speed: 200.0, damage: 15.0, count: 3 };
let effect_def = effect.into_effect("fire_bullet");
// effect_def.id == "fire_bullet"
// effect_def.params == { "speed": 200.0, "damage": 15.0, "count": 3.0 }
```

### 治疗效果

```rust
struct HealEffect {
    amount: f32,
    ratio: f32,
}

impl IntoSkillEffectDefinition for HealEffect {
    fn into_effect(self, id: impl Into<String>) -> SkillEffectDefinition {
        let mut def = SkillEffectDefinition::new(id);
        def.set("amount", self.amount);
        def.set("ratio", self.ratio);
        def
    }
}
```

### 范围伤害效果

```rust
struct AoeDamageEffect {
    radius: f32,
    damage: f32,
    falloff: f32,
}

impl IntoSkillEffectDefinition for AoeDamageEffect {
    fn into_effect(self, id: impl Into<String>) -> SkillEffectDefinition {
        let mut def = SkillEffectDefinition::new(id);
        def.set("radius", self.radius);
        def.set("damage", self.damage);
        def.set("falloff", self.falloff);
        def
    }
}
```

## 与现有模块的关系

- **[`SkillEffectDefinition`]**：`IntoSkillEffectDefinition` 的每个实现都会生成一个 [`SkillEffectDefinition`] 实例，是构建效果定义的主要方式。
- **[`SkillDefinition`]**：可为 [`SkillDefinition`] 实现 `IntoSkillEffectDefinition`，将其整个 `effects` 列表展开为多个 [`SkillEffectDefinition`] 实例，或用于构造完整的 [`SkillDefinition`] 资产。
- **[`FromSkillEffectDefinition`]**：与 `IntoSkillEffectDefinition` 互为逆操作。在设计上，对于实现了 `FromSkillEffectDefinition` 的类型 T，通常也应考虑实现 `IntoSkillEffectDefinition` 以支持双向转换。
- **资产构建**：在程序化构建 [`SkillDefinition`] 资产时，通过 `IntoSkillEffectDefinition` 将各个参数结构体转换为 `SkillEffectDefinition`，然后通过 `add_effect` 组装到 `SkillDefinition` 中。

### 典型使用流程

```
1. 定义参数结构体（如 FireBulletEffect、HealEffect）。
2. 为参数结构体实现 IntoSkillEffectDefinition。
3. 在构建 SkillDefinition 时，创建参数结构体实例。
4. 调用 into_effect(id) 将其转换为 SkillEffectDefinition。
5. 通过 SkillDefinition::add_effect 组装到技能模板中。
6. 最终 SkillDefinition 以 Asset 形式存入 Assets<SkillDefinition>。
```

[`SkillEffectDefinition`]: ./SkillEffectDefinition.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillDefinition::get_effect`]: ./SkillDefinition.md#方法
[`FromSkillEffectDefinition`]: ./FromSkillEffectDefinition.md
