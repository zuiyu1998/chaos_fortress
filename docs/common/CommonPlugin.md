# CommonPlugin

`CommonPlugin` 是一个结构体，实现了 Bevy 的 `Plugin` trait，用于注册游戏中跨模块共享的基础组件。

## 概述

`CommonPlugin` 是通用模块的入口插件，负责向 Bevy 世界注册所有跨模块复用的基础类型。这些类型包括渲染层级、物理碰撞层、攻击范围、冷却计时器以及目标管理相关的组件。

```rust
pub(super) struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VisualDisplayLayer>();
        app.register_type::<GamePhysicsLayer>();
        app.register_type::<AttackRange>();
        app.register_type::<CoolingTimer>();
        app.register_type::<EnemyTarget>();
        app.register_type::<EnemyTargetList>();
    }
}
```

## 注册内容

| 注册项 | 类型 | 说明 |
|--------|------|------|
| `VisualDisplayLayer` | Enum | 视觉渲染层级（Terrain / Character / Bullet），控制 z 轴渲染顺序 |
| `GamePhysicsLayer` | Enum | 物理碰撞层级（World / Character / Enemy），控制实体间碰撞交互 |
| `AttackRange` | Component | 攻击范围（像素），用于远程单位的攻击判定 |
| `CoolingTimer` | Component | 冷却计时器，用于控制攻击频率等周期性行为 |
| `EnemyTarget` | Component | 当前锁定的敌人实体（`Option<Entity>`），可为空 |
| `EnemyTargetList` | Component | 攻击范围内所有敌人实体的列表（`Vec<Entity>`） |

## 使用方式

在 `main.rs` 的 `AppPlugin` 中与其他插件一同添加：

```rust
app.add_plugins((
    // ... 其他插件
    common::CommonPlugin,
));
```

## 与现有文档的关系

| 文档 | 说明 |
|------|------|
| [`VisualDisplayLayer`](VisualDisplayLayer.md) | 视觉渲染层级定义与 z 值对照 |
| [`GamePhysicsLayer`](GamePhysicsLayer.md) | 物理碰撞层级配置与辅助函数 |
| [`AttackRange`](AttackRange.md) | 攻击范围组件及传感器生成函数 |
| [`CoolingTimer`](CoolingTimer.md) | 冷却计时器组件与 `tick_all` 系统 |
| [`EnemyTarget`](EnemyTarget.md) | 当前锁定目标组件 |
| [`EnemyTargetList`](EnemyTargetList.md) | 攻击范围目标列表组件 |
