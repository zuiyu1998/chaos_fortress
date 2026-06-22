# RoleBuilderContext

`RoleBuilderContext` 是构建角色实体所需的**上下文参数对象**，将网格位置、父子关系、属性集以及技能相关容器和定义封装为一个结构体。

## 设计动机

将 `build` 方法中非 `Commands` 的参数集合为单一上下文对象，新增参数只需在 `RoleBuilderContext` 中增加字段，trait 签名保持不变，所有现有实现无需修改。

## 定义

```rust
/// 构建角色实体所需的上下文环境。
///
/// 封装了网格位置、父子关系、属性集以及技能相关容器和定义。`Commands` 由 [`RoleBuilder::build`]
/// 直接传入，不存储在此结构体中。
pub struct RoleBuilderContext<'a> {
    /// 角色在网格中的位置，格式为 `(列, 行)`。
    pub position: (u32, u32),
    /// 可选的父实体（如 Level 实体）。
    /// 当为 `Some` 时，实现者应将角色生成为该实体的子级。
    pub parent: Option<Entity>,
    /// 属性集，定义角色的战斗数值（生命、攻击力等）。
    pub attributes: AttributeSet,
    /// 技能特性构建器容器，用于创建技能子实体时应用技能特性构建器。
    pub skill_container: &'a SkillFeatureBuilderContainer,
    /// 技能效果构建器容器，用于创建技能子实体时应用技能效果构建器。
    pub skill_effect_container: &'a SkillEffectBuilderContainer,
    /// 技能定义，从资产句柄解析得到。
    pub skill: &'a SkillDefinition,
    /// 技能句柄，用于创建 [`SkillInstance`] 时使用。
    pub skill_handle: Handle<SkillDefinition>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|---|---|---|
| `position` | `(u32, u32)` | 角色所在的网格坐标，格式为 `(列, 行)`。列对应 X 轴，行对应 Y 轴，原点 (0,0) 为网格左上角。 |
| `parent` | `Option<Entity>` | 父实体。当角色需要挂接到某个实体（如 `Level`）下时使用。`None` 表示无父实体。 |
| `attributes` | `AttributeSet` | 属性集，封装角色的战斗数值（如最大生命、攻击力、防御力等），由外部系统在构建阶段注入。 |
| `skill_container` | `&'a SkillFeatureBuilderContainer` | 技能特性构建器容器，用于创建技能子实体时应用技能特性构建器。 |
| `skill_effect_container` | `&'a SkillEffectBuilderContainer` | 技能效果构建器容器，用于创建技能子实体时应用技能效果构建器。 |
| `skill` | `&'a SkillDefinition` | 技能定义，从资产句柄解析得到的技能数据。 |
| `skill_handle` | `Handle<SkillDefinition>` | 技能句柄，用于创建 `SkillInstance` 时引用技能资产。 |

## 构造方式

```rust
let ctx = RoleBuilderContext {
    position: (0, 9),
    parent: Some(level_entity),
    attributes: AttributeSet::default(),
    skill_container: &skill_container,
    skill_effect_container: &skill_effect_container,
    skill: &skill_def,
    skill_handle: skill_handle.clone(),
};
```

## 变更指南

当需要新增上下文参数时：

1. 在 `RoleBuilderContext` 中添加新字段。
2. 检查所有构造 `RoleBuilderContext` 的调用点，补充新字段。
3. `RoleBuilder::build` 的 trait 定义及已有实现**无需任何修改**。
