# BattleState

`BattleState` 是一个组件（Component），用于存储战斗实体的**战斗属性**数据，包括当前血量、最大血量和护甲值。

## 用途

- 持有 `BattleState` 组件的实体代表一个可参与战斗的单位（角色或敌人），拥有血量、护甲等属性。
- 战斗系统通过读取 `BattleState` 组件来计算伤害、判断生死、选择攻击目标。
- 该组件通常与 [`EnemyTarget`]、[`AttackRange`] 等组件配合使用。

## 定义

```rust
/// 战斗实体的战斗属性。
#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct BattleState {
    /// 当前血量。
    pub hp: f32,
    /// 最大血量。
    pub max_hp: f32,
    /// 护甲值，减少受到的伤害。
    pub armor: f32,
}

impl BattleState {
    /// 创建一个新的满血 BattleState。
    pub fn new(max_hp: f32, armor: f32) -> Self { ... }
    /// 如果 hp ≤ 0 返回 true。
    pub fn is_dead(&self) -> bool { ... }
    /// 承受伤害（经护甲减伤后）。
    pub fn take_damage(&mut self, raw_damage: f32) { ... }
    /// 从 BattleAttributeSet 生成 BattleState，提取当前 hp、max_hp、armor。
    pub fn from_attribute_set(set: &BattleAttributeSet) -> Self { ... }
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `hp` | `f32` | 当前血量，降至 0 以下时实体死亡。`Default` 为 `0.0`。 |
| `max_hp` | `f32` | 最大血量，创建时 `hp` 初始化为该值。`Default` 为 `0.0`。 |
| `armor` | `f32` | 护甲值，受到的伤害会先减去护甲值（最小为 0）。`Default` 为 `0.0`。 |

## 方法

| 方法 | 说明 |
|------|------|
| `new(max_hp, armor)` | 创建一个新的 `BattleState`，`hp` 初始化为 `max_hp`（满血）。 |
| `from_attribute_set(set)` | 从 `BattleAttributeSet` 生成 `BattleState`，提取当前 `hp`、`max_hp`、`armor`。 |
| `is_dead() -> bool` | 如果 `hp ≤ 0` 返回 `true`，表示实体已死亡。 |
| `take_damage(raw_damage)` | 承受原始伤害 `raw_damage`，减去 `armor` 后作用于 `hp`（最少为 0）。 |

### take_damage 伤害计算公式

```
effective_damage = max(raw_damage - armor, 0)
hp = max(hp - effective_damage, 0)
```

## 与现有模块的关系

- **战斗模块**（`battle`）：`BattleState` 定义在 `battle` 模块中，由 `BattlePlugin` 注册。战斗系统的伤害计算、生死判断等核心逻辑围绕 `BattleState` 展开。
- **[`BattleAttributeSet`]**：`from_attribute_set` 方法提供从 `BattleAttributeSet` 到 `BattleState` 的转换能力，方便在保留修饰器系统的同时兼容旧组件。
- **角色模块**（`role`）：角色实体在构建时可附加 `BattleState` 组件，用于参与战斗。
- **敌人模块**（`enemy`）：敌人实体同样可通过 `BattleState` 组件拥有血量和护甲。

[`EnemyTarget`]: ../common/EnemyTarget.md
[`AttackRange`]: ../common/AttackRange.md
[`BattleAttributeSet`]: ./BattleAttributeSet.md
