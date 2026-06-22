# Screen

`Screen` 是一个枚举，实现了 Bevy 的 [`States`] trait，用于控制游戏主画面状态之间的切换。

## 定义

```rust
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}
```

## 状态说明

| 变体 | 说明 |
|------|------|
| `Splash`（默认） | 启动画面，应用开始时的初始状态，展示游戏 Logo 后自动跳转到标题画面 |
| `Title` | 标题画面，显示游戏标题和"开始游戏"等菜单选项 |
| `Loading` | 加载画面，在标题画面选择开始后进入，等待所有资源加载完成 |
| `Gameplay` | 游戏主画面，实际游戏进行中的状态 |

## 状态转换

```
Splash (启动)  →  Title (标题)
Title (开始)   →  Loading (加载)
Loading (完成) →  Gameplay (游戏)
Gameplay (结束) → Title (返回标题)
```

## 使用该状态的模块

| 模块 | 用途 |
|------|------|
| `src/screens/mod.rs` | 调用 `app.init_state::<Screen>()` 初始化屏幕状态机 |
| `src/screens/splash.rs` | 在 `OnEnter(Screen::Splash)` 时生成启动画面，完成后过渡到 `Screen::Title` |
| `src/screens/loading.rs` | 在 `OnEnter(Screen::Loading)` 时等待资源加载，完成后过渡到 `Screen::Gameplay` |
| `src/screens/title.rs` | 在 `OnEnter(Screen::Title)` 时生成标题画面 |
| `src/screens/gameplay.rs` | 在 `OnEnter(Screen::Gameplay)` 时生成游戏关卡和 UI，在 `OnExit(Screen::Gameplay)` 时清理 |
| `src/level.rs` | 关卡系统使用 `Screen::Gameplay` 作为 `DespawnOnExit` 的标记 |
| `src/role/archer.rs` | 弓箭手 AI 系统仅在 `Screen::Gameplay` 且非暂停状态下运行 |
| `src/dev_tools/mod.rs` | 开发工具仅在 `Screen::Gameplay` 状态下生效 |

## 相关文档

- [`Menu`](./Menu.md)：菜单状态机，与 `Screen` 配合管理暂停菜单的进出
- [`Pause`](./Pause.md)：暂停状态，控制游戏逻辑是否继续运行
- [`PausableSystems`](./PausableSystems.md)：受暂停控制的系统集

[`States`]: https://docs.rs/bevy/0.18/bevy/ecs/schedule/trait.States.html
