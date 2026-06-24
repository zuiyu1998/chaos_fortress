# StatePlugin

`StatePlugin` 是一个插件，实现了 Bevy 的 [`Plugin`] trait，用于集中初始化游戏的状态系统。

## 定义

```rust
/// Plugin that registers [`Pause`], [`Finish`], [`InGame`], and configures
/// [`PausableSystems`] to only run when not paused.
pub(super) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Pause>();
        app.init_state::<Finish>();
        app.init_state::<InGame>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}
```

## 功能

`StatePlugin::build` 执行以下初始化工作：

| 操作 | 说明 |
|------|------|
| `init_state::<Pause>()` | 初始化 [`Pause`](./Pause.md) 状态，默认值为 `Pause(false)`（未暂停） |
| `init_state::<Finish>()` | 初始化 [`Finish`](./Finish.md) 状态，默认值为 `Finish(false)`（未结算） |
| `init_state::<InGame>()` | 初始化 [`InGame`](./InGame.md) 状态，默认值为 `InGame::None`（不在对局中） |
| `configure_sets` | 配置 [`PausableSystems`](./PausableSystems.md) 系统集仅在 `Pause(false)` 时运行 |

## 使用该插件的模块

| 模块 | 用途 |
|------|------|
| `src/main.rs` | 在 `AppPlugin` 的 `add_plugins` 中注册 `state::StatePlugin`，替代手动初始化 |

## 迁移说明

该插件从 `src/main.rs` 的 `AppPlugin::build` 中提取了暂停与结算状态的初始化逻辑，将其封装到 `state` 模块内，使主入口更简洁，职责更清晰。

## 相关文档

- [`Pause`](./Pause.md)：暂停状态
- [`Finish`](./Finish.md)：游戏结算状态
- [`PausableSystems`](./PausableSystems.md)：受暂停状态控制的系统集
- [`Screen`](./Screen.md)：游戏主画面状态机
- [`InGame`](./InGame.md)：游戏阶段状态机

[`Plugin`]: https://docs.rs/bevy/0.18/bevy/app/trait.Plugin.html
