# BulletPosition

`BulletPosition` 是一个标记组件（Marker Component），用于标识子弹实体已记录起始位置。

## 用途

- 持有 `BulletPosition` 组件的实体表示其发射时的初始位置已被记录。
- 该组件作为标记，用于在子弹飞行系统中标识需要追踪起始位置的子弹，以便后续在碰撞或销毁时获取发射原点。
- 该组件通常在子弹生成时由 `bullet` 函数附加。

## 定义

```rust
/// 标记子弹已记录起始位置。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct BulletPosition;
```

## 与现有模块的关系

- **子弹模块**（`bullet`）：`bullet` 函数在生成子弹实体时附加 `BulletPosition` 标记组件。
- **战斗系统**（`battle`）：子弹飞行系统通过查询 `BulletPosition` 标记来识别需要追踪起始位置的子弹。
