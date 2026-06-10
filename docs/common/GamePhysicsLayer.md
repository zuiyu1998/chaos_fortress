# GamePhysicsLayer

`GamePhysicsLayer` 是一个枚举，用于定义游戏中各实体的**物理碰撞层级**，通过 avian2d 的 `PhysicsLayer` trait 驱动 `CollisionLayers` 的生成，控制哪些实体之间会发生物理碰撞。

## 枚举值

| 枚举值 | 对应 bit | 说明 |
|--------|----------|------|
| `World`（默认） | `1 << 0` | 静态世界层，包含地形、墙壁、障碍物等 |
| `Character` | `1 << 1` | 角色层，玩家操控的单位 |
| `Enemy` | `1 << 2` | 敌人层，敌对单位 |

## 碰撞规则

| 实体类型 | 所属层（membership） | 可碰撞层（filter） | 说明 |
|----------|---------------------|-------------------|------|
| `World` | `World` | `Character`, `Enemy` | 世界与角色和敌人碰撞，不与自身碰撞 |
| `Character` | `Character` | `World`, `Enemy` | 角色与世界和敌人碰撞，不与友方碰撞 |
| `Enemy` | `Enemy` | `World`, `Character` | 敌人与世界和角色碰撞，不与其他敌人碰撞 |

## 使用方法

通过 `GamePhysicsLayer` 提供的静态方法快速生成 `CollisionLayers`：

```rust
use avian2d::prelude::*;
use common::GamePhysicsLayer;

// 世界/地形实体（如墙壁、障碍物）
commands.spawn((
    Collider::rectangle(100.0, 10.0),
    GamePhysicsLayer::world_layers(),
));

// 角色实体（玩家）
commands.spawn((
    RigidBody::Dynamic,
    Collider::circle(8.0),
    GamePhysicsLayer::character_layers(),
));

// 敌人实体
commands.spawn((
    RigidBody::Dynamic,
    Collider::circle(8.0),
    GamePhysicsLayer::enemy_layers(),
));
```

也可以直接使用 `CollisionLayers::new` 搭配枚举值进行自定义：

```rust
// 角色只与世界碰撞（不与其他角色或敌人碰撞）
CollisionLayers::new(GamePhysicsLayer::Character, [GamePhysicsLayer::World]);

// 敌人只与世界碰撞
CollisionLayers::new(GamePhysicsLayer::Enemy, [GamePhysicsLayer::World]);
```

## Bit 值说明

`PhysicsLayer` derive 宏按枚举定义的顺序从 `1 << 0` 开始依次分配 bit：

- `World`（第 0 位，`0b001`）—— 默认层，`#[default]` 标记
- `Character`（第 1 位，`0b010`）
- `Enemy`（第 2 位，`0b100`）

## 与现有模块的关系

- **角色模块**：`role` 生成的角色使用 `GamePhysicsLayer::character_layers()`
- **敌人模块**：`enemy` 生成的敌人使用 `GamePhysicsLayer::enemy_layers()`
- **地图模块**：`map_cell` 生成的地形使用 `GamePhysicsLayer::world_layers()`

## 关联 API

- `world_layers() -> CollisionLayers` —— 生成世界实体碰撞配置
- `character_layers() -> CollisionLayers` —— 生成角色实体碰撞配置
- `enemy_layers() -> CollisionLayers` —— 生成敌人实体碰撞配置
