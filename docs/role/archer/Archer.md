# Archer

`Archer` 是一个组件（Component），用于标记一个实体为"弓箭手"。

## 用途

- 持有 `Archer` 组件的实体代表一个弓箭手单位，拥有远程攻击能力。
- 每个弓箭手可配置独立的攻击范围、射速和伤害属性。
- 通过 `Archer` 组件可在 ECS 查询中快速筛选所有弓箭手实体。

## 与 Role 的关系

`Archer` 是 `Role` 的一个具体变体。弓箭手实体同时携带 `Role` 标记和 `Archer` 标记：

- `Role`：标记该实体为可操控角色单位。
- `Archer`：标记该实体为弓箭手，赋予远程攻击能力。

这使得 ECS 查询可以按需选择粒度：

```rust
// 查询所有角色
Query<&Transform, With<Role>>

// 查询所有弓箭手
Query<&Transform, (With<Role>, With<Archer>)>
```

## 建议的属性组件

`Archer` 组件本身为标记组件，实际属性建议通过以下附加组件表达：

| 组件 | 字段 | 说明 |
|---|---|---|
| `AttackRange` | `f32` | 弓箭手的攻击范围（像素） |
| `AttackSpeed` | `f32` | 攻击间隔（秒） |
| `ProjectileDamage` | `f32` | 投射物伤害值 |
