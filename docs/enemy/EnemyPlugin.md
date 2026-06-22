# EnemyPlugin

`EnemyPlugin` 是一个结构体，实现了 Bevy 的 `Plugin` trait，用于注册敌人模块所需的核心组件和系统。

## 概述

`EnemyPlugin` 是敌人模块的入口插件，负责在 App 中注册敌人相关的 ECS 类型和运行系统。

```rust
pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>();
        app.register_type::<Base>();
        app.insert_resource(EnemyBuilderContainer::new());
        app.load_resource::<assets::EnemyAssets>();
        let mut container = app.world_mut().resource_mut::<EnemyBuilderContainer>();
        container.register("basic", BasicEnemyBuilder);

        // 监控敌人进入基地的碰撞事件
        app.add_systems(Update, check_enemy_enters_base);
    }
}
```

## 注册内容

| 注册项 | 类型 | 说明 |
|--------|------|------|
| `Enemy` | Component | 标记实体为敌人单位的组件 |
| `Base` | Component | 标记实体为基地（敌人不可踏入区域）的组件 |
| `EnemyBuilderContainer` | Resource | 敌人构建器容器 |
| `EnemyAssets` | Resource/Asset | 敌人资源句柄 |
| `check_enemy_enters_base` | System | 检测敌人进入基地并触发结算的系统 |

## 使用方式

在 `main.rs` 的 `AppPlugin` 中与其他插件一同添加：

```rust
app.add_plugins((
    // ... 其他插件
    enemy::EnemyPlugin,
));
```

## 与现有模块的关系

### Enemy 组件

`EnemyPlugin` 注册的 `Enemy` 组件用于标记敌人实体，配合 `enemy()` 生成函数使用：

```rust
// enemy::enemy() 返回包含 Enemy 组件的 Bundle
commands.spawn(enemy::enemy(&map_data, col, row, sprite));
```

### Base 组件

`Base` 组件标记基地实体，配合 `enemy::base()` 生成函数使用。基地实体带有 `CollisionEventsEnabled` 和 `Sensor`，可触发碰撞事件但不产生物理响应：

```rust
// 生成一个 2×5 格子的基地区域
commands.spawn(enemy::base(0, 0, cell_size, 2, 5));
```

`base()` 返回的 Bundle 包含：
- `Base` 标记组件
- `Collider::rectangle` 碰撞体（大小 = 宽 × 高，以格子的像素尺寸计算）
- `CollisionEventsEnabled` 启用碰撞事件
- `Sensor` 传感器模式（不产生物理反弹，仅触发事件）
- `GamePhysicsLayer::base_layers()` 物理碰撞层级（Base 层，仅过滤 Enemy）

### check_enemy_enters_base 系统

该系统在 `Update` 阶段运行，读取 `CollisionStart` 碰撞事件：

```rust
pub fn check_enemy_enters_base(
    mut started: MessageReader<CollisionStart>,
    bases: Query<&Base>,
    enemies: Query<&Enemy>,
) {
    for event in started.read() {
        let (e1, e2) = (event.collider1, event.collider2);
        if (bases.contains(e1) && enemies.contains(e2))
            || (bases.contains(e2) && enemies.contains(e1))
        {
            info!("游戏已结算");
        }
    }
}
```

当检测到敌人与基地发生碰撞时，打印日志 `"游戏已结算"`。

### GamePhysicsLayer

建议在生成敌人实体时，附加 `GamePhysicsLayer::enemy_layers()` 配置物理碰撞层级：

```rust
commands.spawn((
    enemy::enemy(&map_data, col, row, sprite),
    GamePhysicsLayer::enemy_layers(),
));
```

### 交互流程

1. `EnemyPlugin` 注册 `Enemy`、`Base` 组件和 `check_enemy_enters_base` 系统到 Bevy 世界
2. 关卡系统生成地图、基地（`enemy::base()`）和敌人（`enemy::enemy()`）
3. 敌人携带 `Enemy` 组件和物理碰撞进入 ECS 世界
4. 敌人 AI 系统驱动敌人移动
5. 当敌人进入基地区域时，`check_enemy_enters_base` 系统检测到碰撞事件，打印 `"游戏已结算"`
6. 敌人 AI 系统通过 `Query<&Enemy>` 查询所有敌人实体进行行为驱动

## 注意事项

- `EnemyPlugin` 使用 `pub(super)` 可见性，仅在 `crate` 内部可用
- 基地使用 `Sensor` 模式，表示它是一个触发器区域而非物理障碍物——敌人可以穿过基地，但碰撞事件会被触发
- 当前 `check_enemy_enters_base` 仅打印日志，后续可扩展为真正的游戏结算逻辑（如暂停游戏、播放结算动画、切换到结算界面等）
