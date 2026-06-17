# SkillFeatureResult

`SkillFeatureResult` 是一个 trait，用于表示 SkillFeature 执行后的**结果状态**。

## 用途

- 实现该 trait 的类型可以表示一个 SkillFeature 执行是否成功。
- 提供统一的结果查询接口，用于判断特征执行结果是否正常。
- 配合 [`SkillRunContext`] 的 `feature_results` 使用，判断各特征执行状态。

## 定义

```rust
/// SkillFeature 执行结果。
pub trait SkillFeatureResult: std::fmt::Debug + Send + Sync {
    /// 返回结果是否正常（成功）。
    fn is_ok(&self) -> bool;
}
```

> `std::fmt::Debug + Send + Sync` 约束确保结果类型可以安全地通过 trait 对象 `Box<dyn SkillFeatureResult>` 跨系统传递。

## 方法

| 方法 | 说明 |
|------|------|
| `is_ok() -> bool` | 返回 `true` 表示结果正常（执行成功），`false` 表示执行失败。 |

## 实现示例

### 自定义结果类型

```rust
/// 自定义特征执行结果。
struct FeatureOutcome {
    spawned_entity: Option<Entity>,
    error_message: Option<String>,
}

impl SkillFeatureResult for FeatureOutcome {
    fn is_ok(&self) -> bool {
        self.spawned_entity.is_some()
    }
}
```

## 与现有模块的关系

- **[`SkillRunContext`]**：[`SkillRunContext.feature_results`] 中的值类型实现了 `SkillFeatureResult` trait，可通过 `is_ok()` 统一查询执行状态。

[`SkillRunContext`]: ./SkillRunContext.md
[`SkillRunContext.feature_results`]: ./SkillRunContext.md#字段说明
