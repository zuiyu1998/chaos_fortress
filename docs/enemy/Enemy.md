# Enemy

`Enemy` 是一个组件（Component），用于标记一个实体为"敌人"。

## 用途

- 持有 `Enemy` 组件的实体代表一个敌方单位。
- 每个敌人占据一个地图格子，拥有独立的属性、技能和 AI 行为。
- 通过 `Enemy` 组件可在 ECS 查询中快速筛选所有敌人实体。

## 与敌人系统的关系

`Enemy` 组件是 [`EnemySystem.md`](./EnemySystem.md) 中定义的敌人数据模型的 ECS 实现载体。敌人的属性、AI 模式和掉落数据通过附加的组件表达，行为由对应的 System 驱动。
