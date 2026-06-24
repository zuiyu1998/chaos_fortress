# InGame

`InGame` 是一个枚举，实现了 Bevy 的 [`States`] trait，用于标记游戏进行中的**阶段**。

## 定义

```rust
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum InGame {
    /// 不在游戏对局中（默认）。
    #[default]
    None,
    /// 备战阶段 — 准备战斗。
    Preparation,
    /// 战斗阶段 — 正在战斗。
    Battle,
}
```

## 取值说明

| 取值 | 含义 |
|------|------|
| `None`（默认） | 不在游戏对局中，例如处于标题画面或加载界面 |
| `Preparation` | 战斗前的准备阶段，玩家可以部署角色、调整策略 |
| `Battle` | 正在战斗，敌人出现，所有战斗系统活跃 |

## 使用该状态的模块

| 模块 | 用途 |
|------|------|
| `src/state/mod.rs` | 定义 `InGame` 枚举，作为游戏阶段状态机 |

## 相关文档

- [`Screen`](./Screen.md)：游戏主画面状态机
- [`Pause`](./Pause.md)：暂停状态
- [`Finish`](./Finish.md)：游戏结算状态

[`States`]: https://docs.rs/bevy/0.18/bevy/ecs/schedule/trait.States.html
