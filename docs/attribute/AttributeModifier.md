# AttributeModifier

`AttributeModifier` 是一个修饰值对象，用于对 `Attribute` 进行数值修正（如 Buff/DeBuff、装备加成、技能效果等）。

## 用途

- 为 `Attribute` 提供灵活的值修正能力，无需直接修改 `base` 或 `value`。
- 支持三种修饰类型，覆盖常见的数值调整需求。
- 多个 `AttributeModifier` 可叠加作用于同一个 `Attribute`，按类型分别计算。

## 类型

`AttributeModifier` 包含唯一 `id`、标签 `tag_id` 和三种修正类型（通过 `ModifierKind` 区分）：

| 类型 | 标识 | 说明 |
|------|------|------|
| **数值** | `Flat` | 在属性值上直接加减固定数值，例如 `+50` 生命值、`-10` 攻击力。 |
| **百分比** | `Percent` | 按基础值的百分比进行加减，例如 `+10%` 攻击力、`-5%` 防御力。 |
| **覆盖** | `Override` | 直接将属性值设为指定数值，忽略其他修饰效果。 |

## 定义

### AttributeModifier

```rust
/// 对 Attribute 的修饰方式。
#[derive(Clone, Debug, PartialEq)]
pub struct AttributeModifier {
    /// 唯一标识符，用于区分和去重修饰器。
    pub id: String,
    /// 标签 ID，用于分组或标记修饰器来源（如装备、技能、Buff 等）。
    pub tag_id: String,
    /// 修饰类型。
    pub kind: ModifierKind,
}
```

### ModifierKind

```rust
/// 修饰类型。
#[derive(Clone, Debug, PartialEq)]
pub enum ModifierKind {
    /// 数值类型：直接加减固定数值。
    Flat(f32),
    /// 百分比类型：按属性基础值的百分比进行修正（例如 0.1 表示 +10%）。
    Percent(f32),
    /// 覆盖类型：直接将属性值设为指定值。
    Override(f32),
}
```

## 计算规则

多个 `AttributeModifier` 作用于同一属性时，按以下顺序计算：

1. **先计算所有 `Flat` 修正**：`value = base + Σflat`
2. **再计算所有 `Percent` 修正**：`value = value × (1 + Σpercent)`
3. **最后检查 `Override`**：若存在覆盖修饰，则 `value = override_value`

> 当存在多个 `Override` 时，以最后一个生效的覆盖值为准。

## 示例

| 基础值 | 修饰器 | 计算过程 | 最终值 |
|--------|--------|----------|--------|
| 100 | `Flat(50)` | 100 + 50 | 150 |
| 100 | `Percent(0.1)` | 100 × (1 + 0.1) | 110 |
| 100 | `Flat(20)`, `Percent(0.1)` | (100 + 20) × 1.1 | 132 |
| 100 | `Override(999)` | 直接设为 999 | 999 |
| 100 | `Flat(30)`, `Override(200)` | 覆盖忽略其他修饰 | 200 |
