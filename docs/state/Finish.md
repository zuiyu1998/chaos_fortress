# Finish

`Finish` 是一个结构体，实现了 Bevy 的 [`States`] trait，用于标记游戏是否已经**结算**（结束）。

## 定义

```rust
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Finish(pub bool);
```

## 取值说明

| 取值 | 含义 |
|------|------|
| `Finish(false)`（默认） | 游戏正常进行，尚未结算 |
| `Finish(true)` | 游戏已结算（结束），应展示结算画面并停止游戏逻辑 |

游戏启动时默认处于 `Finish(false)` 状态。`Default` 派生的默认值即为 `false`。

## 预期用途

`Finish` 状态用于表示游戏的胜负已经揭晓，例如：

- 敌人进入了基地区域（触发游戏失败）
- 所有敌人被消灭（触发游戏胜利）
- 或其他自定义结算条件达成

当 `Finish(true)` 时，相关系统可切换到结算界面、播放结算动画或展示胜负结果。

## 与 `Pause` 的对比

| 状态 | 含义 | 可逆性 |
|------|------|--------|
| `Pause(bool)` | 游戏是否暂停（临时中断） | 可恢复，`false` → `true` ↔ `false` |
| `Finish(bool)` | 游戏是否结算（终局） | 不可逆，默认 `false` → `true`（单向） |

## 相关文档

- [`Pause`](./Pause.md)：暂停状态，控制游戏逻辑的临时启停
- [`PausableSystems`](./PausableSystems.md)：受暂停控制的系统集
- [`Screen`](./Screen.md)：游戏主画面状态机
- [`Menu`](./Menu.md)：菜单状态机

[`States`]: https://docs.rs/bevy/0.18/bevy/ecs/schedule/trait.States.html
