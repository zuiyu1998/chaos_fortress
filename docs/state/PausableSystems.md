# PausableSystems

`PausableSystems` 是一个系统集（[`SystemSet`]），用于标记那些**在游戏暂停时不应运行**的 ECS 系统。

## 定义

```rust
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;
```

## 配置

`PausableSystems` 在 `src/main.rs` 的 `AppPlugin` 中配置：

```rust
// 设置暂停状态
app.init_state::<state::Pause>();

// 配置 PausableSystems：仅在游戏未暂停时运行
app.configure_sets(Update, state::PausableSystems.run_if(in_state(state::Pause(false))));
```

## 说明

`PausableSystems` 本身是一个空集合——它不直接包含任何系统。它的作用是为开发者提供一个**标记集合**：

- 所有在暂停时需要停止的系统，应使用 `.in_set(PausableSystems)` 加入此集合
- 当 `Pause(true)` 时，集合内的系统自动被 Bevy 调度器跳过
- 不在此集合中的系统即使在暂停时也会继续运行（如 UI 输入处理、菜单动画等）

## 使用示例

```rust
// 暂停时不应运行的 AI 系统
app.add_systems(Update, enemy_ai_system.in_set(PausableSystems));

// 暂停时仍应运行的 UI 系统（不在集合中）
app.add_systems(Update, menu_animation_system);
```

## 当前应用中属于此集合的系统

以下是当前代码中使用 `PausableSystems` 或等效条件的系统：

| 系统 | 限制方式 | 说明 |
|------|----------|------|
| 弓箭手 AI 系统 | `.run_if(in_state(Pause(false)))` | 暂停时角色不应移动或攻击 |

随着项目发展，所有游戏逻辑系统（AI、移动、碰撞处理、战斗系统等）应逐步加入 `PausableSystems`。

## 相关文档

- [`Pause`](./Pause.md)：控制暂停状态的状态对象
- [`Screen`](./Screen.md)：游戏主画面状态机
- [`Menu`](./Menu.md)：菜单状态机

[`SystemSet`]: https://docs.rs/bevy/0.18/bevy/ecs/schedule/trait.SystemSet.html
