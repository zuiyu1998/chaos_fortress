# Shop

`Shop` 是一个结构体，实现了 Bevy 的 [`States`] trait，用于标记**商店 UI** 的打开与关闭状态。

## 定义

```rust
/// 商店 UI 是否打开。
///
/// 控制商店面板的显示与交互。与 [`Pause`](crate::state::Pause) 类似，
/// 用于状态驱动的 UI 管理。
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Shop(pub bool);
```

## 取值说明

| 取值 | 含义 |
|------|------|
| `Shop(false)`（默认） | 商店关闭，商店面板隐藏 |
| `Shop(true)` | 商店打开，商店面板可见且可交互 |

游戏启动时默认处于 `Shop(false)` 状态。`Default` 派生的默认值即为 `false`。

## 使用该状态的模块

| 模块 | 用途 |
|------|------|
| `src/shop/mod.rs` | 在 `ShopPlugin` 中调用 `app.init_state::<Shop>()` 初始化；在 `Update` 阶段读取商店状态以控制 UI 显示 |
| `src/level.rs` | 玩家点击"Shop"按钮时设置 `Shop(true)` 打开商店；关闭时恢复 `Shop(false)` |
| （UI 系统） | 商店 UI 渲染系统使用 `in_state(Shop(true))` 条件，确保只在商店打开时更新道具列表 |

## 与 `Pause` 的对比

`Shop` 状态与 [`Pause`](../state/Pause.md) 结构相同，但关注点不同：

- `Shop(bool)` — 控制**商店 UI** 的开关（UI 系统级别），仅管理商店面板的显示状态
- `Pause(bool)` — 控制**游戏逻辑**的启停（ECS 系统级别），影响所有配置了 `PausableSystems` 集合的系统

典型交互流程：

1. 玩家在游戏中点击"Shop"按钮
2. `Shop(true)` — 打开商店面板（覆盖层 UI 显示）
3. 玩家浏览道具、进行购买
4. 玩家点击关闭按钮或再次点击"Shop"按钮
5. `Shop(false)` — 关闭商店面板

## 设计说明

`Shop` 采用与 [`Pause`](../state/Pause.md) 相同的 `newtype bool` 模式：

- **简单直观**：仅需表示开/关两种状态，无需复杂的枚举
- **与 Bevy States 集成**：可直接用于 `.run_if(in_state(Shop(true)))` 等条件配置
- **类型安全**：不同于使用裸 `bool` 资源，状态变化会触发 Bevy 的调度事件

## 相关文档

- [`RoleShopItem`](./RoleShopItem.md)：备战区角色生成组件，购买角色后挂载到备战区格子实体上
- [`ShopItem`](./ShopItem.md)：商店出售道具的定义
- [`ShopItems`](./ShopItem.md#预期容器)：商店道具列表资源
- [`Pause`](../state/Pause.md)：暂停状态，`Shop` 的设计参考
- [`PausableSystems`](../state/PausableSystems.md)：受暂停状态控制的系统集

[`States`]: https://docs.rs/bevy/0.18/bevy/ecs/schedule/trait.States.html
