# BulletPlugin

`BulletPlugin` 是一个插件对象，实现了 Bevy 的 [`Plugin`] trait，负责注册子弹模块相关的组件。

## 用途

- 向 Bevy 应用注册 [`Bullet`], [`BulletPosition`], [`BulletPositionTarget`] 三个组件到类型反射系统（Type Registry）。
- 通过 `add_message` 注册 [`BulletBattleEvent`] 消息。
- 添加两个 `Update` 阶段系统：
  - [`emit_bullet_battle_event`]：监听碰撞事件并发送 [`BulletBattleEvent`] 消息。
  - [`despawn_on_hit`]：读取 [`BulletBattleEvent`] 消息，销毁子弹实体。

## 定义

```rust
/// 注册子弹相关组件的插件。
pub(super) struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>();
        app.register_type::<BulletPosition>();
        app.register_type::<BulletPositionTarget>();
        app.add_message::<BulletBattleEvent>();

        app.add_systems(Update, (emit_bullet_battle_event, despawn_on_hit));
    }
}
```

## 注册的组件

| 组件 | 说明 |
|------|------|
| [`Bullet`] | 标记实体为子弹（项目符号）。 |
| [`BulletPosition`] | 标记子弹已记录起始位置。 |
| [`BulletPositionTarget`] | 存储指向 `BulletPosition` 实体的引用，用于追溯子弹发射原点。 |

## 注册的消息

| 消息 | 说明 |
|------|------|
| [`BulletBattleEvent`] | 子弹碰撞时发出的消息，携带碰撞双方的刚体实体。通过 `add_message` 注册以便 bevy 事件系统处理。 |

## 注册的系统

| 系统 | 阶段 | 说明 |
|------|------|------|
| [`emit_bullet_battle_event`] | `Update` | 监听 avian2d 碰撞开始事件，检测子弹碰撞并发送 [`BulletBattleEvent`] 消息。 |
| [`despawn_on_hit`] | `Update` | 读取 [`BulletBattleEvent`] 消息，将 `bullet` 字段指向的子弹实体销毁。 |

### despawn_on_hit

`despawn_on_hit` 是一个简单的默认系统，每当子弹碰撞后立即销毁子弹实体。

```rust
pub fn despawn_on_hit(
    mut events: MessageReader<BulletBattleEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands.entity(event.bullet).despawn();
    }
}
```

该系统提供了一种基础的行为模式。更复杂的系统（伤害计算、血量管理、穿透逻辑）可以在此基础上替换或补充。

## 与现有模块的关系

- **子弹模块**（`bullet`）：`BulletPlugin` 是子弹模块的入口插件，由主应用（`AppPlugin`）的插件列表添加。
- **主应用**（`main`）：在 `src/main.rs` 中以 `bullet::BulletPlugin` 的形式被添加至 Bevy 应用。
- **战斗系统**（`battle`）：其他系统可通过读取 [`BulletBattleEvent`] 消息来实现自定义的命中逻辑，而无需修改 `BulletPlugin`。

[`Bullet`]: ./Bullet.md
[`BulletPosition`]: ./BulletPosition.md
[`BulletPositionTarget`]: ./BulletPositionTarget.md
[`BulletBattleEvent`]: ./BulletBattleEvent.md
[`emit_bullet_battle_event`]: ./BulletBattleEvent.md#emit_bullet_battle_event-系统
[`Plugin`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html
