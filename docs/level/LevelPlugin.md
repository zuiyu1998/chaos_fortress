# LevelPlugin

`LevelPlugin` 是一个结构体，实现了 Bevy 的 `Plugin` trait，用于注册关卡模块所需的核心资源。

## 概述

`LevelPlugin` 是关卡模块的入口插件，负责在 App 中注册关卡相关的 ECS 资源。

```rust
pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.load_resource::<LevelAssets>();
        app.init_resource::<LevelState>();
    }
}
```

## 注册内容

| 注册项 | 类型 | 说明 |
|--------|------|------|
| `LevelAssets` | Resource / Asset | 关卡资源（背景音乐等），通过 `load_resource` 从 `FromWorld` 获取 |
| `LevelState` | Resource | 关卡运行时数据（金钱等），通过 `init_resource` 注册并初始化 |

## 使用方式

在 `main.rs` 的 `AppPlugin` 中与其他插件一同添加：

```rust
app.add_plugins((
    // ... 其他插件
    level::LevelPlugin,
));
```

## 插件生命周期

1. **`LevelAssets`**：通过 `FromWorld` 自动加载关卡资源句柄（如音乐）。
2. **`LevelState`**：通过 `Default` 初始化为默认值（`money: 100`），游戏进行中系统可读写该资源。

## 与现有模块的关系

- **关卡模块**（`level`）：`LevelPlugin` 是 `level` 模块的唯一入口，由 `main.rs` 添加。
- **`LevelAssets`**：关卡资源，包含背景音乐等静态数据，在插件注册时加载。
- **`LevelState`**：关卡运行时状态数据，存储当前金钱数量，见 [`LevelState`]。
- **`spawn_level`**：关卡生成系统，在进入 `Screen::Gameplay` 时执行，读取 `LevelAssets` 等资源来构建关卡实体，不属于插件的注册范围。

## 注意事项

- `LevelPlugin` 使用 `pub(super)` 可见性，仅在 `crate` 内部可用。
- 当前插件仅注册资源，不添加任何运行系统。`spawn_level` 系统通过 `OnEnter(Screen::Gameplay)` 在屏幕模块中调度，而非在此插件中直接添加。

[`LevelState`]: ./LevelState.md
