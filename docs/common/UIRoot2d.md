# UIRoot2d

`UIRoot2d` 是一个标记组件（Marker Component），用于标识 2D UI 树的根节点。

## 用途

- 持有 `UIRoot2d` 组件的实体是游戏内 2D UI 的入口节点。
- 配合 bevy_lunex 的 `UiLayoutRoot` 和 `UiFetchFromCamera` 使用，将 UI 树与摄像机视口绑定。
- 便于其他系统在查询中快速筛选出 UI 根节点，而无需依赖 `UiLayoutRoot` 的具体实现细节。

## 定义

```rust
/// 2D UI 根节点标记。
#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct UIRoot2d;
```

## 注册

在 `CommonPlugin` 中注册类型以实现反射：

```rust
app.register_type::<UIRoot2d>();
```

## 使用示例

```rust
// 在 level 中生成 UI 根节点
commands.spawn((
    Name::new("UI Root"),
    UIRoot2d,
    UiLayoutRoot::new_2d(),
    UiFetchFromCamera::<0>,
));
```

## 与现有模块的关系

- **level 模块**：`spawn_level` 中生成该组件所在的 UI 根节点实体，作为菜单、HUD 等 UI 元素的父节点。
- **bevy_lunex**：`UIRoot2d` 本身不包含布局逻辑，由 `UiLayoutRoot` 计算布局、`UiFetchFromCamera` 同步视口。
- **查询便利性**：系统可通过 `Query<&UIRoot2d>` 快速定位 UI 根节点，用于动态添加子 UI 元素等场景。
