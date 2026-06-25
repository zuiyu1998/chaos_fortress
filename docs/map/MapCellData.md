# MapCellData

`MapCellData` 是一个**组件（Component）**，用于存储 `MapCell` 之上停留的角色实体（Role）。角色实体可能存在，也可能不存在。

## 设计动机

每个地图格子可能站着一个角色（敌方或我方）。在游戏运行过程中，多个系统需要查询某个格子上的角色：

- **战斗系统**：获取指定格子上的角色进行攻击判定
- **移动系统**：检查目标格子是否被占用
- **UI 系统**：根据格子上的角色显示选中框或血条
- **备战区系统**：查找空闲格子以放置新购买的角色

将角色实体直接作为组件存储在格子实体上，让系统通过 `Query<&MapCellData>` 即可获取，无需额外的全局索引。

## 定义

```rust
/// 地图格子上的角色实体组件。
///
/// `role` 为 `Some(entity)` 表示该格子上站着一个角色，
/// 为 `None` 表示格子空闲。
/// 此组件由格子初始化系统自动添加，默认值为 `None`。
#[derive(Component, Debug, Clone, Copy, Reflect)]
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

### 查找空闲格子

```rust
fn find_empty_bench_slot(cells: Query<(Entity, &MapCell, &MapCellData)>) -> Option<Entity> {
    for (entity, cell, data) in &cells {
        if cell.x <= 1 && data.role.is_none() {
            return Some(entity);
        }
    }
    None
}
```

### 放置角色到格子

```rust
fn place_role_on_cell(
    mut commands: Commands,
    cell_entity: Entity,
    role_entity: Entity,
) {
    commands.entity(cell_entity).insert(MapCellData {
        role: Some(role_entity),
    });
}
```

### 移走格子上的角色

```rust
fn clear_cell(mut commands: Commands, cell_entity: Entity) {
    commands.entity(cell_entity).insert(MapCellData { role: None });
}
```

### 获取格子上角色的组件

```rust
fn get_role_health(
    cell_query: Query<&MapCellData>,
    role_query: Query<&Health>,
) -> Option<f32> {
    let data = cell_query.single();
    if let Some(role_entity) = data.role {
        role_query.get(role_entity).ok().map(|h| h.value)
    } else {
        None
    }
}
```

## 与相关类型的关系

- **`MapCell`**（[`./MapCell.md`](./MapCell.md)）：地图格子坐标组件。`MapCell` 记录格子坐标，`MapCellData` 记录格子上的角色，两者配合使用。
- **`Map`**（[`./Map.md`](./Map.md)）：地图根实体标记组件。`MapCellData` 是 `Map` 的子实体上的组件。
- **`MapData`**（[`./MapData.md`](./MapData.md)）：地图规格资源。

## 设计决策

### 使用 Option<Entity> 而非 Entity

`role` 字段为 `Option<Entity>` 而非直接 `Entity`，原因：

1. **表示空闲状态**：`None` 明确表示格子未被占用，无需哨兵值。
2. **安全**：访问时强制开发者处理可能为空的情况，避免使用无效实体。
3. **与 ECS 模式一致**：`Option<Entity>` 是 Bevy 中表示可选实体引用的惯用方式。

### 使用 Component 而非 Resource

将角色关系存储在格子实体的组件上，而非全局 Resource 的 HashMap：

1. **直接查询**：通过 `Query<&MapCellData>` 可直接获取，无需经过 Resource 间接查找。
2. **组合查询**：可与 `MapCell` 组合为 `Query<(&MapCell, &MapCellData)>`，同时获取坐标和角色。
3. **天然生命周期**：格子实体销毁时，组件自动清理，无需手动维护索引的一致性。

## 相关文档

- [`MapCell.md`](./MapCell.md)：地图格子坐标组件
- [`Map.md`](./Map.md)：地图根实体标记组件
- [`MapData.md`](./MapData.md)：地图规格数据资源
