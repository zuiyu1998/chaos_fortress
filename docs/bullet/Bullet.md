# Bullet

`Bullet` 是一个标记组件（Marker Component），用于标识游戏中的**子弹**实体。

## 用途

- 持有 `Bullet` 组件的实体代表一颗已发射的子弹（如弓箭手的箭矢、法术弹等）。
- 在碰撞检测或伤害系统中，可通过查询 `Bullet` 组件来区分子弹与其他实体，从而触发对应的命中逻辑（如击中敌人后造成伤害并销毁自身）。
- 该组件通常与 `Transform`、`Velocity` 等组件配合使用，实现子弹的飞行与碰撞。

## 定义

```rust
/// 标记实体为子弹。
#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Bullet;
```

## 与现有模块的关系

- **弓箭手模块**（`Archer`）：弓箭手射出的箭矢会附加 `Bullet` 组件，标记其为可飞行碰撞的子弹实体。
- **战斗系统**（`battle_system`）：碰撞检测系统可根据 `Bullet` 组件识别子弹，执行伤害计算。
- **敌人模块**（`enemy`）：子弹与敌人碰撞后，触发敌人受伤逻辑。
