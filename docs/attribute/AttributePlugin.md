# AttributePlugin

`AttributePlugin` 是 Bevy 插件，负责将属性模块相关的类型注册到 Bevy 的类型注册表中，以支持反射、场景序列化和编辑器集成。

## 用途

- 注册 `Attribute` 组件类型，使其可被 Bevy 的反射系统识别。
- 启用类型的内省（introspection）、动态访问和编辑器展示。

## 定义

```rust
/// Plugin that registers [`Attribute`] with Bevy's type registry.
pub(super) struct AttributePlugin;

impl Plugin for AttributePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attribute>();
    }
}
```

## 注册的类型

| 类型 | 说明 |
|------|------|
| `Attribute` | 带边界约束的属性值组件，注册后在编辑器中可见，支持反射访问。 |

## 使用方式

在应用的主 `App` 中添加此插件即可：

```rust
app.add_plugins(AttributePlugin);
```

## 与现有模块的关系

- **主程序**：`AttributePlugin` 通常在 `App` 构建阶段被添加，是模块的入口点。
- **`Attribute`**：插件注册的核心组件，确保其 `Reflect` derive 生成的信息被 Bevy 收录。
- 若模块后续扩展新的反射类型（如 `AttributeModifier`），可在此插件中添加对应的 `register_type` 调用。
