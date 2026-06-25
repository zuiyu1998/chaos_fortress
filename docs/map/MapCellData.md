# MapCellData

`MapCellData` 是一个**普通数据结构**，用于存储格子上的角色实体（Role）。它作为 `MapState` 资源中 `HashMap<MapCell, MapCellData>` 的值存在，而非独立的 ECS 组件。

## 设计动机

每个地图格子可能站着一个角色（敌方或我方）。`MapCellData` 记录每个格子对应的角色实体，集中存储在 `MapState` 中，以便按坐标快速查询。

## 定义

```rust
/// 地图格子上的角色实体数据。
///
/// `role` 为 `Some(entity)` 表示该格子上站着一个角色，
/// 为 `None` 表示格子空闲。
#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct MapCellData {
    /// 停留在此格子上的角色实体（如有）。
    pub role: Option<Entity>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `role` | `Option<Entity>` | 当前格子上的角色实体。没有角色时为 `None` |

## 使用示例

### 通过 MapState 查找空闲格子

```rust
fn find_empty_bench_slot(map_state: Res<MapState>) -> Option<MapCell> {
    for (cell, data) in &map_state.cells {
        if cell.x <= 1 && data.role.is_none() {
            return Some(*cell);
        }
    }
    None
}
```

### 放置角色到格子

```rust
fn place_role_on_cell(
    mut map_state: ResMut<MapState>,
    cell_key: MapCell,
    role_entity: Entity,
) {
    if let Some(data) = map_state.cells.get_mut(&cell_key) {
        data.role = Some(role_entity);
    }
}
```

### 移走格子上的角色

```rust
fn clear_cell(mut map_state: ResMut<MapState>, cell_key: MapCell) {
    if let Some(data) = map_state.cells.get_mut(&cell_key) {
        data.role = None;
    }
}
```

### 获取格子上角色的组件

```rust
fn get_role_health(
    map_state: Res<MapState>,
    role_query: Query<&Health>,
    cell_key: MapCell,
) -> Option<f32> {
    let data = map_state.cells.get(&cell_key)?;
    let role_entity = data.role?;
    role_query.get(role_entity).ok().map(|h| h.value)
}
```

## 与相关类型的关系

- **`MapCell`**（[`./MapCell.md`](./MapCell.md)）：地图格子坐标类型，作为 `MapState` 中 `HashMap` 的键。`MapCellData` 是与之对应的值。
- **`MapState`**（[`./MapState.md`](./MapState.md)）：地图状态资源，`MapCellData` 作为其 `cells: HashMap<MapCell, MapCellData>` 的值存在。
- **`Map`**（[`./Map.md`](./Map.md)）：地图根实体标记组件。
- **`MapData`**（[`./MapData.md`](./MapData.md)）：地图规格资源。

## 设计决策

### 使用 Option<Entity> 而非 Entity

`role` 字段为 `Option<Entity>` 而非直接 `Entity`，原因：

1. **表示空闲状态**：`None` 明确表示格子未被占用，无需哨兵值。
2. **安全**：访问时强制开发者处理可能为空的情况，避免使用无效实体。
3. **与 Rust 模式一致**：`Option<Entity>` 是 Rust 中表示可选引用的惯用方式。

### 非组件设计

`MapCellData` 不是 ECS 组件，而是 `MapState` 资源中的普通数据结构：

1. **集中索引**：存储在 `HashMap<MapCell, MapCellData>` 中，按坐标 O(1) 查询，无需遍历实体树。
2. **数据与渲染分离**：格子实体的职责是渲染和交互，角色归属信息由 `MapState` 统一管理。
3. **配合 `MapState`**：`MapState` 是全局只读资源，任何系统只需 `Res<MapState>` 即可查询格子状态。

## 相关文档

- [`MapCell.md`](./MapCell.md)：地图格子坐标类型
- [`MapState.md`](./MapState.md)：地图状态资源
- [`Map.md`](./Map.md)：地图根实体标记组件
- [`MapData.md`](./MapData.md)：地图规格数据资源
