# Menu

`Menu` 是一个枚举，实现了 Bevy 的 [`States`] trait，用于控制游戏菜单界面之间的切换。

## 定义

```rust
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Main,
    Credits,
    Settings,
    Pause,
}
```

## 状态说明

| 变体 | 说明 |
|------|------|
| `None`（默认） | 不在任何菜单中，游戏正常进行 |
| `Main` | 主菜单，游戏标题画面显示"开始游戏"、"设置"、"致谢"等选项 |
| `Credits` | 致谢画面，展示制作人员名单 |
| `Settings` | 设置画面，可调整游戏选项（如音量） |
| `Pause` | 暂停菜单，在游戏进行时按 Escape 键呼出，提供"继续"、"设置"、"返回主菜单"等选项 |

## 状态转换

```
         ┌─────────────────────────────────┐
         │          Menu::None              │
         │      (不在菜单中，游戏进行)         │
         └──┬──────────────┬───────────────┘
            │              │
            ▼              ▼
      Menu::Main      Menu::Pause
            │              │
       ┌────┼────┐         ├──→ Menu::Settings
       ▼    ▼    ▼         │
   Menu::   Menu::  Menu:  └──→ Menu::None
   Credits  Settings (返回)
```

## 使用该状态的模块

| 模块 | 用途 |
|------|------|
| `src/menus/mod.rs` | 调用 `app.init_state::<Menu>()` 初始化菜单状态机 |
| `src/menus/main.rs` | 在 `OnEnter(Menu::Main)` 时生成主菜单 UI，通过按钮切换到 `Credits`、`Settings` 或开始游戏 |
| `src/menus/credits.rs` | 在 `OnEnter(Menu::Credits)` 时生成致谢画面 |
| `src/menus/settings.rs` | 在 `OnEnter(Menu::Settings)` 时生设置画面，提供返回 `Menu::Main` 或 `Menu::Pause` 的按钮 |
| `src/menus/pause.rs` | 在 `OnEnter(Menu::Pause)` 时生成暂停菜单 UI，提供"继续"（返回 `Menu::None`）、"设置"（`Menu::Settings`）等选项 |
| `src/screens/gameplay.rs` | 按下 Escape 时设置 `Menu::Pause`；进入游戏时调用 `close_menu` 系统关闭菜单（设为 `Menu::None`） |
| `src/screens/title.rs` | 在标题画面选择"开始游戏"时过渡到 `Menu::Main` |

## 与 `Pause` 状态的关系

`Menu::Pause` 和 [`Pause(true)`](./Pause.md) 是两个不同的状态，但有紧密联系：

- `Menu::Pause` 控制**暂停菜单 UI** 的显示与隐藏
- `Pause(bool)` 控制**游戏逻辑**是否继续运行
- 通常情况下按下 Escape 会同时设置 `Menu::Pause` 和 `Pause(true)`，继续游戏时恢复 `Menu::None` 和 `Pause(false)`
- 但两者并非严格绑定——打开设置菜单（`Menu::Settings`）时游戏可能继续或暂停，具体取决于设计

## 相关文档

- [`Screen`](./Screen.md)：游戏主画面状态机
- [`Pause`](./Pause.md)：暂停状态，控制游戏逻辑是否继续运行

[`States`]: https://docs.rs/bevy/0.18/bevy/ecs/schedule/trait.States.html
