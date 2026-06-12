# AttributeSet

`AttributeSet` 是一个包含所有属性的集合对象，用于统一管理实体上的多个 `Attribute`。

## 用途

- 将实体的所有属性（生命值、攻击力、防御力等）集中管理在一个对象中。
- 提供按键名访问、添加、删除属性的能力。
- 简化批量操作（如遍历全部属性、统一重置、跨属性效果联动）。

## 定义

```rust
/// 包含多个属性的集合。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct AttributeSet {
    /// 以属性名称（如 "hp"、"attack"、"defense"）为键的属性映射。
    pub attributes: HashMap<String, Attribute>,
}
```

## 方法

```rust
impl AttributeSet {
    /// 创建一个空的属性集。
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    /// 添加或替换一个属性。
    pub fn insert(&mut self, name: &str, attribute: Attribute) {
        self.attributes.insert(name.to_string(), attribute);
    }

    /// 根据名称获取属性的不可变引用。
    pub fn get(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }

    /// 根据名称获取属性的可变引用。
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Attribute> {
        self.attributes.get_mut(name)
    }

    /// 删除指定名称的属性，返回被删除的属性（若存在）。
    pub fn remove(&mut self, name: &str) -> Option<Attribute> {
        self.attributes.remove(name)
    }

    /// 遍历所有属性的名称。
    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.attributes.keys()
    }

    /// 遍历所有属性。
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Attribute)> {
        self.attributes.iter()
    }
}
```

## 示例

```rust
let mut set = AttributeSet::new();

// 添加属性
set.insert("hp", Attribute::new(100.0));
set.insert("attack", Attribute::new(50.0));
set.insert("defense", Attribute::new(30.0));

// 访问并修改属性
if let Some(hp) = set.get_mut("hp") {
    hp.set_value(80.0);
}

// 添加修饰器到攻击力
if let Some(attack) = set.get_mut("attack") {
    attack.add_modifier(AttributeModifier {
        id: "buff_01".to_string(),
        tag_id: "skill".to_string(),
        kind: ModifierKind::Percent(0.2),
    });
}

// 遍历所有属性
for (name, attr) in set.iter() {
    println!("{}: {}/{}", name, attr.value, attr.max);
}
```

## 与现有模块的关系

- **`Attribute`**：`AttributeSet` 是 `Attribute` 的容器，每个属性按键名索引。
- **战斗系统**：可通过 `set.get("hp")` 获取生命值属性进行处理。
- **角色模块**：`RoleBuilder` 可以使用 `AttributeSet` 统一构建角色的所有属性。
- **Buff 系统**：通过在对应属性上调用 `add_modifier` / `remove_modifier` 实现效果。
