# BattlePlugin

`BattlePlugin` 是一个插件对象，实现了 Bevy 的 [`Plugin`] trait，负责注册战斗模块相关的组件。

## 用途

- 向 Bevy 应用注册 [`BattleState`] 组件到类型反射系统（Type Registry）。
- 通过 `add_message` 注册 [`DeathInBattle`] 消息。
- 添加 `Update` 阶段系统 [`despawn_on_death`]：读取 [`DeathInBattle`] 消息，销毁已死亡的实体。
- 添加 `Update` 阶段系统 [`fire_bullet_on_skill`]：读取 [`SkillEvent`] 消息，根据技能持有者的 [`EnemyTarget`] 锁定敌人并生成子弹。
- 注册后，该组件可在编辑器中序列化/反序列化，并支持运行时反射访问。

## 定义

```rust
/// 注册战斗相关组件的插件。
pub(super) struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BattleState>();
        app.add_message::<DeathInBattle>();

        app.add_systems(Update, (despawn_on_death, fire_bullet_on_skill));
    }
}
```

## 注册的组件

| 组件 | 说明 |
|------|------|
| [`BattleState`] | 存储战斗实体的战斗属性（血量、护甲等）。 |

## 注册的消息

| 消息 | 说明 |
|------|------|
| [`DeathInBattle`] | 战斗实体死亡时发出的消息，携带已死亡的实体。通过 `add_message` 注册以便 Bevy 消息系统处理。 |

## 注册的系统

| 系统 | 阶段 | 说明 |
|------|------|------|
| [`despawn_on_death`] | `Update` | 读取 [`DeathInBattle`] 消息，将死亡实体从场景中销毁。 |
| [`fire_bullet_on_skill`] | `Update` | 读取 [`SkillEvent`] 消息，根据技能持有者的 [`EnemyTarget`] 锁定敌人并生成子弹。 |

### despawn_on_death

`despawn_on_death` 是一个简单的默认系统，在战斗实体死亡时将其从场景中移除。

```rust
pub fn despawn_on_death(
    mut events: MessageReader<DeathInBattle>,
    mut commands: Commands,
) {
    for event in events.read() {
        commands.entity(event.entity).despawn();
    }
}
```

该系统提供了一种基础的行为模式。更复杂的系统（死亡动画、掉落物、重生逻辑）可以在此基础上替换或补充。

### fire_bullet_on_skill

`fire_bullet_on_skill` 是一个系统函数，用于响应技能完成事件，自动朝当前锁定的敌人发射子弹。

#### 功能

1. 读取 [`SkillEvent`] 消息，获取技能持有者（`owner`）。
2. 查询技能持有者身上的 [`EnemyTarget`] 组件，获取锁定的敌人实体。
3. 查询敌人的 [`GlobalTransform`] 获取世界坐标。
4. 查询技能持有者的 [`GlobalTransform`]（或通过 [`BulletPositionTarget`] 获取子弹发射原点）。
5. 计算方向向量并调用 `bullet()` 函数生成子弹实体。

#### 实现

```rust
/// 响应 SkillEvent，朝技能持有者的 EnemyTarget 发射子弹。
///
/// 数据流：
///   SkillEvent.owner
///     → EnemyTarget.0 (敌人实体)
///       → &GlobalTransform (敌人位置)
///     → BulletPositionTarget / &GlobalTransform (发射原点)
///       → bullet(position, direction * BULLET_SPEED)
pub fn fire_bullet_on_skill(
    mut skill_events: MessageReader<SkillEvent>,
    mut commands: Commands,
    owners: Query<(&EnemyTarget, Option<&BulletPositionTarget>, &GlobalTransform)>,
    bullet_positions: Query<&GlobalTransform, With<BulletPosition>>,
    enemy_transforms: Query<&GlobalTransform>,
) {
    const BULLET_SPEED: f32 = 200.0;

    for event in skill_events.read() {
        // 1. 获取技能持有者的 EnemyTarget 和位置信息
        let Ok((enemy_target, bullet_target, owner_transform)) = owners.get(event.owner) else {
            continue;
        };

        // 2. 检查是否有锁定的敌人
        let Some(enemy) = enemy_target.0 else {
            continue;
        };

        // 3. 获取敌人的世界坐标
        let Ok(enemy_transform) = enemy_transforms.get(enemy) else {
            continue;
        };

        // 4. 确定子弹发射原点
        //    优先使用 BulletPositionTarget 指向的子实体坐标，
        //    若无则回退到技能持有者自身的 GlobalTransform。
        let spawn_position = if let Some(bullet_target) = bullet_target {
            if let Ok(pos) = bullet_positions.get(bullet_target.0) {
                pos.translation().truncate()
            } else {
                owner_transform.translation().truncate()
            }
        } else {
            owner_transform.translation().truncate()
        };

        // 5. 计算朝向敌人的方向并生成子弹
        let direction = (enemy_transform.translation().truncate() - spawn_position)
            .normalize_or_zero();
        commands.spawn(bullet(spawn_position, direction * BULLET_SPEED));
    }
}
```

