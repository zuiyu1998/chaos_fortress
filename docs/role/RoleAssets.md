# RoleAssets

`RoleAssets` 是一个 Bevy [`Resource`] 和 [`Asset`]，用于持有角色相关的资源句柄，遵循与 [`LevelAssets`] 相同的模式（通过 `asset_tracking::LoadResource` 注册，确保所有依赖加载完成后才插入 Resource）。

## 定义

```rust
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RoleAssets {
    #[dependency]
    pub archer_attributes: Handle<AttributeTemplate>,
    #[dependency]
    pub archer_skill: Handle<SkillDefinition>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `archer_attributes` | `Handle<AttributeTemplate>` | 弓箭手属性模板，从 `assets/attribute/archer.attribute_template.csv` 加载 |
| `archer_skill` | `Handle<SkillDefinition>` | 弓箭手技能定义，从 `assets/skill/archer.skill.toml` 加载 |

`#[dependency]` 属性告诉 Bevy 该句柄是一个依赖资源，只有在其对应资产（`AttributeTemplate`）完全加载后，`RoleAssets` 才会被视为就绪。

## 初始化

`RoleAssets` 由 `RolePlugin` 自动注册并加载：

```rust
impl Plugin for RolePlugin {
    fn build(&self, app: &mut App) {
        // ...
        app.load_resource::<assets::RoleAssets>();
        // ...
    }
}
```

## 用法

在系统中通过 `Res<RoleAssets>` 访问：

```rust
fn spawn_archer(
    mut commands: Commands,
    role_assets: Res<RoleAssets>,
    template_assets: Res<Assets<AttributeTemplate>>,
) {
    if let Some(template) = template_assets.get(&role_assets.archer_attributes) {
        let attrs = template.build_attribute_set(&["max_hp", "attack", "defense"]);
        // 使用 attrs 构建角色实体...
    }
}
```

## 与现有模块的关系

- **`LevelAssets`**：`RoleAssets` 遵循完全相同的模式（`Resource` + `Asset` + `FromWorld`）。
- **`AttributeTemplate`**：`archer_attributes` 句柄指向一个 `AttributeTemplate` 资产，加载自 `archer.attribute_template.csv`。
- **`asset_tracking::LoadResource`**：负责将 `RoleAssets` 作为延迟加载资源注册，确保依赖全部就绪后才插入世界。
- **`RolePlugin`**：负责调用 `app.load_resource::<RoleAssets>()`。

[`Resource`]: https://docs.rs/bevy/0.18/bevy/ecs/system/trait.Resource.html
[`Asset`]: https://docs.rs/bevy/0.18/bevy/asset/trait.Asset.html
[`LevelAssets`]: ../src/level.rs
[`AttributeTemplate`]: ../src/attribute/mod.rs
