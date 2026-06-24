# EnemySpawner

`EnemySpawner` 是一个 Bevy [`Resource`]，用于**在固定区域内随机生成敌人**。它将生成配置、区域管理和 Builder 调度整合为一个统一的可配置资源。

## 设计动机

`EnemySpawner` 需要一种机制来：

- 定义要生成的敌人（名称、数量、间隔）
- 在固定区域内随机选取生成坐标
- 驱动敌人按时间轴逐个生成
将生成逻辑直接写在 System 中会导致 System 承担过多职责，`EnemySpawner` 将这些数据和行为封装为 Resource，供关卡 System 驱动。

## 定义

```rust
/// 生成区域：敌人在此区域内随机选取格子坐标生成。
#[derive(Clone, Copy, Debug, Default)]
pub struct SpawnArea {
    /// 生成区域的最小列坐标（含）。
    pub col_min: u32,
    /// 生成区域的最大列坐标（含）。
    pub col_max: u32,
    /// 生成区域的最小行坐标（含）。
    pub row_min: u32,
    /// 生成区域的最大行坐标（含）。
    pub row_max: u32,
}

/// 单个敌人的生成配置。
#[derive(Clone, Debug, Default)]
pub struct SpawnEntry {
    /// 注册在 EnemyBuilderContainer 中的名称。
    pub builder_name: String,
    /// 该类型生成的总数量（当前保留字段，供扩展用）。
    pub count: u32,
    /// 连续生成之间的间隔（秒）。
    pub interval: f32,
}

/// 管理敌人生成的 Bevy 资源。
///
/// 在固定区域 [`spawn_area`](EnemySpawner::spawn_area) 内随机选取坐标，
/// 从 [`entries`](EnemySpawner::entries) 中随机选取一个类型进行生成，
/// 每生成一个敌人后重置 [`spawn_timer`](EnemySpawner::spawn_timer) 为
/// 对应 entry 的 [`interval`](SpawnEntry::interval)。
#[derive(Resource, Default)]
pub struct EnemySpawner {
    /// 生成区域（在此矩形区域内随机选取坐标）。
    pub spawn_area: SpawnArea,
    /// 要生成的敌人配置。
    pub entries: Vec<SpawnEntry>,
    /// 生成倒计时（秒），归零时生成下一个敌人。
    pub spawn_timer: f32,
    /// 父实体（包含地图的 Level 实体），敌人实体将挂载为其子级。
    pub parent: Option<Entity>,
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `spawn_area` | `SpawnArea` | 矩形生成区域，运行时在此区域内随机选取坐标。 |
| `entries` | `Vec<SpawnEntry>` | 要生成的敌人配置表，按顺序逐个处理。 |
| `spawn_timer` | `f32` | 生成倒计时，归零时生成下一个敌人。 |
| `parent` | `Option<Entity>` | 父实体（Level 实体），敌人生成后挂载为其子级。 |

### SpawnArea

| 字段 | 类型 | 说明 |
|------|------|------|
| `col_min` | `u32` | 生成区域的最小列坐标（含）。 |
| `col_max` | `u32` | 生成区域的最大列坐标（含）。 |
| `row_min` | `u32` | 生成区域的最小行坐标（含）。 |
| `row_max` | `u32` | 生成区域的最大行坐标（含）。 |

### SpawnEntry

| 字段 | 类型 | 说明 |
|------|------|------|
| `builder_name` | `String` | builder 名称，与 `EnemyBuilderContainer` 中注册的名称对应。 |
| `count` | `u32` | 该类型敌人的生成总数量（当前保留字段，供扩展用）。 |
| `interval` | `f32` | 同类型敌人之间的生成间隔（秒）。 |

## 初始化

`EnemySpawner` 由 `EnemyPlugin` 通过 `init_resource` 插入默认实例，再由 `spawn_level` 在进入关卡时用实际配置覆盖：

```rust
// 在 spawn_level 中
let level_entity = commands.spawn((Name::new("Level"), ...)).id();
commands.insert_resource(EnemySpawner {
    spawn_area: SpawnArea {
        col_min: 0, col_max: 5,
        row_min: 2, row_max: 6,
    },
    entries: vec![
        SpawnEntry { builder_name: "soldier".into(), count: 10, interval: 0.6 },
        SpawnEntry { builder_name: "scout".into(),   count: 5,  interval: 0.4 },
    ],
    parent: Some(level_entity),
    ..default()
});
```

## 使用方式

`tick_enemy_spawner` 定义在 [`level`](crate::level) 模块中，由 `LevelPlugin` 注册，通过 `EnemySystems` System Set 控制运行条件：

```rust
fn tick_enemy_spawner(
    time: Res<Time>,
    mut spawner: ResMut<EnemySpawner>,
    container: Res<EnemyBuilderContainer>,
    enemy_assets: Res<assets::EnemyAssets>,
    template_assets: Res<Assets<AttributeTemplate>>,
    mut commands: Commands,
) {
    let dt = time.delta_secs();
    spawner.spawn_timer -= dt;

    if spawner.spawn_timer > 0.0 {
        return;
    }

    let area = &spawner.spawn_area;
    let mut rng = rand::rng();

    // 在固定区域内随机选取生成坐标
    let col = rng.random_range(area.col_min..=area.col_max);
    let row = rng.random_range(area.row_min..=area.row_max);

    // 随机选取一个 entry 生成敌人
    let entry = &spawner.entries[rng.random_range(0..spawner.entries.len())];

    // 构造基础属性集
    let attrs = template_assets
        .get(&enemy_assets.basic_attributes)
        .map(|t| t.build_attribute_set(&["hp", "max_hp", "armor", "attack"]))
        .unwrap_or_else(|| {
            let mut a = AttributeSet::new();
            a.insert("hp", Attribute::new(100.0));
            a.insert("max_hp", Attribute::new(100.0));
            a.insert("armor", Attribute::new(10.0));
            a.insert("attack", Attribute::new(10.0));
            a
        });

    let ctx = EnemyBuilderContext {
        position: (col, row),
        cell_size: 64.0,
        parent: spawner.parent,
        attributes: attrs,
    };

    let mut cmds = commands.reborrow();
    if let Err(e) = container.build(&entry.builder_name, &mut cmds, ctx) {
        error!("EnemySpawner: failed to build '{}': {e}", entry.builder_name);
    }

    spawner.spawn_timer = entry.interval;
}
```

## 与 `EnemyBuilderContext` 的配合

`EnemySpawner` 会在生成时构造 `EnemyBuilderContext` 并传入 Builder：

```rust
pub struct EnemyBuilderContext {
    pub position: (u32, u32),
    pub cell_size: f32,
    pub parent: Option<Entity>,
    pub attributes: AttributeSet,
}
```

Builder 在构建实体时使用上下文中的位置和属性：

```rust
impl EnemyBuilder for BasicEnemyBuilder {
    fn build(&self, commands: &mut Commands, ctx: EnemyBuilderContext) -> Result<Entity, EnemyBuildError> {
        // 使用 ctx.position 设置实体坐标
        // 使用 ctx.attributes 获取战斗属性
        let hp = ctx.attributes.get("hp").map(|a| a.value).unwrap_or(100.0);
        // ...
    }
}
```

## 与现有模块的关系

```
关卡系统 (spawn_level / LevelPlugin)
     │ (初始化 EnemySpawner + 注册 tick_enemy_spawner)
     ▼
