# MapData

`MapData` 是一个**资源（Resource）**，用于构建 `Map` 实体下的子实体。

## 字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `width` | `u32` | 地图横向格子数（宽），备战区 2 列 + 交战区 10 列 = 12 列 |
| `height` | `u32` | 地图纵向格子数（高），共 5 行 |
| `cell_size` | `f32` | 每个格子的像素尺寸（正方形边长），默认 64.0 |

## 默认值

```rust
impl Default for MapData {
    fn default() -> Self {
        Self {
            width: 12,
            height: 5,
            cell_size: 64.0,
        }
    }
}
```

## 用途

- `MapData` 作为全局资源，在初始化 `Map` 实体时被读取。
- 根据 `width` × `height` 和 `cell_size`，在地图实体下生成对应的子实体（如地形格子、网格线等）。
- 该资源贯穿整个游戏生命周期，在需要重新生成地图时亦可更新。

## 相关文档

- [`Map`](./Map.md)：地图根实体标记组件
- [`MapSystem`](./MapSystem.md)：地图系统文档（布局、工具函数、地形系统）
