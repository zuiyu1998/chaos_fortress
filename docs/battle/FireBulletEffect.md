# FireBulletEffect

`FireBulletEffect` 是一个结构体，封装了"发射子弹"技能效果的运行时参数。它实现了 [`FromSkillEffectDefinition`] 和 [`IntoSkillEffectDefinition`]，支持从 [`SkillEffectDefinition`] 的参数字典构建自身，以及将自身编码回参数字典。

## 用途

- 作为 [`SkillEffectDefinition`] 中 `id = "fire_bullet"` 效果对应的运行时参数对象。
- 实现 [`FromSkillEffectDefinition`] 以便从技能定义资产中读取子弹参数（速度、伤害、数量等）。
- 实现 [`IntoSkillEffectDefinition`] 以便将运行时的参数编码回效果定义，用于持久化或程序化构建。
- 配合 [`SkillEffectBuilder`] 在技能施放时读取参数并生成子弹实体。

## 定义

```rust
/// "发射子弹"技能效果的运行时参数。
#[derive(Debug, Clone, PartialEq)]
pub struct FireBulletEffect {
    /// 子弹飞行速度（像素/秒）。
    pub speed: f32,
    /// 每颗子弹的伤害值。
    pub damage: f32,
    /// 一次释放发射的子弹数量。
    pub count: u32,
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

impl IntoSkillEffectDefinition for FireBulletEffect {
    fn into_effect(self, id: impl Into<String>) -> SkillEffectDefinition {
        let mut def = SkillEffectDefinition::new(id);
        def.set("speed", self.speed);
        def.set("damage", self.damage);
        def.set("count", self.count as f32);
        def
    }
}
```

## 字段说明

| 字段 | 类型 | 说明 | 默认值 |
|------|------|------|--------|
| `speed` | `f32` | 子弹飞行速度，单位为像素/秒 | `200.0` |
| `damage` | `f32` | 每颗子弹对目标造成的伤害值 | `10.0` |
| `count` | `u32` | 一次释放发射的子弹数量，用于散射/多重箭 | `1` |

## 使用示例

### 从 SkillEffectDefinition 读取

```rust
fn read_fire_bullet_effect(skill_def: &SkillDefinition) {
    if let Some(effect) = skill_def.get_effect("fire_bullet") {
        if let Some(params) = FireBulletEffect::from_effect(effect) {
            info!("Bullet speed: {}, damage: {}, count: {}",
                params.speed, params.damage, params.count);
        }
    }
}
```

### 编码为 SkillEffectDefinition

```rust
let params = FireBulletEffect {
    speed: 300.0,
    damage: 25.0,
    count: 3,
};
let effect_def = params.into_effect("fire_bullet");
// effect_def.id == "fire_bullet"
// effect_def.params == { "speed": 300.0, "damage": 25.0, "count": 3.0 }
```

### 在 SkillEffectBuilder 中使用

`FireBulletBuilder` 是 [`SkillEffectBuilder`] 的正式实现，参见 [`FireBulletBuilder`]。

## 与现有模块的关系

- **[`FromSkillEffectDefinition`]**：`FireBulletEffect` 是该 trait 的实现者，从 [`SkillEffectDefinition`] 的 `params` 字典读取子弹参数。
- **[`IntoSkillEffectDefinition`]**：`FireBulletEffect` 也是该 trait 的实现者，将字段编码回 [`SkillEffectDefinition`]。
- **[`SkillEffectDefinition`]**：效果 `id` 为 `"fire_bullet"` 时，对应的效果定义使用此结构体作为运行时参数。
- **[`SkillEffectBuilder`]**：[`SkillEffectBuilder`] 的实现者（如 [`FireBulletBuilder`]）使用 `FireBulletEffect` 解析参数并插入为组件。
- **[`BattlePlugin`]**：`BattlePlugin` 中的 [`fire_bullet_on_skill`] 系统是子弹生成的最终执行者，它接收 [`SkillEvent`] 并根据效果参数生成子弹。
- **[`Bullet`]**：生成的子弹实体携带 [`Bullet`] 标记组件，由子弹模块的碰撞系统自动处理命中逻辑。

[`FromSkillEffectDefinition`]: ../skill/FromSkillEffectDefinition.md
[`IntoSkillEffectDefinition`]: ../skill/IntoSkillEffectDefinition.md
[`SkillEffectDefinition`]: ../skill/SkillEffectDefinition.md
[`SkillEffectBuilder`]: ../skill/SkillEffectBuilder.md
[`BattlePlugin`]: ./BattlePlugin.md
[`fire_bullet_on_skill`]: ./BattlePlugin.md#fire_bullet_on_skill
[`SkillEvent`]: ../skill/SkillEvent.md
[`Bullet`]: ../bullet/Bullet.md
[`FireBulletBuilder`]: ./FireBulletBuilder.md
