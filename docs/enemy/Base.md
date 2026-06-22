# Base

`Base` 是一个组件（Component），用于标记一个实体为**基地**——代表敌人不可踏入的区域。

## 用途

- 持有 `Base` 组件的实体定义了一个**逻辑禁区**，敌人 AI 在移动规划时视其占用格子为不可通行。
- 防止敌人单位进入玩家部署区（备战区），保护玩家角色不被近身。
- 可与物理碰撞结合：为基地实体附加 `Collider` 并使用 `GamePhysicsLayer::base_layers()`，使碰撞到基地的敌人触发游戏结束逻辑。

## 与物理层的关系

`Base` 组件与 [`GamePhysicsLayer::Base`](../common/GamePhysicsLayer.md) 层配合使用：

- **逻辑层面**：敌人 AI 在寻路时检查基地格子，将其排除在可行走格子之外。
- **物理层面**：基地实体可附加碰撞体并归属于 `Base` 层（仅过滤 `Enemy`），碰撞事件可用于触发游戏结束等逻辑。

## 示例

```rust
// 在备战区后方生成一个基地实体
commands.spawn((
    Name::new("Base"),
    Base,
    Sprite::from_color(Color::srgb(0.2, 0.2, 0.8), Vec2::splat(64.0)),
    Transform::from_xyz(x, y, VisualDisplayLayer::Character.z_value()),
    Collider::rectangle(128.0, 320.0),
    GamePhysicsLayer::base_layers(),
));
```

## 相关模块

- **[`Enemy`](./Enemy.md)**：敌人组件，其 AI 系统在移动时检查 `Base` 实体。
- **[`GamePhysicsLayer`](../common/GamePhysicsLayer.md)**：`Base` 层定义及碰撞配置。
- **[`MapSystem`](../map/MapSystem.md)**：基地部署在备战区后方，构成地图的一部分。
