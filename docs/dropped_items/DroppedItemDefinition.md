# DroppedItemDefinition

`DroppedItemDefinition` 是掉落物品模块中的**定义层**概念，它描述了某种掉落物品的初始配置（名称、类型、数量范围和权重），是 [`DroppedItemTemplate`](./DroppedItemTemplate.md) 中存储的基本单位。

> 当前代码中尚未实现 `DroppedItemDefinition` 结构体。本文档描述了预期的设计和使用方式，供后续实现参考。

## 定义

```rust
/// 单个掉落物品的模板定义：描述一种物品的掉落配置。
#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct DroppedItemDefinition {
    /// 物品标识符（在模板中作为键使用）。
    pub name: String,
    /// 物品类型（如 "gold"、"exp"、"wood"）。
    pub item_type: String,
    /// 掉落数量下限。
    pub min_amount: u32,
    /// 掉落数量上限。
    pub max_amount: u32,
    /// 掉落权重（值越大，被选中掉落的概率越高）。
    pub weight: f32,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `String` | 物品标识符，在 `DroppedItemTemplate` 中作为键使用（如 `"gold"`、`"exp"`、`"wood"`） |
| `item_type` | `String` | 物品类型字符串，用于区分掉落物的种类和后续处理逻辑（如 `"gold"`、`"exp"`、`"material"`） |
| `min_amount` | `u32` | 单次掉落数量的下限（如金币至少掉落 `1`） |
| `max_amount` | `u32` | 单次掉落数量的上限（如金币最多掉落 `10`） |
| `weight` | `f32` | 掉落权重，用于概率加权随机选择。值越大，被选中掉落的概率越高 |

## 设计说明

### item_type 使用字符串而非枚举

`item_type` 被设计为 `String` 而非枚举，原因如下：

1. **灵活扩展**：新增物品类型无需修改核心结构体定义，只需在业务逻辑中匹配新的字符串值即可。
2. **可配置化**：未来可从外部配置文件（如 JSON、CSV、RON）加载物品定义，字符串类型天然支持序列化。
3. **与现有模式一致**：项目中的其他定义（如 `AttributeDefinition`）同样使用字符串标识符来解耦定义与逻辑。

常见的 `item_type` 值包括：

| 值 | 说明 |
|------|------|
| `"gold"` | 金币/金钱，直接累加到 `LevelState.money` |
| `"exp"` | 经验值，用于角色升级 |
| `"material"` | 材料，可通过 `name` 进一步区分（如 `"wood"`、`"stone"`、`"ore"`） |

## 使用示例

### 构建定义并注册到模板

```rust
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

### 根据 item_type 分发掉落逻辑

```rust
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

## 与现有模块的关系

- **`DroppedItemTemplate`**（[`./DroppedItemTemplate.md`](./DroppedItemTemplate.md)）：`DroppedItemTemplate` 是 `DroppedItemDefinition` 的容器，每个定义描述一种掉落物品的配置。
- **`LevelState`**（`src/level.rs`）：当 `item_type` 为 `"gold"` 时，掉落数量直接累加到 `LevelState.money`。
- **`battle` 模块**（`src/battle/`）：战斗击杀事件可查询 `DroppedItemTemplate` 获取定义，按 `item_type` 分发生成逻辑。

## 最佳实践

1. **item_type 使用常量**：将常用类型定义为常量（如 `const ITEM_GOLD: &str = "gold"`），减少字符串硬编码带来的拼写错误。
2. **定义与逻辑分离**：`DroppedItemDefinition` 只描述"是什么"，不关心"怎么用"。具体的掉落生成逻辑应由业务系统（如 `battle`）根据 `item_type` 分发。
3. **按场景分组**：不同敌人或不同关卡可定义不同的掉落表，通过 `name` 在模板中区分，避免单个配置文件过于庞大。

[`DroppedItemTemplate`]: ./DroppedItemTemplate.md
