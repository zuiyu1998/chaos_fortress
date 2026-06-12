# BulletBattleEvent

`BulletBattleEvent` 是一个消息（Message）对象，实现了 [`bevy_gearbox`] 的 [`Message`] trait，在子弹与其他实体发生碰撞时携带碰撞双方的信息。

## 用途

- 当子弹（持有 [`Bullet`] 组件的实体）与另一个实体碰撞时，`emit_bullet_battle_event` 系统会从 avian2d 的 [`CollisionStart`] 事件中解析出碰撞双方的**刚体实体**（`body1`/`body2`），并发送 `BulletBattleEvent` 消息。
- 战斗系统可通过读取该消息来触发后续逻辑，例如：
  - 对目标实体造成伤害。
  - 在碰撞位置生成特效（命中动画、粒子效果）。
  - 销毁子弹实体。

## 定义

```rust
/// 子弹与其他实体碰撞时发出的消息。
///
/// 携带碰撞中涉及的**刚体实体**（从 [`CollisionStart`] 的 `body1`/`body2` 字段解析而来，
/// 而非碰撞体实体）。系统可以读取此消息来响应子弹命中（例如造成伤害、生成特效、销毁子弹）。
#[derive(Message, Clone, TypePath)]
pub struct BulletBattleEvent {
    /// 碰撞中涉及的子弹刚体实体。
    pub bullet: Entity,
    /// 子弹碰撞到的另一个刚体实体。
    pub other: Entity,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `bullet` | `Entity` | 持有 [`Bullet`] 组件的子弹**刚体实体**（由 `CollisionStart.body1`/`body2` 解析而来）。 |
| `other` | `Entity` | 与子弹发生碰撞的另一方**刚体实体**（例如敌人、地形等）。 |

## emit_bullet_battle_event 系统

`emit_bullet_battle_event` 是发送 `BulletBattleEvent` 消息的系统函数，注册在 `BulletPlugin` 的 `Update` 阶段。

### 功能

监听 avian2d 的碰撞事件，检测到子弹碰撞时发送 `BulletBattleEvent` 消息。

### 实现

```rust
pub fn emit_bullet_battle_event(
    mut started: MessageReader<CollisionStart>,
    bullets: Query<&Bullet>,
    mut writer: MessageWriter<BulletBattleEvent>,
) {
    for event in started.read() {
        let (e1, e2) = match (event.body1, event.body2) {
            (Some(b1), Some(b2)) => (b1, b2),
            _ => continue,
        };
        if bullets.contains(e1) {
            writer.write(BulletBattleEvent {
                bullet: e1,
                other: e2,
            });
        } else if bullets.contains(e2) {
            writer.write(BulletBattleEvent {
                bullet: e2,
                other: e1,
            });
        }
    }
}
```

### 处理流程

1. 通过 `MessageReader<CollisionStart>` 读取 avian2d 的碰撞开始事件。
2. 从事件的 `body1`/`body2` 字段中提取碰撞双方的**刚体实体**。如果任一缺失，跳过该事件。
3. 通过 `Query<&Bullet>` 判断哪一方是子弹实体。
4. 构建 `BulletBattleEvent` 并写入 `MessageWriter<BulletBattleEvent>`，便于其他系统读取处理。

### 注意事项

- 消息中的 `bullet` 和 `other` 字段是 **RigidBody 实体**，而非 Collider 实体。在 Avian 中，碰撞体（Collider）可以是刚体（RigidBody）的子实体，通过 `body1`/`body2` 可以直接获取刚体层级。

## trait 实现

| Trait | 实现说明 |
|-------|----------|
| [`Message`] | 由 `#[derive(Message)]` 提供，使该结构体可作为 bevy_gearbox 消息使用。 |
| [`Clone`] | 消息可被克隆，允许多个系统同时读取。 |
| [`TypePath`] | 提供类型路径信息，用于 bevy 反射系统。 |

## 与现有模块的关系

- **子弹模块**（`bullet`）：`BulletBattleEvent` 由碰撞检测系统在子弹命中时发送。
- **弓箭手模块**（`Archer`）：弓箭手的战斗系统可读取 `BulletBattleEvent` 来处理箭矢命中逻辑。
- **战斗系统**（`battle_system`）：通过监听 `BulletBattleEvent`，战斗系统可以实现伤害计算、特效生成和子弹销毁。

[`Bullet`]: ./Bullet.md
[`BulletPlugin`]: ./BulletPlugin.md
[`Message`]: https://docs.rs/bevy_gearbox/latest/bevy_gearbox/message/trait.Message.html
[`bevy_gearbox`]: https://docs.rs/bevy_gearbox
[`CollisionStart`]: https://docs.rs/avian2d/latest/avian2d/collision/collision_events/struct.CollisionStart.html
