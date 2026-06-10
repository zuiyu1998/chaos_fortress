# EnemyPlugin

`EnemyPlugin` 是一个结构体，实现了 Bevy 的 `Plugin` trait，用于注册敌人模块所需的核心组件。

## 概述

`EnemyPlugin` 是敌人模块的入口插件，负责在 App 中注册敌人相关的 ECS 类型。当前主要注册 `Enemy` 组件，使 Bevy 的反射系统能够识别和操作它。

```rust
pub(super) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>();
    }
}
```

## 注册内容

| 注册项 | 类型 | 说明 |
|--------|------|------|
| `Enemy` | Component | 标记实体为敌人单位的组件 |

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

### GamePhysicsLayer

建议在生成敌人实体时，附加 `GamePhysicsLayer::enemy_layers()` 配置物理碰撞层级：

```rust
commands.spawn((
    enemy::enemy(&map_data, col, row, sprite),
    GamePhysicsLayer::enemy_layers(),
));
```

### 交互流程

1. `EnemyPlugin` 注册 `Enemy` 组件到 Bevy 世界
2. 关卡系统调用 `enemy::enemy()` 构造敌人实体 Bundle
3. 敌人实体携带 `Enemy` 组件进入 ECS 世界
4. 敌人 AI 系统通过 `Query<&Enemy>` 查询所有敌人实体进行行为驱动

## 注意事项

- `EnemyPlugin` 使用 `pub(super)` 可见性，仅在 `crate` 内部可用
- 当前仅注册 `Enemy` 组件，后续可根据需要扩展注册更多类型
