# EnemyTargetList

`EnemyTargetList` 是一个组件（Component），用于存储当前处于**攻击范围内所有敌人实体**的列表。

## 设计动机

单个 `EnemyTarget` 只能存储一个目标，但攻击范围内可能有多个敌人。`EnemyTargetList` 通过维护一个完整的敌人列表，使得系统可以灵活地选择目标（如最近者、血量最低者等），或用于范围伤害判定。

## 定义

```rust
/// 攻击范围内的所有敌人实体。
///
/// 由碰撞事件维护：`CollisionStarted` 时添加，
/// `CollisionEnded` 时移除。系统可遍历此列表选择目标。
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyTargetList(pub Vec<Entity>);
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `0` | `Vec<Entity>` | 当前攻击范围内所有敌人实体的列表。为空时表示范围内无敌人。 |

## 使用示例

### 初始化

通常在角色构建时附加空列表：

```rust
commands.spawn((
    Role,
    Archer,
    EnemyTargetList(Vec::new()),
    // ... 其他组件 ...
));
```

### 在碰撞事件中维护列表

```rust
fn update_enemy_target_list(
    mut started: EventReader<CollisionStarted>,
    mut ended: EventReader<CollisionEnded>,
    mut lists: Query<&mut EnemyTargetList>,
    enemies: Query<&Enemy>,
) {
    for CollisionStarted(e1, e2) in started.read() {
        if let Ok(mut list) = lists.get_mut(*e1) {
            if enemies.contains(*e2) {
                list.0.push(*e2);
            }
        }
    }
    for CollisionEnded(e1, e2) in ended.read() {
        if let Ok(mut list) = lists.get_mut(*e1) {
            list.0.retain(|e| e != e2);
        }
    }
}
```

### 选择目标

基于列表选择最优目标（如最近者）：

```rust
fn select_closest_target(
    archers: Query<(&EnemyTargetList, &GlobalTransform), With<Archer>>,
    enemy_positions: Query<&GlobalTransform, With<Enemy>>,
) {
    for (list, archer_transform) in &archers {
        let archer_pos = archer_transform.translation().truncate();

        let closest = list.0
            .iter()
            .filter_map(|e| enemy_positions.get(*e).ok())
            .min_by(|a, b| {
                let da = a.translation().truncate().distance_squared(archer_pos);
                let db = b.translation().truncate().distance_squared(archer_pos);
                da.total_cmp(&db)
            });

        if let Some(target) = closest {
            // 向最近敌人攻击
        }
    }
}
```

### 配合 EnemyTarget 使用

先更新列表，再从列表中选择主目标写入 `EnemyTarget`：

```rust
fn sync_primary_target(
    mut archers: Query<(&EnemyTargetList, &mut EnemyTarget)>,
) {
    for (list, mut primary) in &mut archers {
        primary.0 = list.0.first().copied();
    }
}
```

## 与现有模块的关系

| 模块 | 关系 |
|---|---|
| [`EnemyTarget`](EnemyTarget.md) | `EnemyTargetList` 提供完整的候选列表，`EnemyTarget` 从中选出主目标 |
| `AttackRange` | `AttackRange` 传感器通过碰撞事件驱动列表的更新 |
| `Archer` | 弓箭手实体同时携带 `EnemyTargetList` 和 `EnemyTarget` |

## 设计说明

- 使用 `Vec<Entity>` 存储所有范围内的敌人，容量动态增长。
- 列表的添加/移除由碰撞事件驱动的外部系统维护，组件本身仅作为数据容器。
- 与 `EnemyTarget` 的区别：`EnemyTargetList` 保存完整候选集，`EnemyTarget` 保存当前选中的单一目标。
