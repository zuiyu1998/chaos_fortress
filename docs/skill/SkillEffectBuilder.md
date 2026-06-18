# SkillEffectBuilder

`SkillEffectBuilder` 是一个 trait，用于根据 [`SkillEffectDefinition`] 的参数字典构建实体或组件。它参考了 [`SkillFeatureBuilder`] 的设计模式，区别在于它以 [`SkillEffectDefinition`] 而非 [`SkillFeatureDefinition`] 为输入参数。

## 用途

- 实现该 trait 的类型可以根据 [`SkillEffectDefinition`] 的 `params` 字典中的数值参数，在 ECS 世界中生成对应的实体或组件。
- 常用于将技能效果定义（如发射子弹、治疗、范围伤害、施加 Buff）在运行时实例化到场景中。
- 配合 [`SkillDefinition::add_effect`] 和 [`SkillDefinition::get_effect`] 使用，将效果定义转换为具体的游戏对象。
- 与 [`FromSkillEffectDefinition`] 配合：后者从效果构建纯数据对象，前者从效果构建实体。

## 定义

```rust
/// 上下文：传递给 SkillEffectBuilder 的数据。
pub struct SkillEffectBuilderContext {
    /// 要构建的 SkillEffectDefinition 引用。
    pub effect: SkillEffectDefinition,
    /// 施放该技能的实体。
    pub skill: Entity,
    /// 该技能从属的实体（技能持有者/施放者）。
    pub target: Entity,
}

/// 根据 SkillEffectDefinition 构建实体或组件的 trait。
pub trait SkillEffectBuilder: Send + Sync {
    /// 根据效果定义和上下文构建一个实体，返回其 Entity ID。
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError>;
}
```

## 相关类型

### SkillEffectBuilderContext

| 字段 | 类型 | 说明 |
|------|------|------|
| `effect` | [`SkillEffectDefinition`] | 要构建的技能效果定义，包含 `id` 和参数字典 `params`。 |
| `skill` | [`Entity`] | 施放该技能的实体。 |
| `target` | [`Entity`] | 该技能从属的实体（技能持有者/施放者）。 |

### BuildError

```rust
/// 构建实体时返回的错误。
#[derive(Debug)]
pub enum BuildError {
    /// 没有为指定名称注册 builder。
    MissingBuilder(String),
    /// Builder 执行过程中发生错误。
    BuildFailed(String),
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `build(commands, ctx) -> Result<Entity, BuildError>` | 从 [`SkillEffectDefinition`] 和上下文构建一个实体，返回生成的 `Entity`。失败时返回 `BuildError`。 |

## 实现示例

### 发射子弹效果构建器

```rust
struct FireBulletBuilder;

impl SkillEffectBuilder for FireBulletBuilder {
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError> {
        let speed = ctx.effect.get("speed").unwrap_or(200.0);
        let damage = ctx.effect.get("damage").unwrap_or(10.0);
        let count = ctx.effect.get("count").unwrap_or(1.0) as u32;

        // 生成 count 颗子弹，均匀分布在扇形范围内
        for i in 0..count {
            let angle = (i as f32 / count as f32 - 0.5) * std::f32::consts::FRAC_PI_6;
            let direction = Vec2::new(angle.cos(), angle.sin());
            commands.spawn((
                Name::new(format!("Bullet ({})", ctx.effect.id)),
                Bullet,
                ProjectileDamage(damage),
                LinearVelocity(direction * speed),
            ));
        }
        Ok(ctx.skill)
    }
}
```

### 治疗效果构建器

```rust
struct HealBuilder;

impl SkillEffectBuilder for HealBuilder {
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError> {
        let amount = ctx.effect.get("amount").unwrap_or(0.0);
        let ratio = ctx.effect.get("ratio").unwrap_or(1.0);

        // 直接修改目标实体的 BattleState
        commands.entity(ctx.target).insert(HealEvent {
            amount: amount * ratio,
        });
        Ok(ctx.target)
    }
}
```

### 范围伤害效果构建器

```rust
struct AoeDamageBuilder;

impl SkillEffectBuilder for AoeDamageBuilder {
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError> {
        let radius = ctx.effect.get("radius").unwrap_or(3.0);
        let damage = ctx.effect.get("damage").unwrap_or(0.0);
        let duration = ctx.effect.get("duration").unwrap_or(1.0);

        Ok(commands
            .spawn((
                Name::new(format!("AoeDamage ({})", ctx.effect.id)),
                AoeDamage { damage, falloff: ctx.effect.get("falloff").unwrap_or(0.0) },
                Collider::circle(radius),
                Sensor,
                Lifetime(Timer::from_seconds(duration, TimerMode::Once)),
            ))
            .id())
    }
}
```

## 与现有模块的关系

- **[`SkillEffectDefinition`]**：`SkillEffectBuilder` 的 `build` 方法接收包含 [`SkillEffectDefinition`] 的上下文，从中读取参数字典来决定构建行为。
- **[`SkillDefinition`]**：遍历 `SkillDefinition.effects`，根据每个效果的 `id` 选择对应的 `SkillEffectBuilder` 来构建效果实体。
- **[`FromSkillEffectDefinition`]**：`FromSkillEffectDefinition` 从效果构建纯数据对象，`SkillEffectBuilder` 从效果构建实体——两者互为补充。
- **[`SkillFeatureBuilder`]**：`SkillFeatureBuilder` 处理特征维度的实体构建，`SkillEffectBuilder` 处理效果维度的实体构建。两者结构相同，职责互补。
- **[`SkillInstance`]**：施放技能时，通过 [`SkillInstance`] 获取 `skill_id`，再查找 `SkillDefinition`，然后遍历其 effects 并调用对应的 builder 来生成效果实体。
- **[`SkillEvent`]**：效果系统可以监听 [`SkillEvent`] 消息，在技能完成后通过 [`SkillEffectBuilderContainer`] 将效果定义实例化为具体的游戏对象。

### 典型使用流程

```
1. 为每种效果 id 注册对应的 SkillEffectBuilder（如 "fire_bullet" → FireBulletBuilder）。
2. 技能施放时，通过 SkillInstance 获取 SkillDefinition。
3. 遍历 SkillDefinition.effects。
4. 对每个 effect，在 [`SkillEffectBuilderContainer`] 中查找对应的 builder。
5. 调用 builder.build(commands, ctx) 生成效果实体。
6. 效果实体在场景中执行其逻辑（飞行、治疗、爆炸等）。
```

[`SkillEffectDefinition`]: ./SkillEffectDefinition.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillDefinition::add_effect`]: ./SkillDefinition.md#方法
[`SkillDefinition::get_effect`]: ./SkillDefinition.md#方法
[`FromSkillEffectDefinition`]: ./FromSkillEffectDefinition.md
[`SkillFeatureBuilder`]: ./SkillFeatureBuilder.md
[`SkillInstance`]: ./SkillInstance.md
[`SkillEvent`]: ./SkillEvent.md
[`SkillEffectBuilderContainer`]: ./SkillEffectBuilderContainer.md
[`RoleBuilder`]: ../role/RoleBuilder.md
[`BuildError`]: ./SkillFeatureBuilder.md#BuildError
