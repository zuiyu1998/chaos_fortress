# EnemyAssets

`EnemyAssets` 是一个 Bevy [`Resource`] 和 [`Asset`]，用于持有敌人相关的资源句柄，遵循与 [`RoleAssets`] 相同的模式（通过 `asset_tracking::LoadResource` 注册，确保所有依赖加载完成后才插入 Resource）。

## 定义

```rust
#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct EnemyAssets {
    #[dependency]
    pub basic_attributes: Handle<AttributeTemplate>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `basic_attributes` | `Handle<AttributeTemplate>` | 基础敌人属性模板，从 `assets/attribute/basic_enemy.attribute_template.csv` 加载 |

`#[dependency]` 属性告诉 Bevy 该句柄是一个依赖资源，只有在其对应资产（`AttributeTemplate`）完全加载后，`EnemyAssets` 才会被视为就绪。

## 初始化

`EnemyAssets` 由 `EnemyPlugin` 自动注册并加载：

```rust
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // ...
        app.load_resource::<assets::EnemyAssets>();
        // ...
    }
}
```

## 用法

在系统中通过 `Res<EnemyAssets>` 访问：

```rust
fn spawn_enemies(
    mut commands: Commands,
    enemy_assets: Res<EnemyAssets>,
    template_assets: Res<Assets<AttributeTemplate>>,
) {
    if let Some(template) = template_assets.get(&enemy_assets.basic_attributes) {
        let attrs = template.build_attribute_set(&["hp", "max_hp", "armor"]);
        // 使用 attrs 构建敌人实体...
    }
}
```

## 与现有模块的关系

- **[`RoleAssets`]**：`EnemyAssets` 遵循完全相同的模式（`Resource` + `Asset` + `FromWorld`）。
- **`AttributeTemplate`**：`basic_attributes` 句柄指向一个 `AttributeTemplate` 资产，加载自 `basic_enemy.attribute_template.csv`。
- **`asset_tracking::LoadResource`**：负责将 `EnemyAssets` 作为延迟加载资源注册，确保依赖全部就绪后才插入世界。
- **`EnemyPlugin`**：负责调用 `app.load_resource::<EnemyAssets>()`。
- **[`EnemyBuilderContext`](./EnemyBuilderContext.md)**：`enemy()` 函数从模板构建 `AttributeSet` 后通过 `EnemyBuilderContext` 传递给 Builder。
- **[`Enemy`](./Enemy.md)**：标记敌人实体的组件。

[`Resource`]: https://docs.rs/bevy/0.18/bevy/ecs/system/trait.Resource.html
[`Asset`]: https://docs.rs/bevy/0.18/bevy/asset/trait.Asset.html
[`RoleAssets`]: ../role/RoleAssets.md
[`AttributeTemplate`]: ../attribute/mod.rs
