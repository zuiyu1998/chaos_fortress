# RoleShopItem

`RoleShopItem` 是一个 **组件（Component）**，附着在备战区（Preparation Zone）的格子实体上，用于标识该格子需要生成的角色。当玩家从商店购买角色后，系统在对应格子上添加此组件，随后备战区系统读取该组件并调用 [`role::role()`] 在棋盘上生成实际的角色实体。

## 设计动机

商店购买角色的流程需要一种机制来标记"哪个位置生成什么角色"。`RoleShopItem` 将角色生成信息（使用的 builder）以组件的形式绑定到备战区的格子实体上，使得：

- 备战区系统可以直接查询带有 `RoleShopItem` 的格子，按需生成角色
- 购买逻辑只需将 `RoleShopItem` 添加到目标格子，不直接关心角色生成细节
- 格子与角色的对应关系通过 ECS 组件表达，无需额外的查找表

## 定义

```rust
/// 标记一个备战区格子需要生成的角色信息。
///
/// 附着在备战区格子实体上，系统读取此组件后调用
/// `role::role()` 生成对应的角色实体作为该格子的子实体。
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct RoleShopItem {
    /// 角色在 `RoleBuilderContainer` 中的注册名（如 `"archer"`）。
    pub role_name: String,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `role_name` | `String` | 角色在 [`RoleBuilderContainer`] 中的注册名称，传递给 `container.build(role_name, ...)` 使用。例如 `"archer"` 对应 `ArcherRoleBuilder` |

## 使用示例

### 购买后添加到备战区格子

```rust
fn buy_archer(
    click: On<Pointer<Click>>,
    mut level_state: ResMut<LevelState>,
    query: Query<&ShopItem>,
    // 查询备战区的第一个空闲格子
    bench_slots: Query<(Entity, &RoleShopItem), Without<Children>>,
    mut commands: Commands,
) {
    let item = query.get(click.entity()).unwrap();

    if level_state.money < item.price {
        warn!("金币不足");
        return;
    }
    level_state.money -= item.price;

    // 在空闲格子上添加 RoleShopItem，触发角色生成
    if let Some((slot_entity, _)) = bench_slots.iter().next() {
        commands.entity(slot_entity).insert(RoleShopItem {
            role_name: item.value.clone(), // "archer"
        });
    }
}
```

### 备战区生成系统

```rust
/// 读取所有带有 `RoleShopItem` 且尚未生成子角色的备战区格子，
/// 调用 `role::role()` 生成角色实体。
fn spawn_bench_roles(
    bench_slots: Query<(Entity, &RoleShopItem), (With<RoleShopItem>, Without<Children>)>,
    container: Res<RoleBuilderContainer>,
    role_assets: Res<assets::RoleAssets>,
    template_assets: Res<Assets<AttributeTemplate>>,
    skill_container: Res<SkillFeatureBuilderContainer>,
    skill_effect_container: Res<SkillEffectBuilderContainer>,
    skill_assets: Res<Assets<SkillDefinition>>,
    mut commands: Commands,
) {
    for (slot_entity, _slot) in &bench_slots {
        commands.entity(slot_entity).with_children(|spawner| {
            role::role(
                spawner,
                &container,
                0, // column — 从格子实体 Transform 获取
                0, // row — 从格子实体 Transform 获取
                &role_assets,
                &template_assets,
                &skill_container,
                &skill_effect_container,
                &skill_assets,
            );
        });
    }
}
```

## 与相关类型的关系

- **`ShopItem`**（[`./ShopItem.md`](./ShopItem.md)）：商店售卖道具的定义，描述"卖什么"；购买成功后由业务逻辑将信息转移到 `RoleShopItem`（"在哪里生成什么"）。
- **`role::role()`**（[`../role/Role.md`](../role/Role.md)）：实际生成角色实体的函数，`RoleShopItem` 的消费方。
- **`RoleBuilderContainer`**（[`../role/RoleBuilderContainer.md`](../role/RoleBuilderContainer.md)）：根据 `role_name` 查找并调用对应的 `RoleBuilder` 来构建角色。
- **`RoleBuilderContext`**（[`../role/RoleBuilderContext.md`](../role/RoleBuilderContext.md)）：传递给 `RoleBuilder::build()` 的上下文，包含网格坐标、属性集和技能信息。
- **备战区格子**（地图系统）：地图上前两列（column 0~1）为备战区，格子实体的子实体即为已生成的角色。

## 数据流

```
玩家点击购买按钮
       │
       ▼
buy_archer 读取 ShopItem（价格、角色名等）
       │
       ├─ 金币不足 → 拒绝购买
       │
       └─ 金币充足 → 扣除金币
                       │
                       ▼
            在备战区空闲格子上插入 RoleShopItem 组件
                       │
                       ▼
            spawn_bench_roles 系统查询该组件
                       │
                       ▼
            调用 role::role() 在格子下生成角色实体
```

## 设计决策

### 组件而非资源

`RoleShopItem` 被设计为组件而非资源，原因如下：

1. **每个格子独立**：备战区有多个格子（2列 × 5行 = 10个），每个格子可能需要生成不同的角色。组件天然挂载在各自格子的实体上，无需索引映射。
2. **ECS 原生查询**：系统可以通过 `Query<(Entity, &RoleShopItem)>` 直接遍历所有待生成的格子，利用 Bevy 的查询系统按需处理。
3. **与格子生命周期一致**：当备战区格子被清除或重置时，`RoleShopItem` 组件随实体一同销毁，无需手动清理资源。
4. **状态可见**：在 Bevy 编辑器中可以直观地看到每个格子是否已分配角色。

### role_name 使用 String

沿用 [`ShopItem`](./ShopItem.md) 的设计风格，`role_name` 使用 `String` 而非枚举，保持与外部配置和 [`RoleBuilderContainer`] 注册名称的一致性。

## 相关文档

- [`ShopItem.md`](./ShopItem.md)：商店道具定义
- [`Shop.md`](./Shop.md)：商店状态管理
- [`Role.md`](../role/Role.md)：角色实体生成
- [`RoleBuilderContainer.md`](../role/RoleBuilderContainer.md)：角色构建器注册表
- [`MapSystem.md`](../map/MapSystem.md)：地图系统，包含备战区布局说明
