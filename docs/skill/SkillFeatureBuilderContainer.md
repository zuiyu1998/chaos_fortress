# SkillFeatureBuilderContainer

`SkillFeatureBuilderContainer` 是一个 Resource（资源），用于按特征 `id` 注册和查找 [`SkillFeatureBuilder`]，实现运行时根据特征标识动态选择构建器。

## 用途

- 持有 `SkillFeatureBuilderContainer` 的系统可管理多个 [`SkillFeatureBuilder`] 实例，每个 builder 与一个特征 `id` 绑定。
- 在技能施放时，遍历 [`SkillDefinition`] 的 `features` 列表，对每个特征在容器中查找对应的 builder 并执行。
- 参考 [`RoleBuilderContainer`] 的设计模式。

## 定义

```rust
/// 按特征 id 映射到 SkillFeatureBuilder 的容器。
#[derive(Resource)]
pub struct SkillFeatureBuilderContainer {
    builders: HashMap<
        String,
        Box<dyn for<'w, 's> Fn(
            &'w mut Commands<'w, 's>,
            SkillFeatureBuilderContext,
        ) -> Result<Entity, BuildError> + Send + Sync>,
    >,
}

impl SkillFeatureBuilderContainer {
    /// 创建一个空的容器。
    pub fn new() -> Self { ... }
    /// 注册一个命名 builder。
    pub fn register(&mut self, id: impl Into<String>, builder: impl SkillFeatureBuilder + 'static) { ... }
    /// 按 id 查找并执行 builder，返回生成的 Entity。
    pub fn build<'w, 's>(
        &self,
        id: &str,
        commands: &'w mut Commands<'w, 's>,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError> { ... }
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `new()` | 创建一个空的 `SkillFeatureBuilderContainer`。 |
| `register(id, builder)` | 注册一个实现了 [`SkillFeatureBuilder`] 的构建器，与指定的特征 `id` 绑定。 |
| `build(id, commands, ctx) -> Result<Entity, BuildError>` | 按特征 `id` 查找已注册的 builder，传入上下文执行构建，返回生成的 `Entity`。未找到时返回 `Err(BuildError::MissingBuilder(id))`。 |

## 与现有模块的关系

- **[`SkillFeatureBuilder`]**：`SkillFeatureBuilderContainer` 存储和管理 [`SkillFeatureBuilder`] trait 的实现者，按特征 `id` 索引。
- **[`SkillFeatureBuilderContext`]**：[`SkillFeatureBuilderContext`] 是调用 `build` 时传递的上下文参数，包含特征定义和实体信息。
- **[`SkillDefinition`]**：遍历 `SkillDefinition.features`，对每个 feature 在容器中查找对应的 builder 并执行。
- **[`SkillInstance`]**：施放技能时，通过 [`SkillInstance`] 获取 `skill_id`，再查找 [`SkillDefinition`]，然后使用容器将特征实例化为游戏对象。
- **初始化阶段**：各技能系统在插件 `build` 阶段向容器注册自己的 [`SkillFeatureBuilder`]。

### 典型使用流程

```
1. 插件初始化时创建 SkillFeatureBuilderContainer 并插入 Resource。
2. 各系统注册自己的 builder（如 "projectile" → ProjectileSpawner）。
3. 技能施放时，遍历 SkillDefinition.features。
4. 对每个 feature，调用 container.build(feature.id, commands, ctx)。
5. Builder 生成效果实体（投射物、AoE 区域、召唤物等）。
6. 效果实体在场景中执行逻辑。
```

[`SkillFeatureBuilder`]: ./SkillFeatureBuilder.md
[`SkillFeatureBuilderContext`]: ./SkillFeatureBuilder.md#相关类型
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillInstance`]: ./SkillInstance.md
[`RoleBuilderContainer`]: ../role/RoleBuilderContainer.md
