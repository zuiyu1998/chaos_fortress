# AttributeTemplate

`AttributeTemplate` 是一个 **Bevy `Resource`**，用于在全局范围内集中定义和管理一组具名属性的**模板定义**（每个模板定义即一个 `AttributeDefinition`）。它提供了一种声明式的方式来预设实体的属性初始值、边界和修饰器，避免在构造代码中重复硬编码。

> 当前代码中尚未实现 `AttributeTemplate` 与 `AttributeDefinition` 结构体。本文档描述了预期的设计和使用方式，供后续实现参考。

## 定义

```rust
/// 单个属性的模板定义：描述一个属性的初始配置。
pub struct AttributeDefinition {
    /// 属性名称（在 `AttributeSet` 中作为键使用）。
    pub name: String,
    /// 基础值。
    pub base: f32,
    /// 最小值下限。
    pub min: f32,
    /// 最大值上限。
    pub max: f32,
}

/// 全局属性模板资源，按名称索引多个属性定义。
#[derive(Resource)]
pub struct AttributeTemplate {
    /// 以属性名称为键的模板映射。
    pub definitions: HashMap<String, AttributeDefinition>,
}
```

## 字段说明

### AttributeDefinition

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `String` | 属性标识符，与 `AttributeSet` 中的键一致（如 `"hp"`、`"max_hp"`、`"armor"`） |
| `base` | `f32` | 属性的初始基础值 |
| `min` | `f32` | 属性实际值不允许低于的下限（如生命值应为 `0.0`） |
| `max` | `f32` | 属性实际值不允许超过的上限（如生命值可设为最大生命值） |

### AttributeTemplate

| 字段 | 类型 | 说明 |
|------|------|------|
| `definitions` | `HashMap<String, AttributeDefinition>` | 按名称索引的全部属性模板，支持快速查找和遍历 |

## 使用示例

### 注册模板并构建 AttributeSet

```rust
// 插入模板资源
let mut template = AttributeTemplate::new();
template.define(AttributeDefinition {
    name: "hp".into(),
    base: 100.0,
    min: 0.0,
    max: 100.0,
});
template.define(AttributeDefinition {
    name: "max_hp".into(),
    base: 100.0,
    min: 0.0,
    max: f32::MAX,
});
template.define(AttributeDefinition {
    name: "armor".into(),
    base: 30.0,
    min: 0.0,
    max: f32::MAX,
});
app.insert_resource(template);
```

```rust
/// 根据模板名称批量构建 AttributeSet。
pub fn build_attribute_set(template: &AttributeTemplate, names: &[&str]) -> AttributeSet {
    let mut set = AttributeSet::new();
    for name in names {
        if let Some(def) = template.definitions.get(*name) {
            let mut attr = Attribute::new(def.base);
            attr.min = def.min;
            attr.max = def.max;
            set.insert(def.name.as_str(), attr);
        }
    }
    set
}
```

### 配合 battle 函数使用

```rust
fn spawn_enemy(mut commands: Commands, template: Res<AttributeTemplate>) {
    let attrs = build_attribute_set(&template, &["hp", "max_hp", "armor"]);
    commands.spawn(enemy(64.0, 0, 0, sprite, attrs));
}
```

## 与现有模块的关系

- **`AttributeDefinition`**：`AttributeTemplate` 是 `AttributeDefinition` 的容器，每个定义描述一个属性的初始配置。
- **`Attribute`**：运行时调用 `Attribute::new(base)` 并手动设置 `min`/`max`，这些值来源于对应的 `AttributeDefinition`。
- **`AttributeSet`**：`AttributeTemplate` 可作为一个蓝图，通过批量构建方法生成 `AttributeSet`。
- **`BattleAttributeSet`**：`BattleAttributeSet::new(max_hp, armor)` 的内部逻辑与从模板构建 `"hp"`、`"max_hp"`、`"armor"` 的效果一致，模板化后可统一管理这些定义。
- **`battle` 函数**：`src/battle/mod.rs` 中的 `battle(attributes: AttributeSet) -> impl Bundle` 工厂函数接收从模板构建的 `AttributeSet`，生成 `BattleState` 和 `BattleAttributeSet`。
- **`AttributePlugin`**：`AttributePlugin` 负责在 `App` 中注册类型和资源，应在其中添加 `app.insert_resource(AttributeTemplate::new())` 并开放扩展方法。

## 最佳实践

1. **在插件中初始化**：推荐在 `AttributePlugin::build` 中插入 `AttributeTemplate` 默认实例，允许其他模块通过 `define` 方法扩展。
2. **按角色/实体类型分组**：不同实体（如敌人、弓箭手）可定义不同的模板子集，避免全局模板过度膨胀。
3. **配合常量使用**：将属性名称定义为常量（如 `const HP: &str = "hp"`），减少字符串硬编码带来的拼写错误。
4. **与反射集成**：若需编辑器可视化，可为 `AttributeDefinition` derive `Reflect` 并在 `AttributePlugin` 中注册。
