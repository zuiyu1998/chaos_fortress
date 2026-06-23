# DroppedItemTemplate

`DroppedItemTemplate` 是一个同时实现了 **Bevy `Resource`** 和 **Bevy `Asset`** 的结构体，用于在全局范围内集中定义和管理掉落物品的**模板定义**（每个模板即一个 [`DroppedItemDefinition`](./DroppedItemDefinition.md)）。它提供了一种声明式的方式来预设掉落物品的名称、类型、数量范围和权重，避免在构建代码中重复硬编码。

> 当前代码中尚未实现 `DroppedItemTemplate` 结构体。本文档描述了预期的设计和使用方式，供后续实现参考。

## 概述

`DroppedItemTemplate` 同时扮演两种角色：

- **作为 `Resource`**：可在插件中程序化创建并插入 App，运行时通过 `Res<DroppedItemTemplate>` 直接访问。
- **作为 `Asset`**：可从外部文件（如 CSV、JSON、RON）通过 Bevy 的 `AssetServer` 加载，并配合 `asset_tracking::LoadResource` 自动注入为 `Resource`。

## 定义

```rust
/// 全局掉落物品模板资源/资产，按名称索引多个掉落物品定义。
#[derive(Resource, Asset, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct DroppedItemTemplate {
    /// 以物品名称为键的模板映射。
    pub definitions: HashMap<String, DroppedItemDefinition>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `definitions` | `HashMap<String, DroppedItemDefinition>` | 按名称索引的全部掉落物品模板，支持快速查找和遍历 |

单个物品定义 [`DroppedItemDefinition`](./DroppedItemDefinition.md) 的结构请参见其独立文档。

## 方法说明

| 方法 | 签名 | 说明 |
|------|------|------|
| `new` | `fn new() -> Self` | 创建一个空模板 |
| `define` | `fn define(&mut self, def: DroppedItemDefinition)` | 插入或更新一个掉落物品定义 |
| `get` | `fn get(&self, name: &str) -> Option<&DroppedItemDefinition>` | 按名称查找定义 |

## Asset 加载

`DroppedItemTemplate` 作为 `Asset`，可通过以下方式与文件系统集成。专用的 `DroppedItemTemplateLoader`（详见 [`DroppedItemTemplateLoader.md`](./DroppedItemTemplateLoader.md)）支持从 `.dropped_item_template.csv` 文件加载。

### 注册

在插件中注册 Asset 类型和可选的 `AssetLoader`：

```rust
use dropped_items::DroppedItemTemplateLoader;

impl Plugin for DroppedItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DroppedItemTemplate>();
        app.register_asset_loader(DroppedItemTemplateLoader);
        app.init_resource::<DroppedItemTemplate>();
    }
}
```

### 从文件加载

通过 Bevy 的 `AssetServer` 加载：

```rust
fn load_drop_table(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<DroppedItemTemplate> =
        asset_server.load("drops/enemy_drops.dropped_item_template.csv");
    // 后续可将 handle 存储为资源，等待加载完成后使用
}
```

### from_world 模式

当不需要外部文件时，可实现 `FromWorld` 在程序启动时填充默认定义：

```rust
impl FromWorld for DroppedItemTemplate {
    fn from_world(world: &mut World) -> Self {
        let mut template = Self::new();
        template.define(DroppedItemDefinition {
            name: "gold".into(),
            item_type: "gold".into(),
            min_amount: 1,
            max_amount: 10,
            weight: 100.0,
        });
        template.define(DroppedItemDefinition {
            name: "exp".into(),
            item_type: "exp".into(),
            min_amount: 5,
            max_amount: 20,
            weight: 80.0,
        });
        template.define(DroppedItemDefinition {
            name: "wood".into(),
            item_type: "material".into(),
            min_amount: 1,
            max_amount: 3,
            weight: 40.0,
        });
        template
    }
}
```

然后在插件中通过 `app.init_resource::<DroppedItemTemplate>()` 注册即可自动初始化。

### 配合 asset_tracking 使用

项目中已有的 `asset_tracking::LoadResource` trait 可将加载完成的 Asset 自动注入为 `Resource`，使得系统可以通过 `Res<DroppedItemTemplate>` 直接访问，无需手动处理 `Assets<DroppedItemTemplate>` 存储。使用方法与 `EnemyAssets`、`RoleAssets` 等 Asset 资源的加载方式一致。

## 使用示例

### 注册模板并定义掉落物品

```rust
// 插入模板资源
let mut template = DroppedItemTemplate::new();
template.define(DroppedItemDefinition {
    name: "gold".into(),
    item_type: "gold".into(),
    min_amount: 1,
    max_amount: 10,
    weight: 100.0,
});
template.define(DroppedItemDefinition {
    name: "exp".into(),
    item_type: "exp".into(),
    min_amount: 5,
    max_amount: 20,
    weight: 80.0,
});
template.define(DroppedItemDefinition {
    name: "wood".into(),
    item_type: "material".into(),
    min_amount: 1,
    max_amount: 3,
    weight: 40.0,
});
app.insert_resource(template);
```

### 在系统中使用模板决定掉落

```rust
/// 根据权重随机选取一个掉落物品，并生成实际数量。
fn roll_drop(template: &DroppedItemTemplate) -> Option<(String, u32)> {
    let total_weight: f32 = template.definitions.values().map(|d| d.weight).sum();
    if total_weight <= 0.0 {
        return None;
    }
    let roll = fastrand::f32() * total_weight;
    let mut cumulative = 0.0;
    for def in template.definitions.values() {
        cumulative += def.weight;
        if roll <= cumulative {
            let amount = fastrand::u32(def.min_amount..=def.max_amount);
            return Some((def.name.clone(), amount));
        }
    }
    None
}

