# CoolingTimer

`CoolingTimer` 是一个组件（Component），用于存储实体当前的**冷却计时器**，基于 Bevy 的 `Timer`。

## 用途

- 持有 `CoolingTimer` 组件的实体拥有一个持续倒计时的冷却计时器，用于限制技能的释放频率。
- 每次触发技能（如弓箭手攻击）后，计时器重置为对应间隔值，随后逐帧递减至归零，归零后方可再次执行。
- 该组件通常配合 `AttackSpeed` 等属性组件使用，`AttackSpeed` 决定冷却总时长，`CoolingTimer` 封装 Bevy 的 `Timer` 进行计时管理。

## 定义

```rust
/// 冷却计时器。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CoolingTimer(pub Timer);
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `0` | `Timer` | Bevy 内置计时器，支持 `TimerMode::Once`（单次）和 `TimerMode::Repeating`（循环）。冷却场景推荐使用 `TimerMode::Once`。 |

## 辅助函数

`CoolingTimer::tick_all` 是一个系统函数，用于每帧推进所有携带 `CoolingTimer` 组件的倒计时：

```rust
pub fn tick_all(time: Res<Time>, mut query: Query<&mut CoolingTimer>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}
```

## 与现有模块的关系

- **弓箭手模块**：`ArcherRoleBuilder` 在构建实体时附加 `AttackSpeed` 属性组件（攻击间隔），战斗系统据此创建 `Timer::from_seconds(speed, TimerMode::Once)` 并存入 `CoolingTimer`。
- **战斗系统**：每帧遍历所有携带 `CoolingTimer` 的实体，调用 `timer.0.tick(time.delta())` 推进计时；当 `timer.0.just_finished()` 时允许实体执行下一次攻击。
