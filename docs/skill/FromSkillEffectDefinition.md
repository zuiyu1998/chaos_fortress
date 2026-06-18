# FromSkillEffectDefinition

`FromSkillEffectDefinition` 是一个 trait，用于从 [`SkillEffectDefinition`] 的参数字典生成自身实例，将键值对映射为结构体字段。

## 用途

- 实现该 trait 的类型可以从 [`SkillEffectDefinition`] 的 `params: HashMap<String, f32>` 字典中提取数值并构建自身。
- 常用于将技能效果参数定义（如子弹速度、伤害倍率、治疗量）转换为对应的运行时结构体或组件。
- 配合 [`SkillDefinition::get_effect`] 使用，按效果 `id` 获取 [`SkillEffectDefinition`]，再转换为具体的执行参数类型，供效果系统消费。

## 定义

```rust
/// 从 SkillEffectDefinition 的参数字典生成自身实例。
pub trait FromSkillEffectDefinition: Sized {
    /// 从给定的 SkillEffectDefinition 构建自身。
    ///
    /// 实现时通过 `definition.get("key")` 提取各字段对应的数值，
    /// 对缺失键使用合理的默认值或返回 `None`。
    fn from_effect(definition: &SkillEffectDefinition) -> Option<Self>;
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `from_effect(definition) -> Option<Self>` | 从 [`SkillEffectDefinition`] 的 `params` 字典中提取数值并构建实例。当必需字段缺失时返回 `None`。 |

## 实现示例

### 发射子弹效果

```rust
struct FireBulletEffect {
    speed: f32,
    damage: f32,
    count: u32,
}

impl FromSkillEffectDefinition for FireBulletEffect {
    fn from_effect(def: &SkillEffectDefinition) -> Option<Self> {
        Some(Self {
            speed: def.get("speed").unwrap_or(200.0),
            damage: def.get("damage").unwrap_or(10.0),
            count: def.get("count").unwrap_or(1.0) as u32,
        })
    }
}

// 使用方式
if let Some(effect) = skill_def.get_effect("fire_bullet") {
    if let Some(params) = FireBulletEffect::from_effect(effect) {
        // 使用 params.speed, params.damage, params.count
    }
}
```

### 治疗效果

```rust
struct HealEffect {
    amount: f32,
    ratio: f32,
}

impl FromSkillEffectDefinition for HealEffect {
    fn from_effect(def: &SkillEffectDefinition) -> Option<Self> {
        Some(Self {
            amount: def.get("amount").unwrap_or(0.0),
            ratio: def.get("ratio").unwrap_or(1.0),
        })
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

impl FromSkillEffectDefinition for AoeDamageEffect {
    fn from_effect(def: &SkillEffectDefinition) -> Option<Self> {
        Some(Self {
            radius: def.get("radius").unwrap_or(0.0),
            damage: def.get("damage").unwrap_or(0.0),
            falloff: def.get("falloff").unwrap_or(0.0),
        })
    }
}
```

## 与现有模块的关系

- **[`SkillEffectDefinition`]**：`FromSkillEffectDefinition` 从 `SkillEffectDefinition` 的 `params` 字典中读取数值，是该 trait 的唯一输入来源。
- **[`SkillDefinition`]**：通过 [`SkillDefinition::get_effect`] 获取指定 `id` 的 [`SkillEffectDefinition`]，再调用 `from_effect` 转换为具体运行时参数。
- **[`SkillInstance`]**：运行时可通过 `SkillInstance.skill` 句柄在 `Assets<SkillDefinition>` 中查找 [`SkillDefinition`]，然后遍历其 `effects` 列表，将每个效果定义转换为对应的执行参数结构体。
- **[`SkillEvent`]**：效果系统监听 [`SkillEvent`] 消息，在收到技能完成事件后通过 `FromSkillEffectDefinition` 解析效果参数并执行具体逻辑（如 [`fire_bullet_on_skill`]）。
- **[`FromSkillFeatureDefinition`]**：对应特征维度的转换 trait，与本 trait 结构相同、职责互补：`FromSkillFeatureDefinition` 处理数值特征，`FromSkillEffectDefinition` 处理效果参数。

### 典型使用流程

```
1. SkillDefinition 资产加载到 Assets<SkillDefinition> 中。
2. 运行时通过 skill_id 获取 SkillDefinition。
3. 遍历 SkillDefinition.effects，按 effect.id 区分类别。
4. 对每个 effect，调用对应的 FromSkillEffectDefinition 实现。
5. 将转换后的参数结构体传入效果系统执行（如生成子弹、治疗、施加 Buff）。
```

[`SkillEffectDefinition`]: ./SkillEffectDefinition.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillDefinition::get_effect`]: ./SkillDefinition.md#方法
[`SkillInstance`]: ./SkillInstance.md
[`SkillEvent`]: ./SkillEvent.md
[`fire_bullet_on_skill`]: ../battle/BattlePlugin.md#fire_bullet_on_skill
[`FromSkillFeatureDefinition`]: ./FromSkillFeatureDefinition.md
