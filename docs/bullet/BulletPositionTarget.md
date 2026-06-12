# BulletPositionTarget

`BulletPositionTarget` 是一个组件（Component），用于存储指向 `BulletPosition` 实体的**引用**（`Entity`）。

## 用途

- 持有 `BulletPositionTarget` 组件的实体持有一个 `Entity` 引用，指向场景中携带 `BulletPosition` 组件的目标实体。
- 通过该引用，可以在运行时获取目标实体的世界坐标，用于确定子弹的发射原点或追踪起始位置。
- 该组件通常由 `run_skill` 等系统在生成子弹时附加，将弓箭手的 `BulletPosition` 子实体引用传递给子弹。

## 定义

```rust
/// 指向 BulletPosition 实体的引用。
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct BulletPositionTarget(pub Entity);
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `0` | `Entity` | 指向携带 `BulletPosition` 组件的目标实体的引用。例如 `commands.spawn(BulletPositionTarget(entity))`。 |

## 与现有模块的关系

- **子弹模块**（`bullet`）：子弹实体可通过 `BulletPositionTarget` 引用获取目标 `BulletPosition` 实体的位置信息。
- **弓箭手模块**（`Archer`）：`run_skill` 系统在生成子弹时将弓箭手的 `BulletPosition` 子实体 Entity 存入 `BulletPositionTarget`，传递给子弹。
- **战斗系统**（`battle`）：碰撞检测或伤害系统可通过 `BulletPositionTarget` 追溯子弹的发射源位置。
