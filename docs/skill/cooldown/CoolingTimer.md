# CoolingTimer

`CoolingTimer` 是一个组件（Component），用于存储实体当前的**冷却计时器**，基于 Bevy 的 `Timer`。组件包含一个开关字段，控制计时器是否倒计时。

## 用途

- 持有 `CoolingTimer` 组件的实体拥有一个持续倒计时的冷却计时器，用于限制技能的释放频率。
- 每次触发技能（如弓箭手攻击）后，计时器重置为对应间隔值，随后逐帧递减至归零，归零后方可再次执行。
- 当 `enabled` 字段为 `false` 时，系统跳过该计时器的更新，可用于暂停冷却或实现条件性的冷却机制。
- 该组件通常配合 `AttackSpeed` 等属性组件使用，`AttackSpeed` 决定冷却总时长，`CoolingTimer` 封装 Bevy 的 `Timer` 进行计时管理。

## 定义

```rust
/// 冷却计时器。
///
/// 当 `enabled` 为 `false` 时计时器不会推进。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CoolingTimer {
    /// 底层的 Bevy 计时器。
    pub timer: Timer,
    /// 是否启用倒计时。
    /// 仅在 `true` 时系统才会推进该计时器。
    pub enabled: bool,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `timer` | `Timer` | Bevy 内置计时器，支持 `TimerMode::Once`（单次）和 `TimerMode::Repeating`（循环）。冷却场景推荐使用 `TimerMode::Once`。 |
| `enabled` | `bool` | 是否启用倒计时。为 `true` 时系统每帧推进计时器；为 `false` 时跳过更新。初始值为 `true`。 |

## 生命周期

```
技能构建 (CooldownFeatureBuilder)
    │  timer = Timer::from_seconds(duration, Once)
    │  enabled = true
    ▼
tick_cooldown_timer 每帧执行
    │  if enabled: timer.tick(delta)
    │  if just_finished: 标记 feature 完成, enabled = false
    ▼
技能完成 (SkillEvent 发出)
    │  reset_cooldown_timer: timer.reset(), enabled = true
    ▼
等待下一次冷却开始
```

## `enabled` 使用场景

- **默认启用**：`CooldownFeatureBuilder` 在构建时将 `enabled` 设为 `true`，计时器立即开始倒计时。
- **自动禁用**：计时器归零（`just_finished()`）后，系统自动将 `enabled` 设为 `false`，停止后续帧的无意义推进。
- **暂停冷却**：外部系统可将 `enabled` 设为 `false`，计时器暂停推进（例如角色被眩晕或技能被打断时）。
- **恢复冷却**：将 `enabled` 重新设为 `true`，计时器从暂停位置继续倒计时。
- **技能完成时**：`reset_cooldown_timer` 在收到 [`SkillEvent`] 后重置计时器并自动将 `enabled` 恢复为 `true`，为下一次冷却做准备。

## 与现有模块的关系

- **弓箭手模块**：`ArcherRoleBuilder` 在构建实体时附加 `AttackSpeed` 属性组件（攻击间隔），战斗系统据此创建 `Timer::from_seconds(speed, TimerMode::Once)` 并存入 `CoolingTimer`。
- **战斗系统**：每帧遍历所有携带 `CoolingTimer` 的实体，检查 `enabled` 为 `true` 后调用 `timer.tick(time.delta())` 推进计时；当 `timer.just_finished()` 时允许实体执行下一次攻击。
