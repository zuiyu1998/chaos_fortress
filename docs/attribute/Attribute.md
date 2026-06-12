# Attribute

`Attribute` 是一个属性值对象，用于表示实体带有边界约束的数值属性（如生命值、攻击力、防御力等）。

## 用途

- 持有 `Attribute` 的实体拥有一个带上下界的数值属性。
- 提供 `base`（基础值）和 `value`（实际值）的分离，方便实现 Buff/DeBuff 等临时效果。
- 实际值始终被限制在 `[min, max]` 范围内，避免溢出。

## 定义

```rust
/// 带边界约束的属性值。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Attribute {
    /// 基础值（不受临时效果影响的原始值）。
    pub base: f32,
    /// 实际值（受临时效果影响后的当前值）。
    pub value: f32,
    /// 最大值上限。
    pub max: f32,
    /// 最小值下限。
    pub min: f32,
    /// 作用于该属性的修饰器列表。
    pub modifiers: Vec<AttributeModifier>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `base` | `f32` | 基础值，不受临时效果影响。例如角色的初始生命值。 |
| `value` | `f32` | 实际值，经 Buff/DeBuff 等修正后的当前值，始终在 `[min, max]` 范围内。 |
| `max` | `f32` | 最大值上限，`value` 不允许超过此值。 |
| `min` | `f32` | 最小值下限，`value` 不允许低于此值。 |
| `modifiers` | `Vec<AttributeModifier>` | 作用于该属性的修饰器列表，按类型叠加计算影响 `value`。 |

## 辅助方法

```rust
impl Attribute {
    /// 创建一个新属性，基础值和实际值均为 `base`，最大值为 `f32::MAX`，最小值为 `f32::MIN`。
    pub fn new(base: f32) -> Self {
        Self {
            base,
            value: base,
            max: f32::MAX,
            min: f32::MIN,
            modifiers: Vec::new(),
        }
    }

    /// 设置实际值，自动钳制到 `[min, max]` 范围内。
    pub fn set_value(&mut self, new_value: f32) {
        self.value = new_value.clamp(self.min, self.max);
    }

    /// 将实际值重置为基础值。
    pub fn reset(&mut self) {
        self.value = self.base;
    }

    /// 添加一个修饰器到 `modifiers` 列表中。若 `id` 已存在则忽略，添加后自动调用 `recalculate`。
    pub fn add_modifier(&mut self, modifier: AttributeModifier) {
        if self.modifiers.iter().any(|m| m.id == modifier.id) {
            return;
        }
        self.modifiers.push(modifier);
        self.recalculate();
    }

    /// 根据 `id` 删除修饰器，删除成功（存在匹配项）后自动调用 `recalculate`。
    pub fn remove_modifier(&mut self, id: &str) {
        let len_before = self.modifiers.len();
        self.modifiers.retain(|m| m.id != id);
        if self.modifiers.len() < len_before {
            self.recalculate();
        }
    }

    /// 根据 `modifiers` 重新计算 `value`，计算规则：先 Flat、再 Percent、最后 Override。
    pub fn recalculate(&mut self) {
        let mut value = self.base;

        // 1) Flat —— 累加所有数值修正
        for m in &self.modifiers {
            if let ModifierKind::Flat(amount) = &m.kind {
                value += amount;
            }
        }

        // 2) Percent —— 累加百分比，统一乘算
        let mut percent_sum = 0.0f32;
        for m in &self.modifiers {
            if let ModifierKind::Percent(ratio) = &m.kind {
                percent_sum += ratio;
            }
        }
        value *= 1.0 + percent_sum;

        // 3) Override —— 若有覆盖修饰，取最后一个
        for m in &self.modifiers {
            if let ModifierKind::Override(val) = &m.kind {
                value = *val;
            }
        }

        self.value = value;
    }
}
```

## 与现有模块的关系

- **战斗系统**：可用于 `BattleState` 中的血量/护盾等数值，在受到伤害时通过 `set_value` 更新。
- **角色模块**：`RoleBuilder` 可在构建实体时初始化 `Attribute`，例如设置最大生命值、攻击力等。
- **Buff 系统**：临时效果可修改实际值（`value`），持续时间结束后通过 `reset()` 恢复基础值。
