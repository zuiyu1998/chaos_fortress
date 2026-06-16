# SkillPlugin

`SkillPlugin` 是一个插件对象，实现了 Bevy 的 [`Plugin`] trait，负责注册技能模块相关的类型。

## 用途

- 向 Bevy 应用注册 [`SkillDefinition`] 为 `Asset`，使其可通过资产系统加载和访问。
- 向 Bevy 应用注册 [`SkillInstance`] 组件到类型反射系统（Type Registry）。
- 初始化 [`SkillFeatureBuilderContainer`] 资源，用于管理和查找技能特征构建器。创建容器时 `"cooldown"` 特征的 [`CooldownFeatureBuilder`] 已默认注册。
- 注册 [`SkillDefinitionLoader`] 资产加载器，支持从 `.skill.toml` 文件加载技能定义。

## 定义

```rust
/// 注册技能相关类型的插件。
pub(super) struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SkillDefinition>();
        app.register_type::<SkillInstance>();
        app.init_resource::<SkillFeatureBuilderContainer>();
        app.register_asset_loader(loader::SkillDefinitionLoader);
    }
}
```

## 注册的资产

| 资产 | 说明 |
|------|------|
| [`SkillDefinition`] | 技能模板定义，可作为 `Asset` 从 `.skill.toml` 文件加载或程序化创建。通过 `init_asset` 注册。 |

## 注册的组件

| 组件 | 说明 |
|------|------|
| [`SkillInstance`] | 存储技能在实体上的运行时状态（充能层数、状态）。通过 `register_type` 注册到类型反射系统。 |
| [`CooldownFeature`] | 冷却特征组件，记录冷却时长。通过 `register_type` 注册到类型反射系统。 |

## 注册的资源

| 资源 | 说明 |
|------|------|
| [`SkillFeatureBuilderContainer`] | 按特征 `id` 注册和查找 [`SkillFeatureBuilder`] 的容器。通过 `init_resource` 初始化。 |

## 注册的加载器

| 加载器 | 文件扩展名 | 说明 |
|--------|-----------|------|
| [`SkillDefinitionLoader`] | `.skill.toml` | 将 TOML 文件解析为 [`SkillDefinition`] 资产。通过 `register_asset_loader` 注册。 |

## 工厂函数

`skill` 工厂函数提供从 [`SkillDefinition`] 快速创建技能实体并应用特征构建器的方式：

```rust
/// 从 SkillDefinition 创建一个技能子实体，并应用其特征构建器。
pub fn skill(
    spawner: &mut ChildSpawnerCommands,
    container: &SkillFeatureBuilderContainer,
    definition: &SkillDefinition,
) -> Entity { ... }
```

创建技能实体作为 `spawner` 的子实体，包含 [`SkillInstance`] 和 `Name` 组件，然后遍历 `definition.features` 使用 [`SkillFeatureBuilderContainer`] 应用每个特征对应的构建器。

## 与现有模块的关系

- **技能模块**（`skill`）：`SkillPlugin` 是技能模块的入口插件，由主应用（`AppPlugin`）的插件列表添加。
- **主应用**（`main`）：在 `src/main.rs` 中以 `skill::SkillPlugin` 的形式被添加至 Bevy 应用。
- **[`SkillDefinition`]**：作为 `Asset` 注册，支持从 `.skill.toml` 文件加载资产。
- **[`SkillInstance`]**：作为 `Component` 注册到类型反射系统，支持运行时反射。
- **[`SkillFeatureBuilderContainer`]**：作为 `Resource` 初始化，供技能系统注册和使用特征构建器。
- **[`SkillDefinitionLoader`]**：实现 `AssetLoader` trait，将 TOML 文件解析为 `SkillDefinition`。
- **角色模块**（`role`）：`RoleAssets` 中包含 `Handle<SkillDefinition>`，用于加载弓箭手等角色的技能定义。
- **[`AttributePlugin`]**：遵循相同的设计模式（`init_asset` + `register_asset_loader`）。

[`SkillDefinition`]: ./SkillDefinition.md
[`SkillInstance`]: ./SkillInstance.md
[`SkillFeatureBuilderContainer`]: ./SkillFeatureBuilderContainer.md
[`SkillFeatureBuilder`]: ./SkillFeatureBuilder.md
[`CooldownFeature`]: ./CooldownFeature.md
[`CooldownFeatureBuilder`]: ./CooldownFeatureBuilder.md
[`SkillDefinitionLoader`]: ./SkillDefinitionLoader.md
[`AttributePlugin`]: ../attribute/AttributePlugin.md
