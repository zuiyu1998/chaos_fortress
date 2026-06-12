# DamageNumber

`DamageNumber` 是一个标记组件（Marker Component），用于标识伤害数字实体。

## 用途

- 持有 `DamageNumber` 组件的实体代表一个**飘浮伤害数字**，在战斗中向玩家展示造成的伤害值。
- 作为 bevy_lunex UI 树的子节点，通过 `UiLayout` 定位、`UiTextSize` 控制字号、`UiColor` 着色。
- 便于其他系统在查询中快速筛选出伤害数字实体，进行动画更新或生命周期管理。

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
// 生成一个伤害数字（作为 UI 根节点的子实体）
commands.spawn(common::damage_number(
    -42,                                // 伤害数值
    50.0,                                // 相对于父容器的 X 百分比
    30.0,                                // 相对于父容器的 Y 百分比
    asset_server.load("fonts/semibold.ttf"),
));
```

## 与现有模块的关系

- **UI 模块**：伤害数字通常作为 `UIRoot2d` 根节点的子实体生成，或直接挂在世界空间下。
- **战斗系统**：由战斗系统在造成伤害时生成，通过 `DamageNumber` 标记查询并驱动浮空动画。
- **生命周期**：伤害数字一般为临时实体，可配合计时器或动画结束事件自动销毁。
