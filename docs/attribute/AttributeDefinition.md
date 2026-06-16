# AttributeDefinition

`AttributeDefinition` 是属性模块中的**定义层**概念，它描述了某个具名属性的初始配置（名称、默认值、边界）以及它与系统中其他部分的契约。

> 当前属性模块中不存在名为 `AttributeDefinition` 的独立结构体。属性的"定义"通过约定和代码中的硬编码名称隐式表达。本文档记录了这些约定，并提供将属性定义显式化的参考模式。

## 属性定义的组成

一个完整的属性定义包含以下信息：

| 要素 | 说明 | 示例 |
|------|------|------|
| **名称** | 属性的字符串标识符，在 `AttributeSet` 中作为键使用。通常使用小写蛇形命名。 | `"hp"`、`"max_hp"`、`"armor"` |
| **基础值** | 角色/实体初始时该属性的数值。 | `100.0` |
| **最小值** | 属性实际值不允许低于的下限。 | `0.0` |
| **最大值** | 属性实际值不允许超过的上限。 | `f32::MAX` |

## 当前代码中的隐式定义

属性定义目前分散在各个模块的构造代码中。以战斗相关的属性为例：

### BattleAttributeSet 中的属性定义

`BattleAttributeSet` 是一个封装了 `AttributeSet` 的组件，它规定了三个固定的属性名称及其含义：

| 名称 | 含义 | 用途 |
|------|------|------|
| `"hp"` | 当前生命值 | 通过 `BattleAttributeSet::hp()` 访问，死亡判定依据 |
| `"max_hp"` | 最大生命值 | 通过 `BattleAttributeSet::max_hp()` 访问，决定生命值上限 |
| `"armor"` | 护甲值 | 通过 `BattleAttributeSet::armor()` 访问，减免伤害 |

创建时（`BattleAttributeSet::new(max_hp, armor)`）的内部逻辑：

```rust
let mut set = AttributeSet::new();
set.insert("hp", Attribute::new(max_hp));     // 无边界约束
set.insert("max_hp", Attribute::new(max_hp));  // 无边界约束
set.insert("armor", Attribute::new(armor));    // 无边界约束
BattleAttributeSet { attributes: set }
```

> 注意：当前 `Attribute::new(base)` 将 `min` 设为 `f32::MIN`、`max` 设为 `f32::MAX`，即**无边界约束**。实际边界需要在业务代码中手动通过 `set_max`/`set_min`（或直接字段赋值）施加。

## 命名约定

系统中的属性名称遵循以下约定：

| 模式 | 示例 | 说明 |
|------|------|------|
| 小写缩写 | `"hp"` | 常用缩写使用全小写 |
| 小写蛇形 | `"max_hp"`、`"attack_speed"` | 复合词使用下划线分隔 |
| 无命名空间 | `"hp"` vs `"max_hp"` | 前缀/后缀 `max_` 区分同族属性 |

## 与现有模块的关系

- **`Attribute`**：每种定义对应一个 `Attribute` 实例。定义规定其 `base`、`min`、`max`。
- **`AttributeSet`**：属性定义通过 `AttributeSet::insert(name, attr)` 注册到集合中。
- **`BattleAttributeSet`**：封装了对一组固定属性定义（`hp`、`max_hp`、`armor`）的便捷访问方法，是属性定义在战斗场景中的具体应用。
- **`battle` 函数**：`src/battle/mod.rs` 中的 `battle(attributes: AttributeSet) -> impl Bundle` 工厂函数接收已定义好的 `AttributeSet`，将其包装为 `BattleAttributeSet` 并生成 `BattleState`。

## 最佳实践

1. **统一入口**：将常用属性定义集中在一个地方（如常量或工厂函数），避免散落在各处重复硬编码。
2. **显式边界**：创建属性时根据业务需要明确设置 `min` 和 `max`（例如 `hp` 的 `min` 应设为 `0.0`）。
3. **名称一致性**：同一类属性（如战斗属性）在整个项目中保持名称一致，便于跨模块查询和修改。
