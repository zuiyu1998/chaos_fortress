# SkillFeatureResult

`SkillFeatureResult` 是一个枚举（Enum），用于表示 SkillFeature 执行后的**结果状态**，包含三个变体：就绪中、正常、错误。

## 用途

- 用于记录技能特征（SkillFeature）执行的状态，存储在 [`SkillRunContext`] 的 `feature_results` 字典中。
- 提供统一的结果查询方式，通过模式匹配判断各特征执行状态。

## 定义

```rust
/// SkillFeature 执行结果。
#[derive(Debug, Clone)]
pub enum SkillFeatureResult {
    /// 特征执行仍在进行中，尚未完成。
    Ready,
    /// 特征执行成功。
    Ok(Box<dyn SkillFeatureResultData>),
    /// 特征执行失败。
    Error(String),
}
```

## 变体说明

| 变体 | 说明 |
|------|------|
| `Ready` | 特征执行就绪中，尚未完成。 |
| `Ok(Box<dyn SkillFeatureResultData>)` | 特征执行成功，附带结构化数据。 |
| `Error(String)` | 特征执行失败，附带错误消息。 |

## 使用示例

```rust
// 记录成功结果
ctx.record_feature_result("cooldown", SkillFeatureResult::Ok(Box::new(CooldownData { status: "ready".into() })));

// 记录失败结果
ctx.record_feature_result("damage", SkillFeatureResult::Error("target out of range".into()));

// 查询结果
for (id, result) in &ctx.feature_results {
    match result {
        SkillFeatureResult::Ready => info!("feature '{id}' is still in progress"),
        SkillFeatureResult::Ok(data) => info!("feature '{id}' succeeded"),
        SkillFeatureResult::Error(msg) => warn!("feature '{id}' failed: {msg}"),
    }
}

// 检查是否全部成功
let all_ok = ctx.feature_results.values().all(|r| matches!(r, SkillFeatureResult::Ok(_)));
```

## 与现有模块的关系

- **[`SkillRunContext`]**：[`SkillRunContext.feature_results`] 中的值类型为 `SkillFeatureResult` 枚举，可通过模式匹配统一查询执行状态。
- **[`CooldownFeature`]**：冷却特征完成时，记录 `SkillFeatureResult::Ok(Box::new(CooldownData { ... }))` 到父实体的 [`SkillRunContext`] 中。

[`SkillRunContext`]: ./SkillRunContext.md
[`SkillRunContext.feature_results`]: ./SkillRunContext.md#字段说明
[`SkillFeatureResultData`]: ./SkillFeatureResultData.md
[`CooldownFeature`]: ./cooldown/CooldownFeature.md
