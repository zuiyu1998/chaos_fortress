# BenchCell

`BenchCell` 是一个**标记组件（Marker Component）**，用于标识一个地图格子属于备战区（Bench Zone）。

## 设计动机

地图的 12 列格子被划分为两个区域：备战区（列 0~1）和交战区（列 2~11）。多个系统需要区分格子所属的区域：

- **备战区系统**：仅在备战区格子上执行角色部署、交换等操作
- **战斗系统**：确定战斗逻辑是否适用于备战区角色
- **UI 系统**：为备战区和交战区渲染不同的视觉效果
- **移动系统**：限制角色只能部署到备战区或从备战区移动到交战区

使用显式的 `BenchCell` 标记组件，系统可通过 `Query<&BenchCell>` 或 `Without<BenchCell>` 快速过滤，无需运行时计算 `cell.x <= 1`。

## 定义

```rust
/// 标记组件，表示一个格子属于备战区（列 0~1）。
///
/// 仅备战区格子携带此组件，交战区格子不携带。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct BenchCell;
```

## 使用示例

### 查询所有备战区格子

```rust
fn query_bench_cells(cells: Query<(&MapCell, &MapCellData), With<BenchCell>>) {
    for (cell, data) in &cells {
        info!("备战区格子 ({}, {})", cell.x, cell.y);
    }
}
```

### 查询所有交战区格子（非备战区）

```rust
fn query_combat_cells(cells: Query<(&MapCell, &MapCellData), Without<BenchCell>>) {
    for (cell, data) in &cells {
        info!("交战区格子 ({}, {})", cell.x, cell.y);
    }
}
```

### 判断角色是否在备战区

```rust
fn is_role_on_bench(role_entity: Entity, cells: Query<(&MapCellData, Option<&BenchCell>)>) -> bool {
    for (data, bench) in &cells {
        if data.role == Some(role_entity) {
            return bench.is_some();
        }
    }
    false
}
```

## 与相关类型的关系

- **`MapCell`**（[`./MapCell.md`](./MapCell.md)）：地图格子坐标组件。`BenchCell` 作用于 `MapCell` 实体之上。
- **`MapCellData`**（[`./MapCellData.md`](./MapCellData.md)）：地图格子上的角色实体组件。`BenchCell` 与其同属一个实体。
- **`Map`**（[`./Map.md`](./Map.md)）：地图根实体标记组件。
- **`MapData`**（[`./MapData.md`](./MapData.md)）：地图规格资源。

## 设计决策

### 使用标记组件而非查询坐标

`BenchCell` 是一个显式的标记组件，与通过 `MapCell.x <= 1` 判断区域相比：

1. **查询效率更高**：ECS 通过组件 ID 位集过滤，比逐个读取 `x` 字段再做比较更快。
2. **语义清晰**：`With<BenchCell>` 比 `cell.x <= 1` 更直观地表达了"这是备战区格子"的意图。
3. **组合灵活**：可与 `Changed<MapCellData>` 等过滤条件组合，让系统仅在备战区格子发生变化时响应。

### 使用标记而非枚举

不使用 `enum Zone { Bench, Combat }` 作为组件字段，而是采用两个独立标记组件（目前仅有 `BenchCell`，未来可添加 `CombatCell`）：

1. **添加零开销**：标记组件不占用额外内存，仅为位集中一个 bit。
2. **可组合查询**：`With<BenchCell>` / `Without<BenchCell>` 直接在 ECS 查询层过滤。
3. **可扩展**：未来可添加 `CombatCell` 等标记，共存于同一实体不会冲突。

## 相关文档

- [`MapCell.md`](./MapCell.md)：地图格子坐标组件
- [`MapCellData.md`](./MapCellData.md)：地图格子上的角色实体组件
- [`Map.md`](./Map.md)：地图根实体标记组件
- [`MapData.md`](./MapData.md)：地图规格数据资源
