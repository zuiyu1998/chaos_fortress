# BattlePlugin

`BattlePlugin` 是一个插件对象，实现了 Bevy 的 [`Plugin`] trait，负责注册战斗模块相关的组件。

## 用途

- 向 Bevy 应用注册 [`BattleState`] 组件到类型反射系统（Type Registry）。
- 注册后，该组件可在编辑器中序列化/反序列化，并支持运行时反射访问。

## 定义

```rust
/// 注册战斗相关组件的插件。
pub(super) struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BattleState>();
    }
}
```

## 注册的组件

| 组件 | 说明 |
|------|------|
| [`BattleState`] | 存储战斗实体的战斗属性（血量、护甲等）。 |

## 与现有模块的关系

- **战斗模块**（`battle`）：`BattlePlugin` 是战斗模块的入口插件，由主应用（`AppPlugin`）的插件列表添加。
- **主应用**（`main`）：在 `src/main.rs` 中以 `battle::BattlePlugin` 的形式被添加至 Bevy 应用。
- **角色模块**（`role`）：角色实体在构建时可通过 [`BattleState`] 组件参与战斗。
- **敌人模块**（`enemy`）：敌人实体同样可通过 [`BattleState`] 组件拥有血量和护甲。

[`BattleState`]: ./BattleState.md
[`Plugin`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html