EnemySpawner                 ← Bevy Resource
     │
     ├── 随机选取 entries[ ].builder_name
     ├── 在 SpawnArea 内随机生成 (col, row)
     ├── 从 EnemyAssets + AttributeTemplate 构造属性集
     ├── 构造 EnemyBuilderContext（含 parent = Level 实体）
     │
     ▼
EnemyBuilderContainer        ← Bevy Resource
     │ (按名称查找 → 调用闭包)
     ▼
EnemyBuilder::build()
     │
     ├── 使用 ctx.parent 将敌人挂载到 Level 实体下
     ▼
commands.spawn((Enemy, ...)) → 敌人实体 (Level 子级)
```

| 模块 | 关系 |
|------|------|
| [`EnemyBuilderContainer`](./EnemyBuilderContainer.md) | `EnemySpawner` 通过它按名称查找 builder 并生成敌人实体。 |
| [`EnemyBuilderContext`](./EnemyBuilderContext.md) | 生成时构造上下文，传入位置、格子尺寸、父实体和属性集。 |
| [`EnemyBuilder`](./EnemyBuilder.md) | 实际的实体构建逻辑由 builder 实现，`EnemySpawner` 仅负责调度。 |
| [`EnemyAssets`](./EnemyAssets.md) | `EnemySpawner` 驱动系统从中读取 `basic_attributes` 模板来构造属性集。 |
| [`EnemyPlugin`](./EnemyPlugin.md) | 负责注册 `EnemySpawner` Resource，以及标记敌人系统为 `EnemySystems` 集合。 |
| [`LevelPlugin`](../level/LevelPlugin.md) | 负责注册 `tick_enemy_spawner` 驱动系统（位于 `level` 模块）。 |
| [`EnemySystem`](./EnemySystem.md) | 定义了敌人系统的整体规范，`EnemySpawner` 是其 ECS 实现的一部分。 |
| [`Enemy`](./Enemy.md) | 生成的实体通过 `Enemy` 组件标记，最终被敌人系统消费。 |

## 注意事项

- `EnemySpawner` 只负责调度生成**时机**与生成**坐标**，不负责生成后的 AI 行为——敌人实体生成后立即进入 AI 系统驱动的行为循环。
- 当前实现使用 `rand::rng().random_range()` 在 `SpawnArea` 内随机选取整数坐标，属于**临时简化方案**；后续可替换为基于入口点、路径或噪声分布的生成策略。
- `SpawnArea` 定义的是闭区间 `[col_min, col_max] × [row_min, row_max]`，请确保坐标范围有效（`col_min ≤ col_max` 且 `row_min ≤ row_max`）。
- 每次生成时从 `entries` 中**随机选取**一个 entry 的 builder 来生成敌人，不同种类敌人按 `interval` 交替出现。
- 如果需要动态调整生成内容（如根据玩家表现增减敌人种类），可直接修改 `entries`。
