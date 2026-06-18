# SkillEffectBuilderContainer

`SkillEffectBuilderContainer` 是一个 Resource（资源），用于按效果 `id` 注册和查找 [`SkillEffectBuilder`]，实现运行时根据效果标识动态选择构建器。

## 用途

- 持有 `SkillEffectBuilderContainer` 的系统可管理多个 [`SkillEffectBuilder`] 实例，每个 builder 与一个效果 `id` 绑定。
- 在技能施放时，遍历 [`SkillDefinition`] 的 `effects` 列表，对每个效果在容器中查找对应的 builder 并执行。
- 参考 [`SkillFeatureBuilderContainer`] 和 [`RoleBuilderContainer`] 的设计模式。

## 定义

```rust
/// 按效果 id 映射到 SkillEffectBuilder 的容器。
#[derive(Resource)]
pub struct SkillEffectBuilderContainer {
    builders: HashMap<
        String,
        Box<dyn for<'w, 's> Fn(
            &'w mut Commands<'w, 's>,
            SkillEffectBuilderContext,
        ) -> Result<Entity, BuildError> + Send + Sync>,
    >,
}

impl SkillEffectBuilderContainer {
    /// 创建一个空的容器。
    pub fn new() -> Self { ... }
    /// 注册一个命名的效果构建器。
    pub fn register(&mut self, id: impl Into<String>, builder: impl SkillEffectBuilder + 'static) { ... }
    /// 按 id 查找并执行 builder，返回生成的 Entity。
    pub fn build<'w, 's>(
        &self,
        id: &str,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError> { ... }
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `new()` | 创建一个新的 `SkillEffectBuilderContainer`，初始为空。 |
| `register(id, builder)` | 注册一个实现了 [`SkillEffectBuilder`] 的构建器，与指定的效果 `id` 绑定。 |
| `build(id, commands, ctx) -> Result<Entity, BuildError>` | 按效果 `id` 查找已注册的 builder，传入上下文执行构建，返回生成的 `Entity`。未找到时返回 `Err(BuildError::MissingBuilder(id))`。 |

## 与现有模块的关系

- **[`SkillEffectBuilder`]**：`SkillEffectBuilderContainer` 存储和管理 [`SkillEffectBuilder`] trait 的实现者，按效果 `id` 索引。
- **[`SkillEffectBuilderContext`]**：[`SkillEffectBuilderContext`] 是调用 `build` 时传递的上下文参数，包含效果定义和实体信息。
- **[`SkillDefinition`]**：遍历 `SkillDefinition.effects`，对每个 effect 在容器中查找对应的 builder 并执行。
- **[`SkillFeatureBuilderContainer`]**：与 `SkillEffectBuilderContainer` 结构相同、职责互补。前者管理特征构建器（数值维度），后者管理效果构建器（行为维度）。
- **[`SkillInstance`]**：施放技能时，通过 [`SkillInstance`] 获取 `skill_id`，再查找 [`SkillDefinition`]，然后使用容器将效果定义实例化为游戏对象。
- **[`SkillEvent`]**：效果系统可以监听 [`SkillEvent`] 消息，在技能完成后通过容器执行效果定义。
- **初始化阶段**：各效果系统在插件 `build` 阶段向容器注册自己的 [`SkillEffectBuilder`]。

### 典型使用流程

```
1. 创建空的 SkillEffectBuilderContainer。
2. 各系统注册自己的 builder（如 "fire_bullet" → FireBulletBuilder）。
3. 技能施放时，遍历 SkillDefinition.effects。
4. 对每个 effect，调用 container.build(effect.id, commands, ctx)。
5. Builder 生成效果实体（子弹、治疗效果、AoE 区域等）。
6. 效果实体在场景中执行逻辑。
```

[`SkillEffectBuilder`]: ./SkillEffectBuilder.md
[`SkillEffectBuilderContext`]: ./SkillEffectBuilder.md#相关类型
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillInstance`]: ./SkillInstance.md
[`SkillEvent`]: ./SkillEvent.md
[`SkillFeatureBuilderContainer`]: ./SkillFeatureBuilderContainer.md
[`RoleBuilderContainer`]: ../role/RoleBuilderContainer.md
