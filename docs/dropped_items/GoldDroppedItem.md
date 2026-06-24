# GoldDroppedItem

`GoldDroppedItem` 是一个组件（Component），用于标识一个实体为**可拾取的金币掉落物**，并存储其包含的金币数量。

## 用途

- 持有 `GoldDroppedItem` 组件的实体代表一个在地图上掉落**金币**的物体，玩家靠近即可拾取。
- 在敌人被击杀时生成，作为掉落系统的一部分。
- 配合碰撞传感器（`Collider` + `Sensor`）检测玩家角色靠近，自动累加到 `LevelState.money` 并销毁实体。

## 定义

```rust
/// 金币掉落物组件，包含金币数量。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct GoldDroppedItem {
    /// 该掉落物包含的金币数量。
    pub amount: u32,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `amount` | `u32` | 该掉落物包含的金币数量。生成时由掉落逻辑决定，拾取后加入 `LevelState.money` |

## 使用示例

### 生成金币掉落物

在敌人死亡时，根据 `DroppedItemTemplate` 的配置生成对应数量的金币掉落物：

```rust
fn spawn_gold_drop(
    mut commands: Commands,
    template: Res<DroppedItemTemplate>,
) {
    // 从模板获取金币定义
    if let Some(gold_def) = template.get("gold") {
        // 在 min..=max 范围取随机值
        let amount = fastrand::u32(gold_def.min_amount..=gold_def.max_amount);
        commands.spawn((
            Name::new(format!("Gold ({amount})")),
            GoldDroppedItem { amount },
            Sprite::from_color(Color::srgb(1.0, 0.84, 0.0), Vec2::splat(24.0)),
            Transform::from_xyz(world_x, world_y, 0.5),
            Collider::circle(16.0),
            Sensor,
            GamePhysicsLayer::character_layers(),
        ));
    }
}
```

### 拾取金币

```rust
fn pickup_gold(
    mut commands: Commands,
    mut started: MessageReader<CollisionStarted>,
    characters: Query<&Role>,
    gold_items: Query<&GoldDroppedItem>,
    mut level_state: ResMut<LevelState>,
) {
    for event in started.read() {
        let (e1, e2) = (event.collider1, event.collider2);
        // 判断碰撞双方是否为一侧为角色、另一侧为金币
        if let Some(gold_entity) = gold_items.get(e1).ok().map(|_| e1)
            .or_else(|| gold_items.get(e2).ok().map(|_| e2))
        {
            if characters.contains(e1) || characters.contains(e2) {
                if let Ok(gold) = gold_items.get(gold_entity) {
                    level_state.money += gold.amount;
                    info!("拾取金币: +{}", gold.amount);
                    commands.entity(gold_entity).despawn();
                }
            }
        }
    }
}
```

## 与现有模块的关系

- **[`DroppedItemTemplate`](./DroppedItemTemplate.md)** 和 **[`DroppedItemDefinition`](./DroppedItemDefinition.md)**：定义了金币掉落物生成的配置（数量范围、权重）。生成时读取模板中的 `"gold"` 定义来决定 `amount`。
- **`LevelState`**（`src/level.rs`）：拾取时金币数量累加到 `LevelState.money`，驱动 UI 上的金钱显示。
- **`battle` 模块**（`src/battle/`）：战斗击杀事件触发掉落物生成。
- **`common` 模块**（`src/common/`）：`GamePhysicsLayer` 用于设置碰撞层，确保只有玩家角色能触发拾取。

## 最佳实践

1. **数量由模板驱动**：`GoldDroppedItem::amount` 的值应来自掉落模板的随机区间，避免在生成逻辑中硬编码。
2. **碰撞层隔离**：使用 `Sensor` 碰撞器搭配专属拾取层，确保只有玩家角色能拾取，避免敌人或子弹意外触发。
3. **自动脱落**：若金币长时间未被拾取，可配合 `AutoDespawn` 组件和定时器自动清除，防止地图上积累过多掉落物。
