# BattleAttributeSet

`BattleAttributeSet` 是 [`AttributeSet`] 的战斗场景包装，为战斗实体提供基于属性系统的血量、护甲等操作接口。

## 用途

- 包装 `AttributeSet`，将其中的命名属性（如 `"hp"`、`"max_hp"`、`"armor"`）映射为战斗操作。
- 提供 `take_damage`、`is_dead` 等战斗专用方法，内部委托给对应的 `Attribute`。
- 与 `AttributeModifier` 体系兼容，支持通过修饰器临时影响战斗属性。

## 定义

```rust
/// [`AttributeSet`] 的战斗场景包装。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct BattleAttributeSet {
    /// 被包装的属性集。
    pub attributes: AttributeSet,
}
```

## 方法

```rust
impl BattleAttributeSet {
    /// 创建一个新的 BattleAttributeSet，包含指定的血量、最大血量和护甲属性。
    pub fn new(max_hp: f32, armor: f32) -> Self {
        let mut set = AttributeSet::new();
        set.insert("hp", Attribute::new(max_hp));
        set.insert("max_hp", Attribute::new(max_hp));
        set.insert("armor", Attribute::new(armor));
        Self { attributes: set }
    }

    /// 当前血量。
    pub fn hp(&self) -> f32 {
        self.attributes
            .get("hp")
            .map(|a| a.value)
            .unwrap_or(0.0)
    }

    /// 最大血量。
    pub fn max_hp(&self) -> f32 {
        self.attributes
            .get("max_hp")
            .map(|a| a.value)
            .unwrap_or(0.0)
    }

    /// 护甲值。
    pub fn armor(&self) -> f32 {
        self.attributes
            .get("armor")
            .map(|a| a.value)
            .unwrap_or(0.0)
    }

    /// 如果 hp ≤ 0 返回 `true`，表示实体已死亡。
    pub fn is_dead(&self) -> bool {
        self.hp() <= 0.0
    }

    /// 承受原始伤害，经护甲减伤后作用于血量。
    ///
    /// 伤害计算：`effective_damage = max(raw_damage - armor, 0)`。
    pub fn take_damage(&mut self, raw_damage: f32) {
        let effective = (raw_damage - self.armor()).max(0.0);
        if let Some(hp) = self.attributes.get_mut("hp") {
            hp.set_value(hp.value - effective);
        }
    }

    /// 获取底层 `AttributeSet` 的可变引用，用于直接操作属性或添加修饰器。
    pub fn attributes_mut(&mut self) -> &mut AttributeSet {
        &mut self.attributes
    }
}
```

## 示例

```rust
let mut bat = BattleAttributeSet::new(100.0, 20.0);
assert_eq!(bat.hp(), 100.0);
assert!(!bat.is_dead());

bat.take_damage(50.0);
// effective_damage = max(50 - 20, 0) = 30
assert_eq!(bat.hp(), 70.0);

// 通过底层 AttributeSet 添加修饰器
let attrs = bat.attributes_mut();
if let Some(hp) = attrs.get_mut("hp") {
    hp.add_modifier(AttributeModifier {
        id: "heal_buff".to_string(),
        tag_id: "skill".to_string(),
        kind: ModifierKind::Flat(20.0),
    });
}
assert_eq!(bat.hp(), 90.0);
```

## 与现有模块的关系

- **[`AttributeSet`]**：`BattleAttributeSet` 是其包装，所有战斗属性通过命名属性存储在其中。
- **[`Attribute`]**：每个战斗属性（hp、max_hp、armor）都是一个独立的 `Attribute` 对象，支持修饰器。
- **[`AttributeModifier`]**：可通过修饰器临时影响战斗属性（如 Buff 回血、Debuff 降甲）。
- **[`BattleState`]**：`BattleAttributeSet` 可替代 `BattleState`，提供相同的战斗接口但更灵活。

[`AttributeSet`]: ../attribute/AttributeSet.md
[`Attribute`]: ../attribute/Attribute.md
[`AttributeModifier`]: ../attribute/AttributeModifier.md
[`BattleState`]: ./BattleState.md
