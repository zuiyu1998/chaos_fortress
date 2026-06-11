# EnemyTarget

`EnemyTarget` 是一个组件（Component），用于存储实体当前锁定的**敌人实体**，可以为空。

## 设计动机

当弓箭手通过攻击范围传感器检测到敌人进入射程后，需要将目标敌人记录下来供后续系统使用（如瞄准、发射投射物）。`EnemyTarget` 提供了一种统一的方式在当前角色实体上存储目标引用，无需每次重新查询碰撞事件。

## 定义

```rust
/// 存储当前锁定的敌人实体。
///
/// 当值为 `None` 时表示没有目标；值为 `Some(Entity)` 时
/// 表示已锁定一个敌人。
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyTarget(pub Option<Entity>);
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `0` | `Option<Entity>` | 当前锁定的敌人实体。`None` 表示空闲/无目标。 |

## 使用示例

### 初始化

通常在角色构建时附加默认值（无目标）：

```rust
commands.spawn((
    Role,
    Archer,
    EnemyTarget(None),
    // ... 其他组件 ...
));
```

### 在碰撞事件中设置目标

当攻击范围传感器检测到敌人进入时，通过 `CollisionStarted` 事件更新目标：

```rust
fn on_enemy_entered_range(
    mut commands: Commands,
    mut events: EventReader<CollisionStarted>,
    mut archers: Query<&mut EnemyTarget, With<Archer>>,
    enemies: Query<Entity, With<Enemy>>,
) {
    for CollisionStarted(e1, e2) in events.read() {
        let sensor_entity = /* 从碰撞对中识别传感器实体 */;
        let enemy_entity = /* 从碰撞对中识别敌人实体 */;

        if let Ok(mut target) = archers.get_mut(sensor_entity) {
            target.0 = Some(enemy_entity);
        }
    }
}
```

### 在战斗中读取目标

```rust
fn fire_at_target(
    archers: Query<&EnemyTarget, With<Archer>>,
) {
    for target in &archers {
        if let Some(enemy) = target.0 {
            // 向 enemy 发射投射物
        }
    }
}
```

### 清除目标

当敌人离开射程或被消灭时，将目标重置为 `None`：

```rust
target.0 = None;
```

## 与现有模块的关系

| 模块 | 关系 |
|---|---|
| `AttackRange` | `AttackRange` 传感器通过碰撞事件检测目标，`EnemyTarget` 存储检测结果 |
| `Archer` | 弓箭手实体携带 `EnemyTarget`，在战斗系统中读取目标信息 |
| `CoolingTimer` | 攻击冷却与目标锁定配合使用，冷却结束时向锁定目标开火 |

## 设计说明

- 使用 `Option<Entity>` 而非直接存储 `Entity`，使得"没有目标"的状态显式可表达。
- 组件而非资源，每个角色实体独立持有自己的目标。
- 目标的生命周期由外部系统管理（碰撞事件、敌人死亡事件等），组件本身仅作为数据容器。
