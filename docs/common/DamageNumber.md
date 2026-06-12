# DamageNumber

`DamageNumber` 是一个标记组件（Marker Component），用于标识伤害数字实体。

## 用途

- 持有 `DamageNumber` 组件的实体代表一个**飘浮伤害数字**，在战斗中向玩家展示造成的伤害值。
- 由 `damage_number` 工厂函数生成，包含 `AutoDespawn` 标记和 `bevy_tweening` 的浮空动画。
- 动画结束时自动触发 `AnimCompletedEvent`，由 `despawn_on_tween_complete` 系统清理。

## 定义

```rust
/// 伤害数字标记。
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct DamageNumber;
```

## 注册

在 `CommonPlugin` 中注册类型以实现反射：

```rust
app.register_type::<DamageNumber>();
```

## 使用示例

```rust
// 生成一个伤害数字（世界空间）
commands.spawn(common::damage_number(
    -42,                                // 伤害数值
    pos_x,                               // 世界坐标 X
    pos_y,                               // 世界坐标 Y
    asset_server.load("fonts/semibold.ttf"),
));
```

`damage_number` 函数内部返回的 Bundle 包含：

| 组件 | 说明 |
|---|---|
| `Name` | 调试命名，如 `DamageNumber (-42)` |
| `DamageNumber` | 标记组件 |
| `AutoDespawn` | 动画完成后自动销毁 |
| `Text2d` + `TextFont` + `TextColor` | 红色文字显示伤害值 |
| `Transform` | 起始位置 |
| `TweenAnim` | `bevy_tweening` 动画：1 秒内向上移动 80 像素（`QuadraticOut` 缓出） |

## 与现有模块的关系

- **common 模块**：由 `common::damage_number()` 工厂函数生成，自动附加 `AutoDespawn` 标记。
- **战斗系统**：`apply_bullet_damage` 在造成伤害时调用 `damage_number` 生成伤害数字。
- **生命周期**：`AutoDespawn` + `despawn_on_tween_complete` 系统监听 `AnimCompletedEvent`，动画完成后自动销毁实体。
