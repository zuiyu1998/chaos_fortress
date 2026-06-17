# SkillRunContext

`SkillRunContext` 是一个组件（Component），用于存储技能在施放过程中的**运行上下文数据**，包括所属实体以及 SkillFeature 执行结果记录。

## 用途

- 持有 `SkillRunContext` 组件的实体代表一个正在执行中的技能实例，记录本次施放的上下文信息。
- 在技能施放的整个流程中（构建、飞行、命中、结算），各系统可读取该组件获取上下文数据。
- 与 [`SkillInstance`] 配合使用：`SkillInstance` 存储技能的持久运行时状态（充能、冷却），`SkillRunContext` 存储单次施放的临时上下文。

## 定义

```rust
use std::collections::HashMap;

/// 技能运行上下文。
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SkillRunContext {
    /// 技能所属的实体（技能持有者）。
    pub owner: Entity,
    /// SkillFeature 执行结果记录，键为特征 id，值为 trait 对象（实现了 SkillFeatureResult 的类型）。
    #[reflect(ignore)]
    pub feature_results: HashMap<String, Box<dyn SkillFeatureResult>>,
}

impl SkillRunContext {
    /// 创建一个新的 SkillRunContext。
    pub fn new(
        owner: Entity,
    ) -> Self { ... }
    /// 记录一个 SkillFeature 的执行结果。
    pub fn record_feature_result(
        &mut self,
        feature_id: impl Into<String>,
        result: impl SkillFeatureResult + 'static,
    ) { ... }
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `owner` | `Entity` | 技能所属的实体（技能持有者），即拥有该技能的实体。 |
| `feature_results` | `HashMap<String, Box&lt;dyn SkillFeatureResult&gt;&gt;` | SkillFeature 执行结果记录，键为特征 `id`，值为实现了 [`SkillFeatureResult`] trait 的 trait 对象。使用 `#[reflect(ignore)]` 标记，不参与反射。 |

## 方法

| 方法 | 说明 |
|------|------|
| `new(owner)` | 创建一个新的 `SkillRunContext`，`feature_results` 初始为空字典。 |
| `record_feature_result(feature_id, result)` | 记录一个 SkillFeature 的执行结果。`result` 为实现了 `SkillFeatureResult` 的类型。 |

## 与现有模块的关系

- **[`SkillInstance`]**：`SkillInstance` 存储技能的持久状态（充能、状态），`SkillRunContext` 存储单次施放的临时上下文。两者共存于技能实体上。
- **[`SkillFeatureBuilder`]**：在技能施放时，`SkillRunContext` 中的 `feature_results` 可记录每个特征构建器的执行结果。
- **`run_skill` 系统**：在技能施放流程中，创建 `SkillRunContext` 并附加到技能实体上，遍历 features 后记录执行结果。

### 典型使用流程

```
1. run_skill 系统检测到技能就绪（SkillInstance::ready()）。
2. 创建 SkillRunContext 组件并附加到技能实体上，设置 owner。
3. 调用 SkillInstance::use_skill() 消耗充能。
4. 遍历 SkillDefinition.features，使用 SkillFeatureBuilderContainer 应用特征构建器。
5. 每个特征执行后，调用 record_feature_result 记录结果（成功记录 Success，失败记录 Failure 及错误信息）。
6. 后续系统可读取 feature_results 判断特征执行状态，处理失败或连锁效果。
```

[`SkillInstance`]: ./SkillInstance.md
[`SkillDefinition`]: ./SkillDefinition.md
[`SkillFeatureBuilder`]: ./SkillFeatureBuilder.md
[`SkillFeatureBuilderContext`]: ./SkillFeatureBuilder.md#相关类型
