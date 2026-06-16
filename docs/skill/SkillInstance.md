# SkillInstance

`SkillInstance` 是一个组件（Component），用于存储技能在实体上的**运行时状态**，包括冷却进度、充能层数、剩余使用次数等。

## 用途

- 持有 `SkillInstance` 组件的实体代表该技能在当前实体上**已就绪**，可被技能系统调度施放。
- 同一实体可持有多个 `SkillInstance` 组件（每个技能一个实例），通过 `skill_id` 区分。
- 与 [`SkillDefinition`] 模板组件配合使用：`SkillDefinition` 定义技能的静态属性（名称、数值特征等），`SkillInstance` 追踪动态运行时状态。
- 技能冷却由 `SkillInstance` 内置的计时器管理，无需额外附加 [`CoolingTimer`] 组件。

## 定义

```rust
/// 技能运行时状态。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct SkillInstance {
    /// 引用的技能模板 ID。
    pub skill_id: String,
    /// 冷却计时器（秒），归零时技能可用。
    pub cooldown_timer: f32,
    /// 冷却总时长（秒），用于重置计时器。
    pub cooldown_seconds: f32,
    /// 当前充能层数。
    pub charges: u32,
    /// 最大充能层数（0 表示无充能机制）。
    pub max_charges: u32,
    /// 技能当前状态。
    pub status: SkillStatus,
}

impl SkillInstance {
    /// 创建一个新的 SkillInstance。
    pub fn new(
        skill_id: impl Into<String>,
        cooldown_seconds: f32,
    ) -> Self { ... }
    /// 技能是否可施放（冷却完毕且有可用充能）。
    pub fn ready(&self) -> bool { ... }
    /// 施放技能，消耗一次充能并重置冷却。
    pub fn use_skill(&mut self) { ... }
    /// 每帧推进冷却计时。
    pub fn tick(&mut self, delta: f32) { ... }
}
```

## 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `skill_id` | `String` | 引用的 [`SkillDefinition`] 模板 ID，用于在施放时查找技能效果数据。 |
| `cooldown_timer` | `f32` | 当前冷却剩余秒数，归零后可施放。每帧由 `tick` 递减。 |
| `cooldown_seconds` | `f32` | 冷却总时长（秒），施放技能时 `cooldown_timer` 重置为此值。 |
| `charges` | `u32` | 当前充能层数，`> 0` 时技能可用。每次施放消耗 1 层。 |
| `max_charges` | `u32` | 最大充能层数。`0` 表示无充能机制，冷却完毕即可施放。 |
| `status` | `SkillStatus` | 技能的当前运行时状态。 |

### SkillStatus 枚举

```rust
pub enum SkillStatus {
    /// 就绪状态，可施放。
    Ready,
    /// 冷却中。
    Cooling,
    /// 引导中（持续施法），不可被打断。
    Channeling,
    /// 被禁用（沉默、眩晕等）。
    Disabled,
}
```

## 方法

| 方法 | 说明 |
|------|------|
| `new(skill_id, cooldown_seconds)` | 创建一个新的 `SkillInstance`，初始 `Ready`，无充能机制（`charges = 0`，`max_charges = 0`）。 |
| `ready() -> bool` | 返回 `status == Ready && cooldown_timer <= 0.0 && charges > 0`（有充能时）或 `status == Ready && cooldown_timer <= 0.0`（无充能时）。 |
| `use_skill()` | 施放技能：消耗 1 层充能，设置冷却状态，`cooldown_timer = cooldown_seconds`，`status = Cooling`。 |
| `tick(delta)` | 每帧调用，递减 `cooldown_timer`。归零后检查充能：若 `charges > 0` 则 `status = Ready`；否则保持 `Cooling`。 |
| `add_charge(count)` | 增加 `count` 层充能（不超过 `max_charges`）。 |

### tick 数据流

```
每帧调用 tick(delta)
  → cooldown_timer -= delta
  → 若 cooldown_timer <= 0.0：
      → cooldown_timer = 0.0
      → 若 charges > 0 && status != Disabled：
          → status = Ready
```

## 与现有模块的关系

- **[`SkillDefinition`]**：`skill_id` 字段引用 `SkillDefinition.id`。技能系统在施放时通过 `skill_id` 查找对应的 `SkillDefinition` 组件，读取 `name` 和 [`SkillFeatureDefinition`] 特征数据来计算效果。
- **[`CoolingTimer`]**：`SkillInstance` 内置冷却管理，**替代**了在该技能场景下使用独立的 `CoolingTimer` 组件。每个技能实例自带计时器，无需额外组件。
- **`run_skill` 系统**：弓箭手等角色的技能驱动系统应查询 `SkillInstance` 组件而非 `CoolingTimer`，调用 `ready()` 判断是否可施放，施放后调用 `use_skill()`。
- **敌人模块**（`enemy`）：敌人实体携带的 `skills: [Skill]` 在生成时可转化为多个 `SkillInstance` 组件附加到实体上，供 AI 系统调度。
- **[`BattleState`]**：施放技能时，通过 `SkillDefinition` 模板的 `execute` 逻辑计算伤害，再调用 `BattleState::take_damage` 对目标造成伤害。
- **充能恢复系统**：可通过定期调用 `add_charge` 实现随时间恢复充能的机制（例如每 10 秒恢复 1 层）。

### 典型使用流程

```
1. 实体生成时，为每个技能创建一个 SkillInstance 组件并附加。
2. 每帧调用 SkillInstance::tick(delta) 推进冷却。
3. run_skill 系统检查 SkillInstance::ready()。
4. 就绪时调用 use_skill() 消耗充能、触发冷却。
5. 技能系统通过 skill_id 查找 SkillDefinition，获取特征数据计算效果。
6. 效果应用于目标（伤害、治疗、Buff 等）。
```

[`SkillDefinition`]: ./SkillDefinition.md
[`SkillInstance.skill_id`]: ./SkillInstance.md#字段说明
[`SkillFeatureDefinition`]: ./SkillFeatureDefinition.md
[`CoolingTimer`]: ../common/CoolingTimer.md
[`BattleState`]: ../battle/BattleState.md
