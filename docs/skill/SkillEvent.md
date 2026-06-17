# SkillEvent

`SkillEvent` 是一个消息（Message），实现了 [`bevy_gearbox`] 的 [`Message`] trait，用于在技能执行完成后广播结果信息。其内容与 [`SkillRunContext`] 相似，可从 [`SkillRunContext`] 生成实例。

## 用途

- 在技能执行流程的末尾，将 [`SkillRunContext`] 中的结果数据以消息形式发出，供其他系统响应。
- 解耦技能执行系统与后续效果系统（如伤害结算、动画播放、音效触发），后者只需监听 `SkillEvent` 即可做出反应。
- 与 [`SkillRunContext`] 配合使用：`SkillRunContext` 是组件，附着在技能实体上；`SkillEvent` 是消息，跨系统广播。

## 定义

```rust
use std::collections::HashMap;

use bevy::prelude::*;
use bevy_gearbox::prelude::*;

use crate::skill::SkillFeatureResult;

/// 技能执行完成时广播的消息，包含执行结果数据。
#[derive(Message, Clone, TypePath)]
pub struct SkillEvent {
    /// 执行完毕的技能实体。
    pub skill: Entity,
    /// 技能所属的实体（技能持有者）。
    pub owner: Entity,
    /// SkillFeature 执行结果记录，键为特征 id，值为 [`SkillFeatureResult`] 枚举。
    pub feature_results: HashMap<String, SkillFeatureResult>,
}

impl From<(Entity, SkillRunContext)> for SkillEvent {
    fn from((skill, ctx): (Entity, SkillRunContext)) -> Self {
        Self {
            skill,
            owner: ctx.owner,
            feature_results: ctx.feature_results,
        }
    }
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `skill` | `Entity` | 执行完毕的技能实体。 |
| `owner` | `Entity` | 技能所属的实体（技能持有者），即拥有该技能的实体。 |
| `feature_results` | `HashMap<String, SkillFeatureResult>` | SkillFeature 执行结果记录，键为特征 `id`，值为 [`SkillFeatureResult`] 枚举。 |

## 生成与发送

`SkillEvent` 通常在技能执行流程的末尾，从技能实体上的 [`SkillRunContext`] 转换后发出：

```rust
fn emit_skill_event(
    mut ctx_query: Query<(&mut SkillRunContext, Entity)>,
    mut messages: MessageWriter<SkillEvent>,
) {
    for (ctx, skill_entity) in ctx_query.iter() {
        // 检查所有 feature 是否均已执行完毕
        let all_done = ctx.feature_results.values().all(|r| {
            matches!(r, SkillFeatureResult::Ok(_) | SkillFeatureResult::Error(_))
        });
        if all_done {
            messages.send(SkillEvent::from((skill_entity, ctx)));
        }
    }
}
```

## 监听示例

```rust
fn on_skill_completed(mut messages: MessageReader<SkillEvent>) {
    for event in messages.read() {
        info!("Skill completed by {:?}", event.owner);
        for (id, result) in &event.feature_results {
            match result {
                SkillFeatureResult::Ok(data) => info!("  feature '{id}' ok"),
                SkillFeatureResult::Error(msg) => warn!("  feature '{id}' failed: {msg}"),
                SkillFeatureResult::Ready => {} // 不应出现，技能应已全部完成
            }
        }
    }
}
```

## 注册

`SkillEvent` 需在 [`SkillPlugin`] 中通过 `app.add_message::<SkillEvent>()` 注册，才能被 Bevy 的消息系统使用。

## 与现有模块的关系

- **[`SkillRunContext`]**：`SkillEvent` 与 `SkillRunContext` 共享 `owner` + `feature_results` 字段，并额外包含 `skill` 字段记录技能实体。可通过 `From<(Entity, SkillRunContext)>` 转换得到。`SkillRunContext` 是组件，附着在实体上；`SkillEvent` 是消息，跨系统广播。
- **[`SkillFeatureResult`]**：`feature_results` 字典的值类型，记录了每个特征的执行状态与数据。
- **[`SkillPlugin`]**：`SkillEvent` 需在 [`SkillPlugin`] 中通过 `app.add_message::<SkillEvent>()` 注册。
- **[`DeathInBattle`]** 与 **[`BulletBattleEvent`]**：遵循相同的消息设计模式。

[`Message`]: https://docs.rs/bevy_gearbox/latest/bevy_gearbox/gsmt/trait.Message.html
[`SkillRunContext`]: ./SkillRunContext.md
[`SkillFeatureResult`]: ./SkillFeatureResult.md
[`SkillPlugin`]: ./SkillPlugin.md
[`DeathInBattle`]: ../battle/DeathInBattle.md
[`BulletBattleEvent`]: ../bullet/BulletBattleEvent.md
[`bevy_gearbox`]: https://crates.io/crates/bevy_gearbox
