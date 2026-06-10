# AttackRange

`AttackRange` 是一个组件（Component），用于存储实体当前的**攻击范围**，以像素为单位。

## 用途

- 持有 `AttackRange` 组件的实体拥有一个以自身为中心的圆形攻击区域。
- 攻击范围值表示从实体中心到最远攻击目标之间的最大像素距离。
- 该组件通常配合 `Archer` 等标记组件使用，用于远程单位的攻击判定。

## 定义

```rust
/// 攻击范围（像素）。
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct AttackRange(pub f32);
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `0` | `f32` | 攻击范围的像素值。例如 `300.0` 表示该单位可攻击 300 像素内的目标。 |

## 与现有模块的关系

- **弓箭手模块**：`ArcherRoleBuilder` 在构建实体时为角色附加 `AttackRange` 组件，默认值为 `300.0`。
- **战斗系统**：后续可用于攻击距离检测，判断目标是否在射程内。

## 辅助函数

`common::attack_range` 函数可生成一个攻击范围检测传感器（`Collider::circle` + `Sensor`），用于通过碰撞事件检测进入射程的敌人：

```rust
pub fn attack_range(range: f32, layers: CollisionLayers) -> impl Bundle;
```

参数说明：

| 参数 | 类型 | 说明 |
|---|---|---|
| `range` | `f32` | 攻击范围值，同时是传感器圆形碰撞体的半径。 |
| `layers` | `CollisionLayers` | 物理碰撞层级配置，控制传感器与哪些实体交互。 |

该函数返回的 Bundle 包含 `Name`、`AttackRange`、`Visibility`、`Collider::circle(range)`、`Sensor` 和 `CollisionLayers` 组件。调用方需自行添加 `Transform` 来定位实体。
