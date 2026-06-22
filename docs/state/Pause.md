# Pause

`Pause` 是一个结构体，实现了 Bevy 的 [`States`] trait，用于标记游戏是否处于**暂停**状态。

## 定义

```rust
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Pause(pub bool);
```

## 取值说明

| 取值 | 含义 |
|------|------|
| `Pause(false)`（默认） | 游戏正常运行，所有逻辑系统可继续执行 |
| `Pause(true)` | 游戏暂停，配置了 `PausableSystems` 集合的系统被跳过 |

游戏启动时默认处于 `Pause(false)` 状态。`Default` 派生的默认值即为 `false`。

## 使用该状态的模块

| 模块 | 用途 |
|------|------|
| `src/main.rs` | 调用 `app.init_state::<Pause>()` 初始化暂停状态；配置 `PausableSystems` 集合只在 `Pause(false)` 时运行 |
| `src/screens/gameplay.rs` | 按下 Escape 时设置 `Pause(true)` 暂停游戏；退出游戏时通过 `unpause` 系统恢复 `Pause(false)` |
| `src/role/archer.rs` | 弓箭手 AI 系统使用 `in_state(Pause(false))` 条件，确保暂停时不会执行 AI 逻辑 |

## 与 `Menu::Pause` 的关系

`Pause` 状态与 [`Menu::Pause`](./Menu.md) 协同工作，但关注点不同：

- `Pause(bool)` — 控制**游戏逻辑**的启停（ECS 系统级别）
- `Menu::Pause` — 控制**暂停菜单 UI** 的显示与隐藏（UI 系统级别）

典型交互流程：

1. 玩家在游戏中按下 Escape
2. `Pause(true)` — 暂停游戏逻辑（AI、移动等停止）
3. `Menu::Pause` — 显示暂停菜单 UI
4. 玩家点击"继续"
5. `Pause(false)` — 恢复游戏逻辑
6. `Menu::None` — 隐藏暂停菜单 UI

## 相关文档

- [`PausableSystems`](./PausableSystems.md)：受暂停状态控制的系统集
- [`Menu`](./Menu.md)：菜单状态机，暂停菜单是其子状态之一
- [`Screen`](./Screen.md)：游戏主画面状态机

[`States`]: https://docs.rs/bevy/0.18/bevy/ecs/schedule/trait.States.html
