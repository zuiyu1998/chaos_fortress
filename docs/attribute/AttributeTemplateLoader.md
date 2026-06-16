# AttributeTemplateLoader

`AttributeTemplateLoader` 是一个 Bevy [`AssetLoader`]，用于从文件系统加载 [`AttributeTemplate`] 数据。它提供了将属性模板配置外部化的能力，避免在代码中硬编码属性定义。

## 概述

`AttributeTemplateLoader` 实现了 Bevy 的 `AssetLoader` trait，将 `.attribute_template.csv` 文件解析为 [`AttributeTemplate`]（`AttributeTemplate` 本身同时是一个 Bevy [`Resource`] 和 [`Asset`]），使得属性模板可以像其他游戏资源一样通过 Bevy 的 `AssetServer` 进行加载和管理。

## CSV 格式

CSV 文件**必须**包含一个头部行，后续每行数据必须恰好包含四列，以逗号分隔：

```csv
name,base,min,max
hp,100,0,100
max_hp,100,0,3.40282347e+38
armor,30,0,3.40282347e+38
```

| 列名   | 类型  | 说明                              |
|--------|-------|-----------------------------------|
| `name` | str   | 属性名称（在 `AttributeSet` 中作为键） |
| `base` | f32   | 初始基础值                           |
| `min`  | f32   | 最小值下限                           |
| `max`  | f32   | 最大值上限                           |

解析规则：
- 第一行（头部）被自动跳过
- 空行被忽略
- 列值前后的空白符被修剪
- 重复的 `name` 会覆盖之前的定义（后定义者生效）

## 用法

### 注册

`AttributeTemplateLoader` 和 `AttributeTemplate` 的 Asset 注册由 [`AttributePlugin`] 自动完成。只需确保 `AttributePlugin` 已添加到 App 中：

```rust
app.add_plugins(attribute::AttributePlugin);
```

### 加载 CSV 资源

通过 Bevy 的 `AssetServer` 加载：

```rust
fn load_template(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<AttributeTemplate> = asset_server.load("templates/enemy_attributes.attribute_template.csv");
    // 后续可以将 handle 存储为资源，或等待加载完成后使用
}
```

### 在系统中使用加载的模板

```rust
fn use_template(
    template_assets: Res<Assets<AttributeTemplate>>,
    handle: Res<EnemyTemplateHandle>,
) {
    if let Some(template) = template_assets.get(&handle.0) {
        // template 是一个 &AttributeTemplate
        // 使用 template.build_attribute_set(...) 构建实体属性
    }
}
```

### 示例 CSV 文件

**`assets/templates/hero_attributes.attribute_template.csv`**：
```csv
name,base,min,max
hp,150,0,150
max_hp,150,0,3.40282347e+38
attack,25,0,3.40282347e+38
armor,15,0,3.40282347e+38
move_speed,3.5,0,10
```

**`assets/templates/goblin_attributes.attribute_template.csv`**：
```csv
name,base,min,max
hp,40,0,40
max_hp,40,0,3.40282347e+38
attack,8,0,3.40282347e+38
armor,2,0,3.40282347e+38
```

## 与现有模块的关系

- **`AttributeTemplate`**：同时是 Bevy `Resource` 和 `Asset`。程序化创建时作为 `Resource` 使用；从 CSV 文件加载时作为 `Asset` 使用。
- **`AttributeTemplateLoader`**：实现 `AssetLoader`，将 CSV 字节流解析为 `AttributeTemplate`，文件扩展名为 `.attribute_template.csv`。
- **`AttributePlugin`**：负责注册 `AttributeTemplate` 的 Asset 类型和 `AttributeTemplateLoader` 到 Bevy 应用。
- **`asset_tracking`**：项目中已有的 `LoadResource` trait 可将加载完成的 Asset 自动注入为 `Resource`。

## 最佳实践

1. **模板文件管理**：建议将属性模板 CSV 文件存放在 `assets/templates/` 目录下（扩展名为 `.attribute_template.csv`），按角色或实体类型分文件管理。
2. **配合 `load_resource` 使用**：利用项目中已有的 `asset_tracking::LoadResource` trait，可将加载完成的模板自动注入为 `Resource`。
3. **自定义设置**：当前 `AttributeTemplateLoaderSettings` 为空结构体，如需扩展（如分隔符配置、列映射等），可向其中添加字段。
4. **UTF-8 编码**：CSV 文件必须使用 UTF-8 编码，`String::from_utf8` 会在非 UTF-8 输入时返回错误。

[`AssetLoader`]: https://docs.rs/bevy/0.18/bevy/asset/trait.AssetLoader.html
[`Asset`]: https://docs.rs/bevy/0.18/bevy/asset/trait.Asset.html
[`Resource`]: https://docs.rs/bevy/0.18/bevy/ecs/system/trait.Resource.html
[`AttributeTemplate`]: ../src/attribute/mod.rs
[`AttributePlugin`]: ../src/attribute/mod.rs
