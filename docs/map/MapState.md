# MapState

`MapState` 是一个**资源（Resource）**，用于存储地图上所有属于备战区（Preparation Zone）和交战区（Combat Zone）的格子实体及其关联的角色实体。它提供统一的查询入口，使系统无需遍历整个实体树即可按区域或坐标访问格子信息。

## 设计动机

地图系统将 12×5 的棋盘格划分为备战区（列 0~1）和交战区（列 2~11）。在游戏运行过程中，多个系统需要快速查找指定位置的格子或角色：

- 备战区系统：查询空闲格子以放置购买的角色
- 战斗系统：获取交战区所有角色进行攻击/技能判定
- UI 系统：高亮显示可操作格子
- 地图交互系统：根据点击坐标定位格子

`MapState` 在关卡初始化时构建，将格子实体按区域和坐标索引，避免每次查询都遍历整个实体层级。

## 定义

```rust
/// 地图状态资源，存储备战区与交战区的格子实体索引。
///
/// 在关卡初始化时由系统构建，供其他系统查询使用。
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct MapState {
    /// 备战区格子实体索引。
    ///
    /// 第一层 key 为列（0 或 1），第二层 key 为行（0~4）。
    /// 值对为 (格子实体, 可选的角色子实体)。
    pub bench_zone: HashMap<u32, HashMap<u32, CellEntry>>,

    /// 交战区格子实体索引。
    ///
    /// 第一层 key 为列（2~11），第二层 key 为行（0~4）。
    /// 值对为 (格子实体, 可选的角色子实体)。
    pub combat_zone: HashMap<u32, HashMap<u32, CellEntry>>,
}

/// 一个地图格子的实体记录，包含格子本身及其上的角色。
#[derive(Debug, Clone, Reflect)]
pub struct CellEntry {
    /// 格子实体的 Entity。
    pub cell: Entity,
    /// 格子上的角色实体（如有）。
    pub role: Option<Entity>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `bench_zone` | `HashMap<u32, HashMap<u32, CellEntry>>` | 备战区格子索引，列 0~1，行 0~4。每个格子记录 `CellEntry` |
| `combat_zone` | `HashMap<u32, HashMap<u32, CellEntry>>` | 交战区格子索引，列 2~11，行 0~4。每个格子记录 `CellEntry` |

### CellEntry

| 字段 | 类型 | 说明 |
|------|------|------|
| `cell` | `Entity` | 格子实体的标识符，用于向其添加组件或子实体 |
| `role` | `Option<Entity>` | 当前停留在此格子上的角色实体。没有角色时为 `None` |

## 使用示例

### 查询备战区空闲格子

```rust
fn find_empty_bench_slot(map_state: Res<MapState>) -> Option<(u32, u32, Entity)> {
    for col in 0..=1 {
        if let Some(rows) = map_state.bench_zone.get(&col) {
            for row in 0..=4 {
                if let Some(entry) = rows.get(&row) {
                    if entry.role.is_none() {
                        return Some((col, row, entry.cell));
                    }
                }
            }
        }
    }
    None
}
```

### 购买后更新角色位置

```rust
fn buy_archer(
    click: On<Pointer<Click>>,
    mut level_state: ResMut<LevelState>,
    mut map_state: ResMut<MapState>,
    query: Query<&ShopItem>,
    mut commands: Commands,
) {
    let item = query.get(click.entity()).unwrap();
    if level_state.money < item.price { return; }
    level_state.money -= item.price;

    // 查找备战区第一个空闲格子
    for col in 0..=1 {
        let rows = map_state.bench_zone.get(&col).unwrap();
        for row in 0..=4 {
            let entry = rows.get(&row).unwrap();
            if entry.role.is_none() {
                let role_entity = // 在此格子上生成角色...
                // 更新 MapState
                let cell_col = map_state.bench_zone.get_mut(&col).unwrap();
                cell_col.get_mut(&row).unwrap().role = Some(role_entity);
                return;
            }
        }
    }
}
```

### 遍历交战区所有角色

```rust
fn for_each_combat_role(map_state: Res<MapState>) {
    for (col, rows) in &map_state.combat_zone {
        for (row, entry) in rows {
            if let Some(role_entity) = entry.role {
                info!("交战区角色位于 ({}, {})，实体 {:?}", col, row, role_entity);
            }
        }
    }
}
```

## 与相关类型的关系

- **`Map`**（[`./Map.md`](./Map.md)）：地图根实体标记组件。`MapState` 记录的是 `Map` 实体下的子格子实体。
- **`MapData`**（[`./MapData.md`](./MapData.md)）：地图规格资源，提供 `width`（12）、`height`（5）、`cell_size`（64.0）用于坐标计算。
- **`MapSystem`**（[`./MapSystem.md`](./MapSystem.md)）：地图系统文档，包含备战区/交战区的布局说明和工具函数接口。
- **`RoleShopItem`**（[`../shop/RoleShopItem.md`](../shop/RoleShopItem.md)）：备战区角色生成组件。购买后系统据此在格子上生成角色实体，并更新 `MapState` 中的 `role` 字段。
- **`ShopItems`**（[`../shop/ShopItem.md`](../shop/ShopItem.md)）：商店物品资源，购买行为触发 `MapState` 的更新。

## 数据流

```
关卡初始化
    │
    ▼
