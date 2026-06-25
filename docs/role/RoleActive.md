# RoleActive

`RoleActive` 是一个**标记组件（Marker Component）**，用于标识一个角色实体当前处于活跃状态（在战场上）。

## 设计动机

在游戏运行过程中，角色可能处于不同的状态阶段：

- **待部署**：角色已购买但尚未被放置到备战区格子
- **已部署（活跃）**：角色在战场上，参与战斗、技能和移动判定
- **已阵亡**：角色被击败，从战场上移除

`RoleActive` 标记组件让系统可以快速筛选"当前在战场上活跃的角色"，与 `Role` 组件配合使用：

- **战斗系统**：`Query<&Role, With<RoleActive>>` 获取所有活跃角色进行攻击判定
- **移动系统**：只处理活跃角色的移动指令
- **技能系统**：仅对活跃角色应用技能效果

## 定义

```rust
/// 标记组件，表示一个角色实体当前在战场上处于活跃状态。
///
/// 当角色被部署到备战区格子时添加此组件，
/// 角色阵亡或从战场移除时移除。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct RoleActive;
```

## 使用示例

### 查询所有活跃角色

```rust
fn query_active_roles(roles: Query<&Role, With<RoleActive>>) {
    for _role in &roles {
        info!("活跃角色存在");
    }
}
```

### 部署角色时添加

```rust
fn deploy_role(
    mut commands: Commands,
    role_entity: Entity,
) {
    commands.entity(role_entity).insert(RoleActive);
}
```

### 角色阵亡时移除

```rust
fn remove_on_death(
    mut commands: Commands,
    roles: Query<Entity, (With<Role>, With<RoleActive>)>,
    health_query: Query<&Health>,
) {
    for entity in &roles {
        if let Ok(health) = health_query.get(entity) {
            if health.value <= 0.0 {
                commands.entity(entity).remove::<RoleActive>();
            }
        }
    }
}
```

## 与相关类型的关系

- **`Role`**（[`./Role.md`](./Role.md)）：角色标记组件。`RoleActive` 与 `Role` 共同存在于同一个实体上，`Role` 标识"这是一个角色"，`RoleActive` 标识"这个角色当前活跃"。
- **`Archer`**（[`./Archer.md`](./Archer.md)）：弓箭手角色变体标记。弓箭手实体同时携带 `Role`、`Archer` 和 `RoleActive` 三个标记组件。

## 设计决策

### 标记组件而非枚举状态字段

使用 `RoleActive` 标记组件而非在 `Role` 中添加枚举字段（如 `enum RoleState { Active, Inactive }`），原因：

1. **查询过滤**：ECS 通过 `With<RoleActive>` / `Without<RoleActive>` 在查询层直接过滤，无需运行时检查字段。
2. **零开销**：标记组件不占用额外数据内存，仅为位集中的一个 bit。
3. **可组合**：可与 `Changed<RoleActive>` 等过滤条件组合，实现仅在新活跃或非活跃时触发逻辑。
4. **生命周期管理**：添加/移除组件是 Bevy 的标准模式，与实体销毁、组件清理自然集成。

## 相关文档

- [`Role.md`](./Role.md)：角色标记组件
- [`Archer.md`](./Archer.md)：弓箭手角色变体
