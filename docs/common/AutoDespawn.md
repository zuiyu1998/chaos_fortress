# AutoDespawn

`AutoDespawn` 是一个标记组件（Marker Component），用于标识需要在特定条件满足后自动销毁的实体。

## 用途

- 持有 `AutoDespawn` 组件的实体会在条件达成时（例如动画播放完毕、计时器到期、粒子效果结束）被自动回收。
- 通常配合 `bevy_tween` 的动画组件或自定义计时系统使用，在动画结束时触发销毁。
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
// 生成一个带有自动销毁标记的伤害数字
commands.spawn((
    Name::new("DamageNumber (-42)"),
    DamageNumber,
    AutoDespawn,
    Text2d::new("-42"),
    TextFont {
        font: asset_server.load("fonts/semibold.ttf"),
        font_size: 64.0,
        ..default()
    },
    TextColor(Color::srgb(1.0, 0.2, 0.2)),
    Transform::from_xyz(pos_x, pos_y, 10.0),
));
```

对应的销毁系统示例：

```rust
fn despawn_after_tween(
    mut commands: Commands,
    query: Query<Entity, (With<AutoDespawn>, Without<bevy_tween::tween::TweenInterpolationValue>)>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
```

## 与现有模块的关系

- **common 模块**：`AutoDespawn` 作为通用标记组件，可附加到任何需要自动销毁的实体上。
- **战斗系统**：伤害数字等临时视觉效果可在动画完成后触发销毁。
- **生命周期管理**：配合 `bevy_tween` 的 `TweenInterpolationValue` 组件是否存在来判断动画是否仍在运行；动画结束后执行清理。