#### 查询说明

| 查询 | 用途 |
|------|------|
| `MessageReader<SkillEvent>` | 读取技能完成事件，获取 `owner`（技能持有者） |
| `Query<(&EnemyTarget, Option<&BulletPositionTarget>, &GlobalTransform)>` | 在 `event.owner` 上查询锁定目标、子弹发射原点引用和自身位置 |
| `Query<&GlobalTransform, With<BulletPosition>>` | 通过 `BulletPositionTarget` 的引用坐标查询子实体的世界位置 |
| `Query<&GlobalTransform>` | 查询敌人的世界坐标 |

#### 设计说明

- **消息驱动**：系统仅在收到 [`SkillEvent`] 时触发，与冷却计时器解耦，适用于任意技能系统。
- **可选子实体**：通过 `Option<&BulletPositionTarget>` 兼容有/无子弹发射点的实体。
  - 若实体携带 `BulletPositionTarget`，则从其引用的 `BulletPosition` 子实体获取发射坐标。
  - 若未携带，直接使用实体自身的 `GlobalTransform` 作为发射原点。
- **无目标跳过**：`EnemyTarget.0` 为 `None` 时跳过，避免空指针发射。
- **速度常量**：`BULLET_SPEED` 暂定为 `200.0`（与 [`ArcherPlugin`] 中的 `run_skill` 保持一致），后续可根据技能配置参数化。`

## 与现有模块的关系

- **战斗模块**（`battle`）：`BattlePlugin` 是战斗模块的入口插件，由主应用（`AppPlugin`）的插件列表添加。
- **主应用**（`main`）：在 `src/main.rs` 中以 `battle::BattlePlugin` 的形式被添加至 Bevy 应用。
- **角色模块**（`role`）：角色实体在构建时可通过 [`BattleState`] 组件参与战斗。[`fire_bullet_on_skill`] 读取角色的 [`EnemyTarget`] 和 [`BulletPositionTarget`] 来锁定目标并确定发射原点。
- **敌人模块**（`enemy`）：敌人实体同样可通过 [`BattleState`] 组件拥有血量和护甲。[`fire_bullet_on_skill`] 查询敌人的 [`GlobalTransform`] 获取位置作为子弹飞行目标。
- **技能模块**（`skill`）：[`SkillEvent`] 消息由技能执行系统发出，`fire_bullet_on_skill` 作为消费者读取该消息实现"技能完成 → 发射子弹"的连锁反应。
- **子弹模块**（`bullet`）：`fire_bullet_on_skill` 调用 `bullet()` 函数生成子弹，子弹随后由 [`BulletPlugin`] 中的碰撞系统自动处理。

[`BattleState`]: ./BattleState.md
[`BattlePlugin`]: ./BattlePlugin.md
[`DeathInBattle`]: ./DeathInBattle.md
[`despawn_on_death`]: #despawn_on_death
[`fire_bullet_on_skill`]: #fire_bullet_on_skill
[`SkillEvent`]: ../skill/SkillEvent.md
[`EnemyTarget`]: ../common/EnemyTarget.md
[`BulletPositionTarget`]: ../bullet/BulletPositionTarget.md
[`BulletPosition`]: ../bullet/BulletPosition.md
[`BulletPlugin`]: ../bullet/BulletPlugin.md
[`ArcherPlugin`]: ../role/archer/ArcherPlugin.md
[`GlobalTransform`]: https://docs.rs/bevy/latest/bevy/transform/components/struct.GlobalTransform.html
[`Plugin`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html
