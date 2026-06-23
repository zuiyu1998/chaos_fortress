# DroppedItemTemplateLoader

`DroppedItemTemplateLoader` 是一个 Bevy [`AssetLoader`]，用于从文件系统加载 [`DroppedItemTemplate`]（[`./DroppedItemTemplate.md`](./DroppedItemTemplate.md)）数据。它提供了将掉落物品模板配置外部化的能力，避免在代码中硬编码掉落定义。

## 概述

`DroppedItemTemplateLoader` 实现了 Bevy 的 `AssetLoader` trait，将 `.dropped_item_template.csv` 文件解析为 [`DroppedItemTemplate`]（`DroppedItemTemplate` 本身同时是一个 Bevy [`Resource`] 和 [`Asset`]），使得掉落物品模板可以像其他游戏资源一样通过 Bevy 的 `AssetServer` 进行加载和管理。

## CSV 格式

CSV 文件**必须**包含一个头部行，后续每行数据必须恰好包含五列，以逗号分隔：

```csv
name,item_type,min_amount,max_amount,weight
gold,gold,1,10,100.0
exp,exp,5,20,80.0
wood,material,1,3,40.0
```

| 列名 | 类型 | 说明 |
|------|------|------|
| `name` | str | 物品标识符（在 `DroppedItemTemplate` 中作为键） |
| `item_type` | str | 物品类型字符串（如 `"gold"`、`"exp"`、`"material"`） |
| `min_amount` | u32 | 单次掉落数量下限 |
| `max_amount` | u32 | 单次掉落数量上限 |
| `weight` | f32 | 掉落权重，用于概率加权随机选择 |

解析规则：

- 第一行（头部）被自动跳过
- 空行被忽略
- 列值前后的空白符被修剪
- 重复的 `name` 会覆盖之前的定义（后定义者生效）
- `min_amount` 和 `max_amount` 必须为非负整数
- `weight` 必须为非负浮点数

## 用法

### 注册

`DroppedItemTemplateLoader` 和 `DroppedItemTemplate` 的 Asset 注册由 `LevelPlugin` 或 `DroppedItemPlugin` 自动完成。只需确保对应插件已添加到 App 中：

```rust
app.add_plugins(level::LevelPlugin);
```

或在自定义插件中手动注册：

```rust
impl Plugin for DroppedItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DroppedItemTemplate>();
        app.register_asset_loader(DroppedItemTemplateLoader);
        app.init_resource::<DroppedItemTemplate>();
    }
}
```

### 加载 CSV 资源

通过 Bevy 的 `AssetServer` 加载：

```rust
fn load_drop_table(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<DroppedItemTemplate> =
        asset_server.load("drops/enemy_drops.dropped_item_template.csv");
    // 后续可以将 handle 存储为资源，或等待加载完成后使用
}
```

### 在系统中使用加载的模板

```rust
fn use_drop_table(
    drop_assets: Res<Assets<DroppedItemTemplate>>,
    handle: Res<EnemyDropHandle>,
    mut level_state: ResMut<LevelState>,
) {
    if let Some(template) = drop_assets.get(&handle.0) {
        // template 是一个 &DroppedItemTemplate
        if let Some(gold_def) = template.get("gold") {
            // 使用 gold_def.min_amount、gold_def.max_amount 等
        }
    }
}
```

配合项目中已有的 `asset_tracking::LoadResource` trait，可将加载完成的模板自动注入为 `Res<DroppedItemTemplate>`：

```rust
fn use_drop_table_auto(template: Res<DroppedItemTemplate>) {
    // 无需手动处理 handle，直接通过 Res 访问
    if let Some(def) = template.get("gold") {
        info!("金币掉落配置: {}~{}", def.min_amount, def.max_amount);
    }
}
```

### 示例 CSV 文件

**`assets/drops/goblin_drops.dropped_item_template.csv`**：

```csv
name,item_type,min_amount,max_amount,weight
gold,gold,1,5,100.0
exp,exp,3,10,60.0
bone,material,1,2,30.0
```

**`assets/drops/boss_drops.dropped_item_template.csv`**：

```csv
name,item_type,min_amount,max_amount,weight
gold,gold,10,30,100.0
exp,exp,20,50,80.0
gem,material,1,3,20.0
rare_gem,material,1,1,5.0
```

## 与现有模块的关系

- **[`DroppedItemTemplate`](./DroppedItemTemplate.md)**：同时是 Bevy `Resource` 和 `Asset`。程序化创建时作为 `Resource` 使用；从 CSV 文件加载时作为 `Asset` 使用。
- **`DroppedItemTemplateLoader`**：实现 `AssetLoader`，将 CSV 字节流解析为 `DroppedItemTemplate`，文件扩展名为 `.dropped_item_template.csv`。
- **`LevelPlugin`**（`src/level.rs`）：负责注册 `DroppedItemTemplate` 的 Asset 类型和 `DroppedItemTemplateLoader` 到 Bevy 应用。
- **`asset_tracking`**（`src/asset_tracking.rs`）：项目中已有的 `LoadResource` trait 可将加载完成的 Asset 自动注入为 `Resource`。

## 与 AttributeTemplateLoader 的对比

| 特性 | `AttributeTemplateLoader` | `DroppedItemTemplateLoader` |
|------|--------------------------|----------------------------|
| 目标类型 | `AttributeTemplate` | `DroppedItemTemplate` |
| 文件扩展名 | `.attribute_template.csv` | `.dropped_item_template.csv` |
| CSV 列数 | 4 列 (`name,base,min,max`) | 5 列 (`name,item_type,min_amount,max_amount,weight`) |
| 数值字段 | `base` (f32)、`min` (f32)、`max` (f32) | `min_amount` (u32)、`max_amount` (u32)、`weight` (f32) |
| 字符串字段 | `name` | `name`、`item_type` |

## 最佳实践

1. **模板文件管理**：建议将掉落物品 CSV 文件存放在 `assets/drops/` 目录下，按敌人类型或关卡分文件管理（如 `goblin_drops.dropped_item_template.csv`）。
2. **配合 `load_resource` 使用**：利用项目中已有的 `asset_tracking::LoadResource` trait，可将加载完成的模板自动注入为 `Resource`，无需手动管理 `Handle`。
3. **权重值保持正数**：`weight` 应始终为正浮点数。若所有权重之和为 0，随机掉落逻辑将无物品可掉落；负权重会导致未定义行为。
4. **数量范围合理性**：确保 `min_amount <= max_amount`，避免解析时通过业务逻辑校验产生无效配置。
5. **UTF-8 编码**：CSV 文件必须使用 UTF-8 编码，`String::from_utf8` 会在非 UTF-8 输入时返回错误。

[`AssetLoader`]: https://docs.rs/bevy/0.18/bevy/asset/trait.AssetLoader.html
[`Asset`]: https://docs.rs/bevy/0.18/bevy/asset/trait.Asset.html
[`Resource`]: https://docs.rs/bevy/0.18/bevy/ecs/system/trait.Resource.html
[`DroppedItemTemplate`]: ./DroppedItemTemplate.md
