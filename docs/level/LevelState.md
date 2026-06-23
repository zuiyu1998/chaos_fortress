# LevelState

`LevelState` 是一个资源（Resource），用于存储当前关卡的**运行时状态数据**，包括单位的金钱等。

## 用途

- 持有 `LevelState` 资源的系统可以读取或修改当前关卡的金钱数量。
- 在关卡初始化时自动创建，默认初始金钱为 `100`。
- 可用于商店购买、单位升级、波次奖励等经济系统的数据基础。

## 定义

```rust
/// 关卡运行时数据资源。
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelState {
    /// 当前金钱数量。
    pub money: u32,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `money` | `u32` | 当前金钱数量，默认初始值为 `100`。 |

## 默认值

`LevelState` 实现了 `Default`，初始值为：

```rust
LevelState { money: 100 }
```

## 注册方式

在 `LevelPlugin` 中通过 `app.init_resource::<LevelState>()` 注册，确保系统启动时自动初始化。

## 与现有模块的关系

- **关卡模块**（`level`）：`LevelState` 定义在 `level` 模块中，由 `LevelPlugin` 注册和初始化。
- **战斗模块**（`battle`）：战斗系统可通过 `ResMut<LevelState>` 访问金钱数据，用于击杀奖励等逻辑。
- **UI 模块**：UI 系统可读取 `LevelState` 显示当前金钱数量。
