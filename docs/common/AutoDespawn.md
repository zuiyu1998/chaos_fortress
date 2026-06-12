# AutoDespawn

`AutoDespawn` 是一个标记组件（Marker Component），用于标识需要在特定条件满足后自动销毁的实体。

## 用途

- 持有 `AutoDespawn` 组件的实体会在条件达成时（例如动画播放完毕）被自动回收。
- 配合 `bevy_tweening` 的 `TweenAnim` + `AnimCompletedEvent` 使用，在动画结束时触发销毁。
- 便于系统集中管理临时实体的生命周期，避免手动追踪和清理。

## 定义

```rust
/// 自动销毁标记。
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct AutoDespawn;
```

## 注册

在 `CommonPlugin` 中注册类型以实现反射：

```rust
app.register_type::<AutoDespawn>();
```

## 使用示例

```rust
// 生成一个带有自动销毁标记的伤害数字（推荐使用工厂函数）
commands.spawn(common::damage_number(
    -42,
    pos_x,
    pos_y,
    asset_server.load("fonts/semibold.ttf"),
));
```

`damage_number` 函数内部自动附加了 `AutoDespawn` 组件以及 `bevy_tweening` 的 `TweenAnim` 动画组件。

对应的销毁系统（注册在 `CommonPlugin` 的 `PostUpdate` 调度中）：

```rust
fn despawn_on_tween_complete(
    mut commands: Commands,
    mut events: MessageReader<AnimCompletedEvent>,
    query: Query<&AutoDespawn>,
) {
    for event in events.read() {
        if query.contains(event.anim_entity) {
            commands.entity(event.anim_entity).despawn();
        }
    }
}
```

## 与现有模块的关系

- **common 模块**：`AutoDespawn` 作为通用标记组件，自动附加在 `damage_number` 工厂函数生成的 Bundle 中。
- **战斗系统**：`AnimCompletedEvent` 由 `TweenAnim` 在动画完成时自动发射，`despawn_on_tween_complete` 系统监听该事件并销毁实体。
- **生命周期管理**：配合 `bevy_tweening` 的 `AnimCompletedEvent` 事件机制，动画结束后可靠触发清理。
