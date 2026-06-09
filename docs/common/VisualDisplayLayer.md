# VisualDisplayLayer

`VisualDisplayLayer` 是一个枚举，用于定义游戏中各视觉元素的**渲染层级**（z 轴顺序），确保不同种类的对象按正确的层次关系显示。

## 枚举值

| 枚举值 | 对应 f32 值 | 说明 |
|--------|-------------|------|
| `Terrain` | `0.0` | 地形层，包含地图格子、地形贴图等 |
| `Character` | `1.0` | 人物层，包含角色、敌人等可交互单位 |

## 层级规则

- `Character` 的 f32 值 **大于** `Terrain` 的值，因此角色始终渲染在地形之上。
- 预留数值间隔便于后续在两层之间插入新的中间层（如特效、高亮框等）。

## 用途

在构造实体的 `Transform` 时，使用枚举对应的 f32 值设置 `z` 坐标，以控制渲染顺序。

```
// 示例（伪代码）：
Transform::from_xyz(x, y, VisualDisplayLayer::Terrain as f32)
Transform::from_xyz(x, y, VisualDisplayLayer::Character as f32)
```

## 与现有模块的关系

- **地图模块**：`map_cell` 生成的地图格子使用 `Terrain` 层。
- **角色模块**：`role` 生成的角色精灵使用 `Character` 层。
