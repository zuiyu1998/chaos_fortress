# CooldownFeatureBuilder

`CooldownFeatureBuilder` 是一个结构体，实现了 [`SkillFeatureBuilder`] trait，用于根据冷却特征定义为技能实体附加 [`CooldownFeature`] 和 [`CoolingTimer`] 组件，并将技能实体设置为持有者实体的子实体。

## 用途

- 从 [`SkillFeatureDefinition`] 的 `features` 字典中读取 `cooldown_duration` 参数。
- 在技能实体（`ctx.skill`）上附加 [`CooldownFeature`] 和 [`CoolingTimer`] 组件。
- 将技能实体设置为持有者实体（`ctx.target`）的子实体。
- 配合 [`SkillFeatureBuilderContainer`] 使用，在技能施放时注册为 `"cooldown"` 特征的构建器。

## 定义

```rust
/// 根据冷却特征为实体附加 CooldownFeature 和 CoolingTimer 的构建器。
pub struct CooldownFeatureBuilder;

impl SkillFeatureBuilder for CooldownFeatureBuilder {
    fn build(
        &self,
        commands: &mut Commands,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError> {
        let cooldown_duration = ctx
            .feature
            .get("cooldown_duration")
            .unwrap_or(1.0);

        commands
            .entity(ctx.skill)
            .insert((
                CooldownFeature { cooldown_duration },
                CoolingTimer(Timer::from_seconds(
                    cooldown_duration,
                    TimerMode::Once,
                )),
            ))
            .set_parent_in_place(ctx.target);

        Ok(ctx.skill)
    }
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `build(commands, ctx) -> Result<Entity, BuildError>` | 从 [`SkillFeatureBuilderContext`] 中读取 `cooldown_duration`，在技能实体上插入 [`CooldownFeature`] 和 [`CoolingTimer`] 组件，将技能实体设置为持有者实体（`ctx.target`）的子实体，返回技能实体的 Entity。 |

### build 行为说明

1. 从 `ctx.feature` 的数值字典中获取 `"cooldown_duration"` 键的值，缺失时使用默认值 `1.0`。
2. 在 `ctx.skill`（技能实体）上插入 `CooldownFeature { cooldown_duration }` 和 `CoolingTimer(Timer::from_seconds(cooldown_duration, TimerMode::Once))`。
3. 将技能实体 `ctx.skill` 设置为 `ctx.target`（持有者实体）的子实体。
4. 返回 `ctx.skill`（即被修改的实体）。

## 注册方式

```rust
// 在插件中创建容器并注册
let mut container = SkillFeatureBuilderContainer::new();
container.register("cooldown", CooldownFeatureBuilder);
app.insert_resource(container);
```

或者在已有的 `SkillFeatureBuilderContainer` 中注册：

```rust
fn setup_cooldown_builder(mut container: ResMut<SkillFeatureBuilderContainer>) {
    container.register("cooldown", CooldownFeatureBuilder);
}
```

## 与现有模块的关系

- **[`SkillFeatureBuilder`]**：`CooldownFeatureBuilder` 实现了该 trait，提供从冷却特征定义到 [`CooldownFeature`] 和 [`CoolingTimer`] 组件的构建逻辑。
- **[`SkillFeatureBuilderContext`]**：`build` 方法通过上下文获取 `feature`（特征定义）、`skill`（技能实体）和 `target`（持有者实体）。
- **[`CooldownFeature`]**：`CooldownFeature` 是附加在技能实体上的组件，记录冷却时长。`CooldownFeatureBuilder` 根据特征数据创建并附加该组件。
- **[`CoolingTimer`]**：构建器在技能实体上附加该组件，可由外部系统每帧推进冷却计时。
- **[`SkillFeatureBuilderContainer`]**：按特征 `id` 注册 `CooldownFeatureBuilder`，运行时根据 `"cooldown"` id 查找并执行。
- **`RunSkill` 系统**：在技能施放流程中，使用 `SkillFeatureBuilderContainer` 查找 `"cooldown"` 对应的 builder，执行后目标实体获得冷却计时器。

### 典型使用流程

```
1. 初始化时注册：container.register("cooldown", CooldownFeatureBuilder)。
2. 技能施放时遍历 SkillDefinition.features。
3. 遇到 id = "cooldown" 的特征时，在容器中查找 CooldownFeatureBuilder。
4. 调用 builder.build(commands, ctx) 在技能实体上附加 CooldownFeature 和 CoolingTimer，并将技能实体设置为持有者实体的子实体。
5. 冷却完毕后技能可再次施放。
```

[`SkillFeatureBuilder`]: ./SkillFeatureBuilder.md
[`SkillFeatureBuilderContext`]: ./SkillFeatureBuilder.md#相关类型
[`SkillFeatureBuilderContainer`]: ./SkillFeatureBuilderContainer.md
[`CooldownFeature`]: ./CooldownFeature.md
[`CoolingTimer`]: ../common/CoolingTimer.md
[`SkillRunContext`]: ./SkillRunContext.md
