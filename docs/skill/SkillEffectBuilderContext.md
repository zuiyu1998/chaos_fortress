# SkillEffectBuilderContext

`SkillEffectBuilderContext` 是一个普通结构体，作为上下文传递给 [`SkillEffectBuilder::build`]，携带效果定义和实体信息。

## 用途

- 作为 [`SkillEffectBuilder::build`] 方法的第二个参数，提供构建效果所需的所有输入。
- 将效果定义（[`SkillEffectDefinition`]）、技能实体和持有者实体打包为一个上下文对象，便于 builder 读取参数并操作实体。

## 定义

```rust
/// 上下文：传递给 SkillEffectBuilder::build 的数据。
pub struct SkillEffectBuilderContext {
    /// 要构建的 SkillEffectDefinition 引用。
    pub effect: SkillEffectDefinition,
    /// 施放该技能的实体。
    pub skill: Entity,
    /// 该技能从属的实体（技能持有者/施放者）。
    pub target: Entity,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `effect` | [`SkillEffectDefinition`] | 要构建的效果定义，包含效果 `id` 和参数字典 `params`。Builder 通过 `ctx.effect.get("key")` 读取效果参数。 |
| `skill` | [`Entity`] | 施放该技能的实体。Builder 可通过 `commands.entity(ctx.skill).insert(...)` 在技能实体上附加组件。 |
| `target` | [`Entity`] | 该技能从属的实体（技能持有者/施放者）。Builder 可通过此字段定位目标实体进行操作。 |

## 使用示例

```rust
fn some_effect_system(
    mut commands: Commands,
    container: Res<SkillEffectBuilderContainer>,
    definitions: Res<Assets<SkillDefinition>>,
    instances: Query<&SkillInstance>,
) {
    for instance in &instances {
        if let Some(def) = definitions.get(&instance.skill) {
            for effect in &def.effects {
                let ctx = SkillEffectBuilderContext {
                    effect: effect.clone(),
                    skill: instance.entity(),
                    target: instance.owner(),
                };
                let _ = container.build(&effect.id, &mut commands, ctx);
            }
        }
    }
}
```

## 与现有模块的关系

- **[`SkillEffectBuilder`]**：`SkillEffectBuilderContext` 是 [`SkillEffectBuilder::build`] 方法的第二个参数，提供 builder 执行所需的全部上下文。
- **[`SkillEffectDefinition`]**：`ctx.effect` 字段持有 [`SkillEffectDefinition`] 的引用，builder 从中读取效果参数。
- **[`SkillDefinition`]**：在遍历 [`SkillDefinition`] 的 `effects` 列表时，为每个效果构建对应的 `SkillEffectBuilderContext` 实例。
- **[`SkillFeatureBuilderContext`]**：与 `SkillFeatureBuilderContext` 结构相同（`definition`/`skill`/`target`），区别在于特征上下文持有 [`SkillFeatureDefinition`]，而效果上下文持有 [`SkillEffectDefinition`]。

[`SkillEffectBuilder`]: ./SkillEffectBuilder.md
[`SkillEffectBuilder::build`]: ./SkillEffectBuilder.md#定义
[`SkillEffectDefinition`]: ./SkillEffectDefinition.md
[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`SkillFeatureBuilderContext`]: ./SkillFeatureBuilder.md#SkillFeatureBuilderContext
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillInstance`]: ./SkillInstance.md
