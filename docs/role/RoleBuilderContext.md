# RoleBuilderContext

`RoleBuilderContext` 是 `RoleBuilder::build` 方法的**统一上下文参数对象**，将构建角色实体所需的环境参数封装为一个结构体，避免 trait 方法签名随着需求增长而不断膨胀。

## 设计动机

当 `build` 方法直接接收多个散落参数（`commands`、`position`、`parent`）时，每次新增参数都需修改 trait 定义和所有实现，破坏接口的稳定性和实现的兼容性。

通过将参数集合为单一上下文对象，新增参数只需在 `RoleBuilderContext` 中增加字段，trait 签名保持不变，所有现有实现无需修改。

## 定义

```rust
/// 构建角色实体所需的上下文环境。
///
/// 封装了实体创建工具、网格位置以及父子关系等横切关注点。
pub struct RoleBuilderContext<'w, 's> {
    /// 用于生成实体的 Bevy `Commands`。
    pub commands: &'w mut Commands<'w, 's>,
    /// 角色在网格中的位置，格式为 `(行, 列)`。
    pub position: (u32, u32),
    /// 可选的父实体（如 Level 实体）。
    /// 当为 `Some` 时，实现者应将角色生成为该实体的子级。
    pub parent: Option<Entity>,
}
```

> 使用泛型生命周期 `'w, 's` 以兼容 Bevy 的 `Commands` 类型签名。

## 字段说明

| 字段 | 类型 | 说明 |
|---|---|---|
| `commands` | `&mut Commands<'w, 's>` | Bevy 的实体生成器。通过它调用 `spawn`、`insert`、`set_parent` 等操作。 |
| `position` | `(u32, u32)` | 角色所在的网格坐标，格式为 `(列, 行)`。列对应 X 轴，行对应 Y 轴，原点 (0,0) 为网格左上角。 |
| `parent` | `Option<Entity>` | 父实体。当角色需要挂接到某个实体（如 `Level`）下时使用。`None` 表示无父实体。 |

## 构造方式

建议通过命名构造函数或结构体更新语法创建：

```rust
// 方式一：直接构造
let ctx = RoleBuilderContext {
    commands: &mut commands,
    position: (0, 9),
    parent: Some(level_entity),
};

// 方式二：辅助方法（在 RoleBuilder 或 RoleBuilderContext 上提供）
let ctx = RoleBuilderContext::new(&mut commands, (0, 9), Some(level_entity));
```

## 使用示例

```rust
fn spawn_player(mut commands: Commands, level_entity: Entity) {
    let ctx = RoleBuilderContext {
        commands: &mut commands,
        position: (0, 9),
        parent: Some(level_entity),
    };

    PlayerBuilder { name: "Hero".into() }.build(ctx);
}
```

## 变更指南

当需要新增上下文参数时：

1. 在 `RoleBuilderContext` 中添加新字段。
2. 检查所有构造 `RoleBuilderContext` 的调用点，补充新字段。
3. `RoleBuilder` trait 定义及已有实现**无需任何修改**。
