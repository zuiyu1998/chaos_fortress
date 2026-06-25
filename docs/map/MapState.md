# MapState

`MapState` 是一个**资源（Resource）**，存储地图上所有格子的 `MapCellData`，以 `MapCell` 为键进行索引。由 `sync_map_state_on_add` 系统在格子生成时自动填充。

## 设计动机

地图上的每个格子实体都带有 `MapCell`（坐标）和 `MapCellData`（角色实体）组件。ECS 查询通过实体遍历可以获取这些数据，但某些场景需要**按坐标直接查找**：

- **备战区系统**：给定一个坐标，快速判断格子是否空闲
- **战斗系统**：根据坐标范围获取所有格子上的角色
- **UI 系统**：根据点击坐标查找对应的格子数据
- **移动系统**：检查路径上的格子是否被占用

`MapState` 提供 O(1) 的坐标 → `MapCellData` 映射，避免每次查找都遍历实体树。

## 定义

```rust
/// 地图状态资源，以 [`MapCell`] 为键索引所有格子的 MapCellData。
///
/// 在关卡初始化时由 `spawn_level` 预填充，
/// 提供 O(1) 的格子数据查询。
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct MapState {
    /// 以 MapCell 为键索引的格子数据。
    pub cells: HashMap<MapCell, MapCellData>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `cells` | `HashMap<MapCell, MapCellData>` | 以 `MapCell` 为键，存储格子的角色实体数据 |

## 构建

`MapState` 在 `map` 模块初始化时以空的 `HashMap` 创建。系统 `sync_map_state_on_add` 监听 `MapCell` 组件的添加事件，自动将新增的格子插入 `MapState`：

```rust
pub fn sync_map_state_on_add(
    mut map_state: ResMut<MapState>,
    cells: Query<&MapCell, Added<MapCell>>,
) {
    for cell in &cells {
        map_state.cells.entry(*cell).or_insert_with(MapCellData::default);
    }
}
```

该系统在 `Update` 阶段运行，每个格子仅处理一次（`Added` 过滤器）。无论格子何时生成，`MapState` 都会自动保持同步。

## 使用示例

### 按坐标查询格子数据

```rust
fn get_cell(x: u32, y: u32, map_state: Res<MapState>) -> Option<&MapCellData> {
    map_state.cells.get(&MapCell { x, y })
}
```

### 判断指定坐标是否空闲

```rust
fn is_cell_empty(x: u32, y: u32, map_state: Res<MapState>) -> bool {
    map_state.cells.get(&MapCell { x, y }).map_or(false, |d| d.role.is_none())
}
```

### 遍历交战区所有角色

```rust
fn for_each_combat_role(map_state: Res<MapState>) {
    for (cell, data) in &map_state.cells {
        if cell.x >= 2 {
            if let Some(role_entity) = data.role {
                info!("交战区角色位于 ({}, {})，实体 {:?}", cell.x, cell.y, role_entity);
            }
        }
    }
}
```

## 与相关类型的关系

- **`MapCell`**（[`./MapCell.md`](./MapCell.md)）：地图格子坐标组件。`MapState` 以其为键。
- **`MapCellData`**（[`./MapCellData.md`](./MapCellData.md)）：地图格子上的角色实体组件。`MapState` 存储的就是每个坐标对应的 `MapCellData`。
- **`Map`**（[`./Map.md`](./Map.md)）：地图根实体标记组件。
- **`MapData`**（[`./MapData.md`](./MapData.md)）：地图规格资源。
- **`BenchCell`**（[`./BenchCell.md`](./BenchCell.md)）：备战区格子标记组件，用于区域过滤。

## 设计决策

### Resource 而非 Component

`MapState` 设计为 Resource 而非在实体上存储，原因：

1. **全局访问**：任何系统只需 `Res<MapState>` 即可查询，无需事先获取特定实体引用。
2. **一次性构建**：地图初始化时构建完成，后续只读使用，无需每帧同步。
3. **与 Component 互补**：`MapCellData` 组件提供从实体到数据的访问（ECS 原生），`MapState` 提供从坐标到数据的访问（按需索引），两者相辅相成。

### HashMap 键的选择

使用 `MapCell` 作为键，而非 `(u32, u32)` 元组：

1. **类型安全**：`MapCell` 是有意义的类型名，比裸元组更具可读性。
2. **一致性**：`MapCell` 本身就代表一个格子坐标，键的语义与其类型定义一致。
3. **扩展性**：若未来 `MapCell` 增加字段（如区域 ID），`MapState` 无需改型。

## 相关文档

- [`MapCell.md`](./MapCell.md)：地图格子坐标组件
- [`MapCellData.md`](./MapCellData.md)：地图格子上的角色实体组件
- [`Map.md`](./Map.md)：地图根实体标记组件
- [`MapData.md`](./MapData.md)：地图规格数据资源
- [`BenchCell.md`](./BenchCell.md)：备战区格子标记组件
