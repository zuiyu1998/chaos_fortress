# EnemyBuilderContext

`EnemyBuilderContext` 是构建敌人实体所需的**上下文参数对象**，将网格位置、格子尺寸和父子关系封装为一个结构体。

## 设计动机

将 `build` 方法中非 `Commands` 的参数集合为单一上下文对象，新增参数只需在 `EnemyBuilderContext` 中增加字段，trait 签名保持不变，所有现有实现无需修改。

敌人额外需要 `cell_size` 字段（区别于 `RoleBuilderContext`），因为敌人生成位置的计算依赖格子尺寸，且不同关卡可能使用不同的格子大小。

## 定义

```rust
/// 构建敌人实体所需的上下文环境。
///
/// 封装了网格位置、格子尺寸以及父子关系。`Commands` 由 [`EnemyBuilder::build`]
/// 直接传入，不存储在此结构体中。
pub struct EnemyBuilderContext {
    /// 敌人在网格中的位置，格式为 `(列, 行)`。
    pub position: (u32, u32),
    /// 每个格子的像素尺寸（正方形边长）。
    pub cell_size: f32,
    /// 可选的父实体（如 Level 实体）。
    /// 当为 `Some` 时，实现者应将敌人生成为该实体的子级。
    pub parent: Option<Entity>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|---|---|---|
| `position` | `(u32, u32)` | 敌人所在的网格坐标，格式为 `(列, 行)`。列对应 X 轴，行对应 Y 轴，原点 (0,0) 为网格左上角。 |
| `cell_size` | `f32` | 每个格子的像素尺寸（正方形边长），用于将网格坐标转换为屏幕坐标：`x = col * cell_size`，`y = -(row * cell_size)`。 |
| `parent` | `Option<Entity>` | 父实体。当敌人需要挂接到某个实体（如 `Level`）下时使用。`None` 表示无父实体。 |

## 构造方式

```rust
let ctx = EnemyBuilderContext {
    position: (4, 2),
    cell_size: 64.0,
    parent: Some(level_entity),
};
```

## 变更指南

当需要新增上下文参数时：

1. 在 `EnemyBuilderContext` 中添加新字段。
2. 检查所有构造 `EnemyBuilderContext` 的调用点，补充新字段。
3. `EnemyBuilder::build` 的 trait 定义及已有实现**无需任何修改**。