fn handle_enemy_death(
    template: Res<DroppedItemTemplate>,
    mut level_state: ResMut<LevelState>,
) {
    if let Some((name, amount)) = roll_drop(&template) {
        if let Some(def) = template.get(&name) {
            match def.item_type.as_str() {
                "gold" => {
                    level_state.money += amount;
                    info!("掉落金币: {}", amount);
                }
                "exp" => {
                    // 累加经验值逻辑
                }
                "material" => {
                    // 添加材料到背包逻辑
                }
                _ => warn!("未知的掉落物品类型: {}", def.item_type),
            }
        }
    }
}
```

### 配合资源加载使用

```rust
fn setup_drops(mut template: ResMut<DroppedItemTemplate>) {
    // 可在 StartUp 或 OnEnter(Screen::Gameplay) 中初始化
    template.define(DroppedItemDefinition {
        name: "gold".into(),
        item_type: "gold".into(),
        min_amount: 5,
        max_amount: 15,
        weight: 100.0,
    });
}
```

## 与现有模块的关系

- **[`DroppedItemDefinition`](./DroppedItemDefinition.md)**：`DroppedItemTemplate` 是 `DroppedItemDefinition` 的容器，每个定义描述一种掉落物品的配置。
- **`LevelState`**（`src/level.rs`）：当 `item_type` 为 `"gold"` 时，掉落金额直接累加到 `LevelState.money`，用于 UI 显示和商店消费。
- **`battle` 模块**（`src/battle/`）：当敌人在战斗中被击杀时，`battle` 系统可依赖 `DroppedItemTemplate` 来生成掉落物。
- **`Enemy` 模块**（`src/enemy/`）：不同敌人类型可拥有独立的掉落配置，未来可在 `EnemyBuilderContext` 或敌人数据中引入掉落表名称，按名称从 `DroppedItemTemplate` 中查找。
- **`LevelPlugin`**：推荐在 `LevelPlugin` 中初始化 `DroppedItemTemplate`，确保进入关卡时可用。
- **`asset_tracking`**（`src/asset_tracking.rs`）：`asset_tracking::LoadResource` trait 可将加载完成的 `DroppedItemTemplate` Asset 自动注入为 `Resource`，使得系统通过 `Res<DroppedItemTemplate>` 直接访问。

## 与 AttributeTemplate 的对比

| 特性 | `AttributeTemplate` | `DroppedItemTemplate` |
|------|-------------------|---------------------|
| 角色 | Resource + Asset | Resource + Asset |
| 元素类型 | `AttributeDefinition` | `DroppedItemDefinition` |
| 键类型 | `String`（属性名） | `String`（物品名） |
| 加载方式 | CSV → `AttributeTemplateLoader` | CSV → `DroppedItemTemplateLoader` |
| 程序创建 | `FromWorld` / 手动 `define` | `FromWorld` / 手动 `define` |

## 最佳实践

1. **在插件中初始化**：推荐在 `LevelPlugin::build` 中通过 `app.init_resource::<DroppedItemTemplate>()` 注册（配合 `FromWorld` 提供默认定义），或通过 `app.insert_resource(template)` 插入自定义实例，并允许其他模块通过 `define` 方法扩展。
2. **按敌人类型分组**：不同敌人可定义不同的掉落表名称（如 `"goblin_drops"`、`"boss_drops"`），在 `DroppedItemTemplate` 中以名称前缀区分，避免全局模板过度膨胀。
3. **配合权重平衡**：使用 `weight` 字段控制掉落概率，避免数值过于随机或固定。建议所有物品的权重在同类中相对可比。
4. **与反射集成**：若需编辑器可视化，可为 `DroppedItemTemplate` 和 `DroppedItemDefinition` derive `Reflect` 并在 `LevelPlugin` 中注册。
5. **外部文件与程序创建二选一**：若掉落表由设计师配置，优先使用 Asset 加载（从 CSV/JSON/RON 文件读取）；若掉落表由代码逻辑动态计算，优先使用 `FromWorld` 程序化创建。两种方式通过 `asset_tracking::LoadResource` 均可统一为 `Res<DroppedItemTemplate>` 访问。
