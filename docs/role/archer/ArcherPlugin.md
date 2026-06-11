# ArcherPlugin

`ArcherPlugin` 是一个结构体，实现了 Bevy 的 `Plugin` trait，用于注册弓箭手模块所需的核心组件。

## 概述

`ArcherPlugin` 是弓箭手模块的入口插件，负责在 App 中注册与弓箭手相关的 ECS 类型，使 Bevy 的反射系统能够识别和操作它们。

```rust
pub(super) struct ArcherPlugin;

impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Archer>();
        app.register_type::<AttackSpeed>();
        app.register_type::<ProjectileDamage>();
    }
}
```

## 注册内容

| 注册项 | 类型 | 说明 |
|--------|------|------|
| `Archer` | Component | 标记实体为弓箭手的标记组件 |
| `AttackSpeed` | Component | 弓箭手的攻击间隔（秒） |
| `ProjectileDamage` | Component | 弓箭手投射物伤害值 |

## 使用方式

在 `main.rs` 的 `AppPlugin` 中与其他插件一同添加：

```rust
app.add_plugins((
    // ... 其他插件
    role::archer::ArcherPlugin,
));
```

## 与现有模块的关系

### Archer 组件

`ArcherPlugin` 注册的 `Archer` 组件用于标记弓箭手实体，配合 `ArcherRoleBuilder` 在构建时附加：

```rust
// ArcherRoleBuilder::build 中生成的实体包含 Archer 组件
commands.spawn((
    Role,
    Archer,
    AttackRange(self.attack_range),
    AttackSpeed(self.attack_speed),
    ProjectileDamage(self.projectile_damage),
    // ...
));
```

### 属性组件

`AttackSpeed` 和 `ProjectileDamage` 是弓箭手专属的属性组件，在 `archer.rs` 中定义，搭配 `AttackRange`（定义在 `common` 模块）共同描述弓箭手的远程战斗能力。

### 交互流程

1. `ArcherPlugin` 注册 `Archer`、`AttackSpeed`、`ProjectileDamage` 到 Bevy 世界
2. 角色系统通过 `RoleBuilderContainer` 调用 `ArcherRoleBuilder` 构造弓箭手实体
3. 弓箭手实体携带 `Archer` 标记及属性组件进入 ECS 世界
4. 战斗系统通过 `Query<&Archer>` 查询所有弓箭手实体进行远程攻击行为驱动

## 注意事项

- `ArcherPlugin` 使用 `pub(super)` 可见性，仅在 `crate` 内部可用
- `Archer` 为标记组件，不含运行时数据；实际属性由 `AttackSpeed`、`ProjectileDamage` 等组件承载
- 后续可根据需要扩展注册更多类型（如箭矢相关的组件、事件等）
