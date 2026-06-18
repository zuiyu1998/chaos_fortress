# FireBulletBuilder

`FireBulletBuilder` 是一个结构体，实现了 [`SkillEffectBuilder`] trait，用于根据 [`SkillEffectDefinition`] 的参数字典为技能实体附加 [`FireBulletEffect`] 组件。

## 用途

- 从 [`SkillEffectBuilderContext`] 中获取效果定义（[`SkillEffectDefinition`]），通过 [`FireBulletEffect::from_effect`] 解析出子弹参数。
- 在技能实体（`ctx.skill`）上插入 [`FireBulletEffect`] 组件，供后续子弹生成系统读取。
- 配合 [`SkillEffectBuilderContainer`] 在技能施放时注册为 `"fire_bullet"` 效果构建器。

## 定义

```rust
/// 根据 FireBulletEffect 参数为技能实体附加组件的构建器。
pub struct FireBulletBuilder;

impl SkillEffectBuilder for FireBulletBuilder {
    fn build(
        &self,
        commands: &mut Commands,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError> {
        let params = FireBulletEffect::from_effect(&ctx.effect)
            .ok_or_else(|| {
                BuildError::BuildFailed(
                    format!("missing fire_bullet params for effect '{}'", ctx.effect.id)
                )
            })?;

        commands
            .entity(ctx.skill)
            .insert(params);

        Ok(ctx.skill)
    }
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `build(commands, ctx) -> Result<Entity, BuildError>` | 从 [`SkillEffectBuilderContext`] 中读取 [`FireBulletEffect`] 参数，在技能实体（`ctx.skill`）上插入 [`FireBulletEffect`] 组件。返回技能实体的 `Entity`。 |

### build 行为说明

1. 通过 [`FireBulletEffect::from_effect`] 从 `ctx.effect`（[`SkillEffectDefinition`]）中解析出 [`FireBulletEffect`] 数据对象，包含 `speed`、`damage`、`count` 三个字段。
2. 若解析失败（缺少必要参数），返回 `Err(BuildError::BuildFailed(...))`。
3. 在技能实体 `ctx.skill` 上插入 [`FireBulletEffect`] 组件，供后续子弹生成系统（如 [`BattlePlugin`] 中的 `fire_bullet_on_skill` 系统）读取。
4. 返回 `ctx.skill`（即被修改的技能实体）。

[`BattlePlugin`]: ./BattlePlugin.md
[`fire_bullet_on_skill`]: ./BattlePlugin.md#fire_bullet_on_skill

## 注册方式

```rust
// 在插件中创建容器并注册
let mut container = SkillEffectBuilderContainer::new();
container.register("fire_bullet", FireBulletBuilder);
app.insert_resource(container);
```

或者在已有的 [`SkillEffectBuilderContainer`] 中注册：

```rust
fn setup_fire_bullet_builder(mut container: ResMut<SkillEffectBuilderContainer>) {
    container.register("fire_bullet", FireBulletBuilder);
}
```

## 与现有模块的关系

- **[`SkillEffectBuilder`]**：`FireBulletBuilder` 实现了该 trait，提供从 [`SkillEffectDefinition`] 到 [`FireBulletEffect`] 组件的构建逻辑。
- **[`SkillEffectBuilderContext`]**：`build` 方法通过上下文获取 `effect`（效果定义）、`skill`（技能实体）和 `target`（技能持有者实体）。
- **[`SkillEffectBuilderContainer`]**：按效果 `id` 注册 `FireBulletBuilder`，运行时根据 `"fire_bullet"` 查找并执行。
- **[`FireBulletEffect`]**：`FireBulletBuilder` 使用 [`FireBulletEffect::from_effect`] 从效果定义参数字典中解析出 `speed`、`damage`、`count` 三个运行时参数，并将该组件插入到技能实体上。
- **[`BattlePlugin`]**：`BattlePlugin` 中的 `fire_bullet_on_skill` 系统从技能实体上读取 [`FireBulletEffect`] 组件，根据其参数（`speed`、`damage`、`count`）生成子弹实体。

### 典型使用流程

```
1. 初始化时注册：container.register("fire_bullet", FireBulletBuilder)。
2. 技能施放时遍历 SkillDefinition.effects。
3. 遇到 id = "fire_bullet" 的效果时，在 SkillEffectBuilderContainer 中查找 FireBulletBuilder。
4. 调用 builder.build(commands, ctx) 在技能实体上插入 FireBulletEffect 组件。
5. 后续系统（如 fire_bullet_on_skill）读取该组件，根据参数生成子弹实体。
```

[`SkillEffectBuilder`]: ../skill/SkillEffectBuilder.md
[`SkillEffectBuilderContext`]: ../skill/SkillEffectBuilder.md#相关类型
[`SkillEffectBuilderContainer`]: ../skill/SkillEffectBuilderContainer.md
[`SkillEffectDefinition`]: ../skill/SkillEffectDefinition.md
[`FireBulletEffect`]: ./FireBulletEffect.md
[`FireBulletEffect::from_effect`]: ./FireBulletEffect.md#定义
