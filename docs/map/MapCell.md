# MapCell

`MapCell` 是一个**组件（Component）**，用于标记一个实体为地图格子，并记录其在棋盘格上的网格坐标 `(x, y)`。

## 设计动机

在地图系统中，每个格子对应棋盘上的一个位置（12 列 × 5 行）。许多系统需要根据格子实体的坐标查询或过滤：

- **地图初始化系统**：在生成格子时需要知道每个格子实体对应的列和行
- **交互系统**：点击格子时，需要知道点击的是哪个坐标位置
- **战斗系统**：根据角色所在的格子坐标计算攻击范围、移动路径
- **UI 系统**：高亮特定行列的格子

将坐标直接编码为组件，让系统通过 `Query<&MapCell>` 即可获取位置，无需从 `Transform` 或 `Name` 字符串中反推。

## 定义

```rust
/// 地图格子组件，记录网格坐标 (x, y)。
///
/// x 为列（0~11），y 为行（0~4）。
/// 每个 `Map` 实体下的格子子实体都携带此组件。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct MapCell {
    /// 列坐标（0~11）。
    pub x: u32,
    /// 行坐标（0~4）。
    pub y: u32,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `x` | `u32` | 列坐标（0 ~ 11），从地图左上角起算 |
| `y` | `u32` | 行坐标（0 ~ 4），从地图左上角起算 |

## 坐标约定

- `(0, 0)` 为地图左上角的格子。
- `x` 向右递增，`y` 向下递增。
- `(11, 4)` 为地图右下角的格子。
- 列 0~1 属于**备战区**，列 2~11 属于**交战区**。

## 使用示例

### 查询所有格子坐标

```rust
fn print_all_cells(cells: Query<&MapCell>) {
    for cell in &cells {
        info!("格子位于 ({}, {})", cell.x, cell.y);
    }
}
```

### 根据坐标过滤

```rust
fn find_cell_at(x: u32, y: u32, cells: Query<&MapCell>) -> Option<Entity> {
    for (entity, cell) in cells.iter_with_entities() {
        if cell.x == x && cell.y == y {
            return Some(entity);
        }
    }
    None
}
```

### 判断区域

```rust
fn is_bench_zone(cell: &MapCell) -> bool {
    cell.x <= 1
}

fn is_combat_zone(cell: &MapCell) -> bool {
    cell.x >= 2
}
```

## 与相关类型的关系

- **`Map`**（[`./Map.md`](./Map.md)）：地图根实体标记组件。`MapCell` 是 `Map` 的子实体上的组件。
- **`BenchCell`**（[`./BenchCell.md`](./BenchCell.md)）：备战区格子标记组件，标识格子所属区域。
- **`MapCellData`**（[`./MapCellData.md`](./MapCellData.md)）：地图格子的角色实体数据，存储于 `MapState` 资源中。
- **`MapData`**（[`./MapData.md`](./MapData.md)）：地图规格资源，定义 `width`（12）和 `height`（5），决定了 `MapCell.x` 和 `MapCell.y` 的取值范围。
- **`MapSystem`**（[`./MapSystem.md`](./MapSystem.md)）：地图系统文档，提供了格子坐标与像素坐标互转的工具函数。

## 设计决策

### 存储网格坐标而非像素坐标

`MapCell` 只存储逻辑网格坐标 `(x, y)`，像素坐标由 `cell_size`（来自 `MapData`）计算得出。这样做的原因：

1. **逻辑与渲染分离**：网格坐标是"在哪一格"，像素坐标是"画在哪"。系统关心的是前者。
2. **计算简单**：`pixel_x = cell.x * MapData.cell_size` 即可得到像素位置，无需存储冗余信息。
3. **与地图数据对齐**：地形数据、角色位置等都以网格坐标表示，保持一致。

### 使用 Component 而非 Resource 存储

每个格子实体携带一个 `MapCell` 组件，而非集中存储在某个 Resource 的 `HashMap<Entity, (u32, u32)>` 中。原因：

1. **天然关联**：组件本身就是实体到数据的映射，无需额外的索引结构。
2. **查询高效**：`Query<&MapCell>` 遍历所有格子是 Bevy 原生的 ECS 模式，性能优秀。
3. **组合灵活**：可以与其他组件组合查询（如 `Query<(&MapCell, &Sprite), With<Terrain>>`）。

## 相关文档

- [`MapCellData.md`](./MapCellData.md)：地图格子上的角色实体组件
- [`Map.md`](./Map.md)：地图根实体标记组件
- [`MapData.md`](./MapData.md)：地图规格数据资源
- [`MapSystem.md`](./MapSystem.md)：地图系统文档（布局、工具函数、地形系统）
