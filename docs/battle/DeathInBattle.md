# DeathInBattle

`DeathInBattle` 是一个消息（Message）对象，实现了 Bevy 的 [`Message`] trait，在战斗实体死亡（血量归零）时发出。

## 用途

- 当持有 [`BattleState`] 组件的实体血量降至 0 时，系统可发送 `DeathInBattle` 消息。
- 其他系统可读取该消息来触发死亡后续逻辑，例如：
  - 播放死亡动画或特效。
  - 从场景中移除实体。
  - 产生掉落物或经验值。
  - 更新任务进度或统计数据。

## 定义

```rust
/// 战斗实体死亡时发出的消息。
///
/// 携带已死亡的实体。系统可以读取此消息来触发死亡相关逻辑
/// （例如播放死亡动画、移除实体、掉落物品）。
#[derive(Message, Clone, TypePath)]
pub struct DeathInBattle {
    /// 已死亡的实体。
    pub entity: Entity,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `entity` | `Entity` | 已死亡的战斗实体。 |

## trait 实现

| Trait | 实现说明 |
|-------|----------|
| [`Message`] | 由 `#[derive(Message)]` 提供，使该结构体可作为 Bevy 消息使用。 |
| [`Clone`] | 消息可被克隆，允许多个系统同时读取。 |
| [`TypePath`] | 提供类型路径信息，用于 Bevy 反射系统。 |

## 与现有模块的关系

- **战斗模块**（`battle`）：`DeathInBattle` 定义在 `battle` 模块中，由 [`BattlePlugin`] 通过 `add_message` 注册。
- **战斗状态**（[`BattleState`]）：当 `BattleState::is_dead()` 返回 `true` 时，系统可发送 `DeathInBattle` 消息。
- **角色模块**（`role`）：角色死亡时可通过 `DeathInBattle` 消息触发对应的销毁或重生逻辑。
- **敌人模块**（`enemy`）：敌人死亡时可通过 `DeathInBattle` 消息触发掉落物生成或经验值计算。

[`BattleState`]: ./BattleState.md
[`BattlePlugin`]: ./BattlePlugin.md
[`Message`]: https://docs.rs/bevy/latest/bevy/ecs/message/trait.Message.html
