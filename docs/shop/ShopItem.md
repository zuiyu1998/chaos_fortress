# ShopItem

`ShopItem` 是商店模块中的**定义层**概念，描述商店中上架的一种可购买道具，包括道具的名称、描述、价格、类型和购买后的效果。

> 当前代码中商店仅有占位按钮，尚未实现 `ShopItem` 相关逻辑。本文档描述了预期的设计，供后续实现参考。

## 设计动机

商店系统需要一种统一的数据结构来描述所有可购买的道具。`ShopItem` 将道具的展示信息（名称、描述、价格）与行为信息（类型、效果 ID）封装为一个结构体，使得：

- 商店 UI 可以直接渲染道具列表
- 购买系统可以根据 `item_type` 分发购买逻辑
- 未来可从外部配置文件（如 JSON、RON）加载商店道具

## 定义

```rust
/// 商店中出售的道具定义。
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct ShopItem {
    /// 道具唯一标识符（用于在商店列表中查找）。
    pub id: String,
    /// 道具名称（用于 UI 显示）。
    pub name: String,
    /// 道具描述（用于 UI 显示）。
    pub description: String,
    /// 购买价格（金币）。
    pub price: u32,
    /// 道具类型（决定购买后的行为，如 "role"、"enhancement"、"consumable"）。
    pub item_type: String,
    /// 道具关联的参数值（如购买角色时对应 builder_name）。
    pub value: String,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `String` | 道具唯一标识符，用于在商店物品列表中查找和引用（如 `"archer_recruit"`、`"hp_potion"`） |
| `name` | `String` | 道具名称，在商店 UI 中显示（如 `"招募弓箭手"`、`"生命药水"`） |
| `description` | `String` | 道具描述，在商店 UI 中显示（如 `"在场上增加一名弓箭手"`、`"恢复 50 点生命值"`） |
| `price` | `u32` | 购买价格（金币），购买时从 `LevelState.money` 中扣除 |
| `item_type` | `String` | 道具类型，决定购买后的行为分发（如 `"role"`、`"enhancement"`、`"consumable"`） |
| `value` | `String` | 道具关联的参数值，含义取决于 `item_type`（如角色 builder 名称、效果定义 ID） |

## 设计决策

### 使用字符串而非枚举区分道具类型

`item_type` 被设计为 `String` 而非枚举，原因如下：

1. **灵活扩展**：新增道具类型无需修改核心结构体定义，只需在业务逻辑中匹配新的字符串值即可。
2. **可配置化**：未来可从外部配置文件（如 RON、JSON）加载商店道具列表，字符串类型天然支持序列化。
3. **与掉落物品一致**：`DroppedItemDefinition` 同样使用 `String` 类型的 `item_type` 字段，保持模块间设计风格统一。
4. **运行时动态性**：某些关卡可能通过脚本地动态添加自定义道具类型，字符串方式无需重新编译即可支持。

常见的 `item_type` 值包括：

| 值 | 说明 |
|---|---|
| `"role"` | 招募角色：在棋盘上添加一个新角色，`value` 为 `RoleBuilderContainer` 中的注册名 |
| `"enhancement"` | 强化道具：对指定目标施加临时或永久增益，`value` 为效果定义标识符 |
| `"consumable"` | 消耗品：一次性使用的物品，`value` 为效果定义标识符 |

### price 与 `LevelState.money` 的关系

- 购买时系统检查 `LevelState.money >= ShopItem.price`
- 购买成功后扣除 `LevelState.money -= ShopItem.price`
- 若余额不足，购买请求被拒绝，UI 给出反馈

### value 字段的设计

`value` 字段是一个通用字符串，具体含义由 `item_type` 决定：

| `item_type` | `value` 含义 | 示例 |
|---|---|---|
| `"role"` | 角色在 `RoleBuilderContainer` 中的注册名 | `"archer"`、`"swordsman"` |
| `"enhancement"` | 效果定义标识符 | `"attack_boost"`、`"armor_up"` |
| `"consumable"` | 效果定义标识符 | `"hp_potion"`、`"rage_potion"` |

使用字符串而非泛型参数，便于从外部配置文件加载。

## 使用示例

### 注册商店道具

```rust
let shop_items = vec![
    ShopItem {
        id: "recruit_archer".into(),
        name: "招募弓箭手".into(),
        description: "在场上增加一名弓箭手".into(),
        price: 50,
        item_type: "role".into(),
        value: "archer".into(),
    },
    ShopItem {
        id: "hp_potion".into(),
        name: "生命药水".into(),
        description: "恢复 50 点生命值".into(),
        price: 20,
        item_type: "consumable".into(),
        value: "hp_potion_effect".into(),
    },
    ShopItem {
        id: "attack_boost".into(),
        name: "攻击强化".into(),
        description: "所有角色攻击力提升 10%，持续 30 秒".into(),
        price: 80,
        item_type: "enhancement".into(),
        value: "attack_boost_30s".into(),
    },
];

