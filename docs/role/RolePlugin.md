# RolePlugin

`RolePlugin` 是一个结构体，实现了 Bevy 的 `Plugin` trait，用于注册角色模块所需的核心组件与资源。

## 概述

`RolePlugin` 是角色模块的入口插件，负责注册 `Role` 标记组件、初始化 `RoleBuilderContainer` 资源，并将子模块插件（如 `ArcherPlugin`）添加到应用中。

```rust
pub(super) struct RolePlugin;

impl Plugin for RolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Role>();
        app.insert_resource(RoleBuilderContainer::new());
        app.add_plugins(archer::ArcherPlugin);

        app.add_systems(
            Update,
            (
                update_enemy_target_on_collision,
                cleanup_enemy_target_list,
                sync_primary_target,
            ),
        );
    }
}
```

## 注册内容

| 注册项 | 类型 | 说明 |
|--------|------|------|
| `Role` | Component | 标记实体为角色（可控单位）的标记组件 |
| `RoleBuilderContainer` | Resource | 按名称查找构建器的 Bevy 资源，用于动态创建角色实体 |

## 子模块插件

`RolePlugin` 在构建时会自动添加以下子模块插件：

| 插件 | 说明 |
|------|------|
| [`ArcherPlugin`](archer/ArcherPlugin.md) | 弓箭手模块，注册 `Archer`、`AttackSpeed`、`ProjectileDamage` 等组件，注册 `ArcherRoleBuilder` 到 `RoleBuilderContainer` |

## 系统

### update_enemy_target_on_collision

该系统处理 `CollisionStarted` 和 `CollisionEnded` 事件，当碰撞涉及的一对实体分别是 `Role` 和 `Enemy` 时，更新 `EnemyTarget` 及 `EnemyTargetList`。

```rust
/// 根据碰撞事件维护 Role 实体的敌人目标列表。
///
/// 当 Role（或攻击范围传感器）与 Enemy 发生碰撞时，
/// 将敌人加入 EnemyTargetList；碰撞结束时移除。
pub fn update_enemy_target_on_collision(
    mut started: MessageReader<CollisionStart>,
    mut ended: MessageReader<CollisionEnd>,
    sensors: Query<&ChildOf, With<AttackRange>>,
    mut lists: Query<&mut EnemyTargetList>,
    enemies: Query<&Enemy>,
) {
    for event in started.read() {
        let (e1, e2) = (event.collider1, event.collider2);
        if let Some(role_entity) = find_role(e1, e2, &sensors, &enemies) {
            if let Ok(mut list) = lists.get_mut(role_entity) {
                let enemy = if enemies.contains(e1) { e1 } else { e2 };
                if !list.0.contains(&enemy) {
                    list.0.push(enemy);
                }
            }
        }
    }
    for event in ended.read() {
        let (e1, e2) = (event.collider1, event.collider2);
        if let Some(role_entity) = find_role(e1, e2, &sensors, &enemies) {
            if let Ok(mut list) = lists.get_mut(role_entity) {
                let enemy = if enemies.contains(e1) { e1 } else { e2 };
                list.0.retain(|e| e != &enemy);
            }
        }
    }
}

/// 辅助函数：从一对碰撞实体中识别出角色根实体。
///
/// 碰撞实体可能是角色本体或其子实体（如 AttackRange 传感器），
/// 通过 Parent 链向上查找角色根实体。
fn find_role(
    e1: Entity,
    e2: Entity,
    sensors: &Query<&ChildOf, With<AttackRange>>,
    enemies: &Query<&Enemy>,
) -> Option<Entity> {
    // 如果 e1 是敌人，则 e2 是角色方
    if enemies.contains(e1) {
        resolve_role_root(e2, sensors)
    // 如果 e2 是敌人，则 e1 是角色方
    } else if enemies.contains(e2) {
        resolve_role_root(e1, sensors)
    } else {
        None
    }
}

/// 将可能的传感器子实体沿 Parent 链向上解析为角色根实体。
fn resolve_role_root(
    entity: Entity,
    sensors: &Query<&ChildOf, With<AttackRange>>,
) -> Option<Entity> {
    if sensors.get(entity).is_ok() {
        sensors.get(entity).ok().map(|p| p.get())
    } else {
        Some(entity) // 已经是角色本体
    }
}
```

