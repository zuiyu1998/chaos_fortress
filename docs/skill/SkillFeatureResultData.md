# SkillFeatureResultData

`SkillFeatureResultData` 是一个 trait，表示 [`SkillFeatureResult`] 中 `Ok` 状态下存储的数据类型标记，用于将结构化数据与执行结果绑定，替代直接使用裸字符串。

## 用途

- 标记 `SkillFeatureResult::Ok` 状态下可存储的数据类型。
- 各 SkillFeature 可以实现此 trait，从而在成功结果中携带自定义的结构化数据。

## 定义

```rust
/// SkillFeatureResult::Ok 状态下存储的数据类型标记。
pub trait SkillFeatureResultData {}
```

## 实现示例

```rust
/// 冷却特征的成功结果数据。
struct CooldownData {
    remaining: Duration,
    skill_id: String,
}

impl SkillFeatureResultData for CooldownData {}
```

## 与现有模块的关系

- **[`SkillFeatureResult`]**：`SkillFeatureResultData` 作为 trait bound，约束 `SkillFeatureResult::Ok` 中可存储的数据类型，参见 `Box<dyn SkillFeatureResultData>`。
- **[`SkillRunContext`]**：结合 `SkillFeatureResultData`，可以从 `SkillRunContext.feature_results` 中提取类型化的成功数据，而不仅是字符串消息。

[`SkillFeatureResult`]: ./SkillFeatureResult.md
[`SkillRunContext`]: ./SkillRunContext.md
