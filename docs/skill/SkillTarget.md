# SkillTarget

`SkillTarget` 是一个组件（Component），用于持有指向技能实体的引用，方便从其他实体（如技能持有者）快速访问对应的技能实体。

## 用途

- 持有 `SkillTarget` 组件的实体（通常是技能持有者）可通过该组件直接访问技能实体。
- 在技能系统中，持有者实体可通过 `SkillTarget` 组件查找对应的 [`SkillInstance`] 和 [`SkillRunContext`]。
- 避免在系统中通过父子关系查询来定位技能实体，提供更直接的访问路径。

## 定义

```rust
/// 指向技能实体的引用。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct SkillTarget(pub Entity);
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `0` | `Entity` | 技能实体的 Entity 引用，用于在系统中直接访问技能实体。 |

## 方法

| 方法 | 说明 |
|------|------|
| `new(entity)` | 创建一个新的 `SkillTarget`，指向指定的技能实体。 |

## 与现有模块的关系

- **[`SkillInstance`]**：通过 `SkillTarget` 获取技能实体后，可读取其上的 [`SkillInstance`] 组件获取运行时状态。
- **[`SkillRunContext`]**：通过 `SkillTarget` 获取技能实体后，可读取其上的 [`SkillRunContext`] 组件获取施放上下文。
- **技能持有者**：`SkillTarget` 组件通常附加在技能持有者实体上，指向其技能子实体。

### 典型使用流程

```
1. 技能创建时，在持有者实体上附加 SkillTarget 组件，指向新生成的技能实体。
2. 系统中通过持有者实体上的 SkillTarget 快速定位技能实体。
3. 通过技能实体上的 SkillInstance、SkillRunContext 等组件获取状态信息。
```

[`SkillInstance`]: ./SkillInstance.md
[`SkillRunContext`]: ./SkillRunContext.md
