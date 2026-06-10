# RoleBuilderContext

`RoleBuilderContext` 是构建角色实体所需的**上下文参数对象**，将网格位置和父子关系封装为一个结构体。

## 设计动机

将 `build` 方法中非 `Commands` 的参数集合为单一上下文对象，新增参数只需在 `RoleBuilderContext` 中增加字段，trait 签名保持不变，所有现有实现无需修改。

## 定义

```rust
/// 构建角色实体所需的上下文环境。
///
/// 封装了网格位置以及父子关系。`Commands` 由 [`RoleBuilder::build`]
/// 直接传入，不存储在此结构体中。
pub struct RoleBuilderContext {
    /// 角色在网格中的位置，格式为 `(列, 行)`。
    pub position: (u32, u32),
    /// 可选的父实体（如 Level 实体）。
    /// 当为 `Some` 时，实现者应将角色生成为该实体的子级。
    pub parent: Option<Entity>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|---|---|---|
| `position` | `(u32, u32)` | 角色所在的网格坐标，格式为 `(列, 行)`。列对应 X 轴，行对应 Y 轴，原点 (0,0) 为网格左上角。 |
| `parent` | `Option<Entity>` | 父实体。当角色需要挂接到某个实体（如 `Level`）下时使用。`None` 表示无父实体。 |

## 构造方式

```rust
let ctx = RoleBuilderContext {
    position: (0, 9),
    parent: Some(level_entity),
};
```

## 变更指南

当需要新增上下文参数时：

1. 在 `RoleBuilderContext` 中添加新字段。
2. 检查所有构造 `RoleBuilderContext` 的调用点，补充新字段。
3. `RoleBuilder::build` 的 trait 定义及已有实现**无需任何修改**。