#### 工作流程

1. `CollisionStarted` 事件触发，碰撞实体对为 `(sensor_or_role, enemy)`
2. 通过 `enemies.contains()` 和 `sensors` 查询识别角色根实体
3. 将敌人实体追加到角色实体的 `EnemyTargetList` 中
4. `CollisionEnded` 时从列表中移除对应敌人

#### 涉及的组件

| 组件 | 用途 |
|------|------|
| [`EnemyTargetList`](../../common/EnemyTargetList.md) | 存储攻击范围内所有敌人 |
| [`AttackRange`](../../common/AttackRange.md) | 用于识别传感器子实体 |
| `Enemy` | 标记敌人实体 |
| `Role` | 标记角色实体 |

### cleanup_enemy_target_list

该系统清理所有 [`EnemyTargetList`](../../common/EnemyTargetList.md) 中已经失效的敌人实体引用（如已被销毁或移除了 `Enemy` 组件的实体），防止过期引用累积。

```rust
/// 清理 EnemyTargetList 中已失效的敌人实体引用。
///
/// 移除已被销毁或失去 Enemy 组件的实体，防止过期引用累积。
pub fn cleanup_enemy_target_list(
    mut lists: Query<&mut EnemyTargetList>,
    enemies: Query<&Enemy>,
) {
    for mut list in &mut lists {
        list.0.retain(|e| enemies.contains(*e));
    }
}
```

#### 工作流程

1. 遍历所有携带 `EnemyTargetList` 的实体
2. 对列表中的每个敌人实体，检查其是否仍然存在且携带 `Enemy` 组件
3. 移除已失效的实体引用

#### 涉及的组件

| 组件 | 用途 |
|------|------|
| [`EnemyTargetList`](../../common/EnemyTargetList.md) | 存储攻击范围内所有敌人 |
| `Enemy` | 用于校验实体是否仍为有效敌人 |

### sync_primary_target

该系统从 [`EnemyTargetList`](../../common/EnemyTargetList.md) 中选取第一个敌人作为主目标同步到 [`EnemyTarget`](../../common/EnemyTarget.md)。

```rust
/// 从 EnemyTargetList 列表头部选取主目标同步到 EnemyTarget。
///
/// 列表为空时将 EnemyTarget 设为 None。
pub fn sync_primary_target(
    mut query: Query<(&EnemyTargetList, &mut EnemyTarget)>,
) {
    for (list, mut target) in &mut query {
        target.0 = list.0.first().copied();
    }
}
```

#### 工作流程

1. 遍历所有同时携带 `EnemyTargetList` 和 `EnemyTarget` 的实体
2. 从 `EnemyTargetList` 中取第一个元素作为主目标
3. 写入 `EnemyTarget`；列表为空时写入 `None`

#### 涉及的组件

| 组件 | 用途 |
|------|------|
| [`EnemyTargetList`](../../common/EnemyTargetList.md) | 存储攻击范围内所有敌人的候选列表 |
| [`EnemyTarget`](../../common/EnemyTarget.md) | 存储当前选中的主目标 |

## 使用方式

在 `main.rs` 的 `AppPlugin` 中与其他插件一同添加：

```rust
app.add_plugins((
    // ... 其他插件
    role::RolePlugin,
));
```

## 与现有文档的关系

| 文档 | 说明 |
|------|------|
| [`Role`](Role.md) | `Role` 标记组件定义与用途 |
| [`RoleBuilder`](RoleBuilder.md) | 角色构建器 trait，用于扩展新的角色类型 |
| [`RoleBuilderContainer`](RoleBuilderContainer.md) | 按名称查找 builder 的资源容器 |
| [`RoleBuilderContext`](RoleBuilderContext.md) | 构建角色的位置与父实体上下文 |
| [`ArcherPlugin`](archer/ArcherPlugin.md) | 弓箭手子模块插件 |
