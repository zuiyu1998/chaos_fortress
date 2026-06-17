# SkillRunContext

`SkillRunContext` 是一个组件（Component），用于存储技能在施放过程中的**运行上下文数据**，包括所属实体以及 SkillFeature 执行结果记录。

## 用途

- 持有 `SkillRunContext` 组件的实体代表一个正在执行中的技能实例，记录本次施放的上下文信息。
- 在技能施放的整个流程中（构建、飞行、命中、结算），各系统可读取该组件获取上下文数据。
- 与 [`SkillInstance`] 配合使用：`SkillInstance` 存储技能的持久运行时状态（充能、冷却），`SkillRunContext` 存储单次施放的临时上下文。

## 定义

```rust
use std::collections::HashMap;

use crate::skill::SkillDefinition;

/// 技能运行上下文。
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SkillRunContext {
    /// 技能所属的实体（技能持有者）。
    pub owner: Entity,
    /// SkillFeature 执行结果记录，键为特征 id，值为 [`SkillFeatureResult`] 枚举。
    #[reflect(ignore)]
    pub feature_results: HashMap<String, SkillFeatureResult>,
}

impl SkillRunContext {
    /// 创建一个新的 SkillRunContext。
    /// 根据技能定义，为每个 feature 预设初始状态 `Ready`。
    pub fn new(
        owner: Entity,
        definition: &SkillDefinition,
    ) -> Self { ... }
    /// 记录一个 SkillFeature 的执行结果。
    pub fn record_feature_result(
        &mut self,
        feature_id: impl Into<String>,
        result: SkillFeatureResult,
    ) { ... }
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `owner` | `Entity` | 技能所属的实体（技能持有者），即拥有该技能的实体。 |
| `feature_results` | `HashMap<String, SkillFeatureResult>` | SkillFeature 执行结果记录，键为特征 `id`，值为 [`SkillFeatureResult`] 枚举。使用 `#[reflect(ignore)]` 标记，不参与反射。 |

## 方法

| 方法 | 说明 |
|------|------|
| `new(owner, definition)` | 创建一个新的 `SkillRunContext`。根据 `definition.features` 为每个 feature 预设 [`SkillFeatureResult::Ready`] 作为初始状态。 |
| `record_feature_result(feature_id, result)` | 记录一个 SkillFeature 的执行结果。`result` 为 [`SkillFeatureResult`] 枚举值。 |

## 与现有模块的关系

- **[`SkillDefinition`]**：`new` 方法接收 `&SkillDefinition`，从中读取 `features` 列表以预填充 `feature_results`。
- **[`SkillInstance`]**：`SkillInstance` 存储技能的持久状态（充能、状态），`SkillRunContext` 存储单次施放的临时上下文。两者共存于技能实体上。
- **[`SkillFeatureBuilder`]**：在技能施放时，`SkillRunContext` 中的 `feature_results` 可记录每个特征构建器的执行结果。
- **`run_skill` 系统**：在技能施放流程中，创建 `SkillRunContext` 并附加到技能实体上，遍历 features 后更新其执行结果。

### 典型使用流程

```
1. run_skill 系统检测到技能就绪（SkillInstance::ready()）。
2. 创建 SkillRunContext 组件并附加到技能实体上，传入 owner 和 SkillDefinition。
   → feature_results 自动预填充每个 feature 的初始状态为 Ready。
3. 调用 SkillInstance::use_skill() 消耗充能。
4. 遍历 SkillDefinition.features，使用 SkillFeatureBuilderContainer 应用特征构建器。
5. 每个特征执行后，调用 record_feature_result 更新结果（成功记录 Ok，失败记录 Error 及错误信息）。
   → 特征状态从 Ready 转变为 Ok 或 Error。
6. 后续系统可读取 feature_results 判断特征是否全部完成（全部为 Ok 时即可执行后续逻辑）。
```

[`SkillInstance`]: ./SkillInstance.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillFeatureBuilder`]: ./SkillFeatureBuilder.md
[`SkillFeatureBuilderContext`]: ./SkillFeatureBuilder.md#相关类型
[`SkillFeatureResult`]: ./SkillFeatureResult.md
[`SkillFeatureResult::Ready`]: ./SkillFeatureResult.md#变体说明