app.insert_resource(ShopItemList { items: shop_items });
```

### 购买逻辑分发

```rust
fn handle_purchase(
    mut level_state: ResMut<LevelState>,
    shop_list: Res<ShopItemList>,
    role_container: Res<RoleBuilderContainer>,
    purchase_events: EventReader<PurchaseEvent>,
    mut commands: Commands,
) {
    for event in purchase_events.read() {
        let Some(item) = shop_list.find(&event.item_id) else {
            warn!("商店道具不存在: {}", event.item_id);
            continue;
        };

        if level_state.money < item.price {
            warn!("金币不足，无法购买: {}", item.name);
            continue;
        }

        level_state.money -= item.price;

        match item.item_type.as_str() {
            "role" => {
                // 使用 RoleBuilderContainer 在棋盘上生成角色
                let ctx = RoleBuilderContext { /* ... */ };
                role_container.build(&item.value, &mut commands, ctx);
            }
            "enhancement" => {
                // 应用全局强化效果
                apply_enhancement(&item.value);
            }
            "consumable" => {
                // 使用消耗品效果
                apply_consumable(&item.value);
            }
            _ => warn!("未知的道具类型: {}", item.item_type),
        }
    }
}
```

### 商店 UI 渲染（伪代码）

```rust
fn render_shop_ui(
    shop_list: Res<ShopItemList>,
    level_state: Res<LevelState>,
) {
    for item in &shop_list.items {
        // 渲染道具卡片：名称、描述、价格
        // 若 level_state.money >= item.price，按钮为可点击状态
        // 否则按钮置灰并提示金币不足
        // 点击后发送 PurchaseEvent { item_id: item.id.clone() }
    }
}
```

## 与现有模块的关系

- **`LevelState`**（[`../level/LevelState.md`](../level/LevelState.md)）：`LevelState.money` 是商店系统的经济基础，购买时从中扣除金币。
- **`RoleBuilderContainer`**（[`../role/RoleBuilderContainer.md`](../role/RoleBuilderContainer.md)）：当 `item_type` 为 `"role"` 时，通过 `RoleBuilderContainer` 在棋盘上生成角色实体。
- **`RoleBuilderContext`**（[`../role/RoleBuilderContext.md`](../role/RoleBuilderContext.md)）：购买角色时构造上下文，传入角色位置、父实体等信息。
- **`Shop`**（[`./Shop.md`](./Shop.md)）：商店状态，控制商店 UI 的打开与关闭。
- **UI 系统**（`src/level.rs`）：商店 UI 面板读取 `ShopItems` 资源渲染道具列表，监听用户点击事件。
- **关卡模块**（[`../level/LevelPlugin.md`](../level/LevelPlugin.md)）：商店系统作为 `LevelPlugin` 的一部分注册和初始化。

## 预期容器

```rust
/// 商店道具列表资源，存储当前关卡可购买的所有道具。
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct ShopItemList {
    pub items: Vec<ShopItem>,
}

impl ShopItemList {
    /// 按 id 查找道具。
    pub fn find(&self, id: &str) -> Option<&ShopItem> {
        self.items.iter().find(|item| item.id == id)
    }
}
```

## 变更指南

当需要新增道具类型时：

1. 确定新类型的字符串标识符（如 `"buff"`、`"upgrade"`），并在注册道具时使用该值。
2. 在 `handle_purchase` 等购买分发的 `match` 表达式中补充新类型的分支（使用 `as_str()` 匹配字符串）。
3. 在文档的"value 字段的设计"表格中补充新类型对应的 `value` 含义。
4. 建议将常用类型定义为常量（如 `const ITEM_ROLE: &str = "role"`），减少字符串硬编码带来的拼写错误。

当需要新增道具字段时：

1. 在 `ShopItem` 结构体中添加新字段。
2. 更新本文档的字段说明表和所有构造 `ShopItem` 的调用点。
3. 如果新字段影响 UI 显示，同步更新 UI 渲染系统。

## 最佳实践

1. **道具 ID 采用命名约定**：使用 `{type}_{detail}` 格式，如 `recruit_archer`、`enhance_attack`、`potion_hp`，确保全局唯一且易于理解。
2. **价格平衡**：道具定价应与关卡的经济产出（击杀奖励、波次结算）相匹配，避免价格过高或过低。
3. **道具数量适中**：单个关卡的商店道具建议控制在 5-10 个，过多的选项会增加玩家决策负担。
4. **分离定义与逻辑**：`ShopItem` 只描述"卖什么"，不关心"怎么用"。购买后的逻辑由业务系统根据 `item_type` 分发。
5. **使用常量定义道具类型和 ID**：将常用道具类型（如 `const ITEM_TYPE_ROLE: &str = "role"`）和道具 ID 定义为常量，减少字符串硬编码带来的拼写错误。
