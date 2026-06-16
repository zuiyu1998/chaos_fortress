# SkillFeatureBuilder

`SkillFeatureBuilder` 是一个 trait，用于根据 [`SkillFeatureDefinition`] 的数值参数构建实体或组件。它参考了 [`RoleBuilder`] 的设计模式。

## 用途

- 实现该 trait 的类型可以根据 [`SkillFeatureDefinition`] 的 `features` 字典中的数值参数，在 ECS 世界中生成对应的实体或组件。
- 常用于将技能数值特征（如伤害区域、投射物、召唤物、Buff 区域）在运行时实例化到场景中。
- 配合 [`SkillDefinition::add_feature`] 和 [`SkillDefinition::get_feature`] 使用，将特征定义转换为具体的游戏对象。
- 与 [`FromSkillFeatureDefinition`] 配合：后者从特征构建纯数据对象，前者从特征构建实体。

## 定义

```rust
/// 上下文：传递给 SkillFeatureBuilder 的数据。
pub struct SkillFeatureBuilderContext {
    /// 要构建的 SkillFeatureDefinition 引用。
    pub feature: SkillFeatureDefinition,
    /// 施放该技能的实体。
    pub skill: Entity,
    /// 该技能从属的实体（技能持有者/施放者）。
    pub target: Entity,
}

/// 根据 SkillFeatureDefinition 构建实体或组件的 trait。
pub trait SkillFeatureBuilder: Send + Sync {
    /// 根据特征定义和上下文构建一个实体，返回其 Entity ID。
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError>;
}
```

## 相关类型

### SkillFeatureBuilderContext

| 字段 | 类型 | 说明 |
|------|------|------|
| `feature` | [`SkillFeatureDefinition`] | 要构建的技能特征定义，包含 `id` 和数值字典 `features`。 |
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
| `build(commands, ctx) -> Result<Entity, BuildError>` | 从 [`SkillFeatureDefinition`] 和上下文构建一个实体，返回生成的 `Entity`。失败时返回 `BuildError`。 |

## 实现示例

### 投射物特征构建器

```rust
struct ProjectileSpawner;

impl SkillFeatureBuilder for ProjectileSpawner {
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError> {
        let speed = ctx.feature.get("speed").unwrap_or(200.0);
        let damage = ctx.feature.get("damage").unwrap_or(10.0);

        Ok(commands
            .spawn((
                Name::new(format!("Projectile ({})", ctx.feature.id)),
                Sprite::default(),
                ProjectileDamage(damage),
                Velocity(Vec2::X * speed),
            ))
            .id())
    }
}
```

### AoE 区域构建器

```rust
struct AoEBuilder;

impl SkillFeatureBuilder for AoEBuilder {
    fn build<'w, 's>(
        &self,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError> {
        let radius = ctx.feature.get("radius").unwrap_or(3.0);
        let duration = ctx.feature.get("duration").unwrap_or(2.0);

        Ok(commands
            .spawn((
                Name::new(format!("AoE ({})", ctx.feature.id)),
                Visibility::default(),
                Collider::circle(radius),
                Sensor,
                Lifetime(Timer::from_seconds(duration, TimerMode::Once)),
            ))
            .id())
    }
}
```


## 与现有模块的关系

- **[`SkillFeatureDefinition`]**：`SkillFeatureBuilder` 的 `build` 方法接收包含 [`SkillFeatureDefinition`] 的上下文，从中读取数值参数来决定构建行为。
- **[`SkillDefinition`]**：遍历 `SkillDefinition.features`，根据每个特征的 `id` 选择对应的 `SkillFeatureBuilder` 来构建实体。
- **[`FromSkillFeatureDefinition`]**：`FromSkillFeatureDefinition` 从特征构建纯数据对象，`SkillFeatureBuilder` 从特征构建实体——两者互为补充。
- **[`SkillInstance`]**：施放技能时，通过 [`SkillInstance`] 获取 `skill_id`，再查找 `SkillDefinition`，然后遍历其 features 并调用对应的 builder 来生成效果实体。
- **`RunSkill` 系统**：在技能施放流程中，使用 [`SkillFeatureBuilderContainer`] 将特征定义实例化为游戏对象（投射物、AoE 区域、召唤物等）。

### 典型使用流程

```
1. 为每种特征 id 注册对应的 SkillFeatureBuilder（如 "projectile" → ProjectileSpawner）。
2. 技能施放时，通过 SkillInstance 获取 SkillDefinition。
3. 遍历 SkillDefinition.features。
4. 对每个 feature，在 [`SkillFeatureBuilderContainer`] 中查找对应的 builder。
5. 调用 builder.build(commands, ctx) 生成效果实体。
6. 效果实体在场景中执行其逻辑（飞行、爆炸、持续伤害等）。
```

[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillDefinition::add_feature`]: ./SkillDefinition.md#方法
[`SkillDefinition::get_feature`]: ./SkillDefinition.md#方法
[`FromSkillFeatureDefinition`]: ./FromSkillFeatureDefinition.md
[`SkillInstance`]: ./SkillInstance.md
[`SkillFeatureBuilderContainer`]: ./SkillFeatureBuilderContainer.md
[`RoleBuilder`]: ../role/RoleBuilder.md
[`RoleBuilderContainer`]: ../role/RoleBuilderContainer.md