map() 生成 12×5 格子实体
    │
    ▼
初始化系统构建 MapState（按列/行索引所有格子）
    │
    ▼
    ├─ 备战区系统：查询空闲格子 → 插入 RoleShopItem
    │                              → spawn_bench_roles 生成角色
    │                              → 更新 MapState.bench_zone[col][row].role
    │
    └─ 战斗系统：遍历 MapState.combat_zone 获取所有角色
       UI 系统：根据 MapState 高亮可操作格子
```

## 设计决策

### HashMap 嵌套而非平铺

使用 `HashMap<u32, HashMap<u32, CellEntry>>` 嵌套结构而非单层平铺（如 `HashMap<(u32, u32), CellEntry>`），原因如下：

1. **按列查询高效**：系统常需要遍历某一区域的所有格子（如"备战区第 0 列所有行"），外层 key 为列可实现按列快速切片。
2. **与区域概念对齐**：`bench_zone` 和 `combat_zone` 两个字段各自独立，列范围清晰（0~1 vs 2~11），不会混淆。
3. **遍历顺序明确**：外层按列、内层按行遍历，与地图渲染和玩家视觉习惯一致。

### 使用 HashMap 而非 Vec

格子数量固定（12×5 = 60），使用 `HashMap` 而非 `Vec` 或数组的原因：

1. **稀疏访问**：尽管格子是连续的，但系统通常只访问特定坐标，`HashMap` 提供 O(1) 查找。
2. **Entity 不可索引**：`Entity` 是 Bevy 的不透明标识符，不能直接用作数组索引。
3. **灵活性**：未来若支持动态地图（如扩展列数），`HashMap` 无需调整数组大小。
4. **可考虑用 `Grid<CellEntry>` 封装**：如果性能敏感，后续可重构为固定大小数组 + 行列计算。

### CellEntry.role 为 Option<Entity>

`role` 字段直接存储角色实体标识符（而非引用子实体或存储坐标），原因如下：

1. **直接访问**：系统拿到 `Entity` 后即可通过 `Query` 读取角色组件，无需额外查找。
2. **生命周期解耦**：角色实体的创建和销毁由其自身系统管理，`MapState` 只是记录"哪个格子现在有什么"，不负责生命周期。
3. **简单更新**：角色移动、死亡或交换时，只需更新对应格子的 `role` 为 `Some(new_entity)` 或 `None`。

### 构建时机

`MapState` 应在 `Map` 实体及其子格子实体生成后立即初始化，建议在 `OnEnter(Screen::Gameplay)` 或关卡加载系统的末尾执行，确保其他系统查询时数据已就绪。

## 相关文档

- [`Map.md`](./Map.md)：地图根实体标记组件
- [`MapData.md`](./MapData.md)：地图规格数据资源
- [`MapSystem.md`](./MapSystem.md)：地图系统文档（备战区/交战区布局、工具函数）
- [`RoleShopItem.md`](../shop/RoleShopItem.md)：备战区角色生成组件
- [`ShopItem.md`](../shop/ShopItem.md)：商店物品定义
