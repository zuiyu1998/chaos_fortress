# Idle2Combat

`Idle2Combat` 是一个实现了 `GearboxMessage` 的消息结构体，用于触发弓箭手状态机从 **Idle（静止）** 到 **Combat（战斗）** 的状态转换。

## 设计动机

在 [`ArcherRoleBuilder`](ArcherRoleBuilder.md) 的 `setup_state_machine` 中，状态机启动时进入 `Idle` 状态，并设置了 `Combat → Idle` 的自动过渡（`AlwaysEdge`）。而 **Idle → Combat** 的切换需要由外部系统主动触发，因此需要定义一个消息类型作为触发器。

## 定义

```rust
use bevy::prelude::*;
use bevy_gearbox::prelude::*;

/// 用于触发弓箭手从 Idle 到 Combat 状态转换的消息。
#[derive(Message, Clone)]
pub struct Idle2Combat {
    /// 目标状态机实体的 ID。
    pub machine: Entity,
}

impl GearboxMessage for Idle2Combat {
    type Validator = AcceptAll;

    fn target(&self) -> Entity {
        self.machine
    }
}
```

### 类型说明

| 关联类型 | 值 | 含义 |
|---|---|---|
| `Validator` | `AcceptAll` | 所有该类型的消息都会被对应的 `MessageEdge` 接受，无需额外过滤 |

### 方法说明

| 方法 | 返回 | 说明 |
|---|---|---|
| `target()` | `Entity` | 返回目标状态机实体的 ID，gearbox 根据此字段将消息路由到正确的状态机 |

## 注册

`Idle2Combat` 已在 `ArcherPlugin` 中注册：

```rust
impl Plugin for ArcherPlugin {
    fn build(&self, app: &mut App) {
        // ... 其他注册 ...
        app.register_transition::<Idle2Combat>();
        // ... 其他系统 ...
    }
}
```

## 使用方式

### 在 `setup_state_machine` 中创建过渡

由 [`setup_state_machine`](ArcherRoleBuilder.md#setup_state_machine) 在初始化状态机时创建对应 `MessageEdge`：

```rust
pub fn setup_state_machine(machine: Entity, commands: &mut Commands) {
    // ... 创建 Idle / Combat 子状态 ...

    // 消息驱动过渡：Idle → Combat
    commands.spawn_transition::<Idle2Combat>(idle, combat);
    // 自动过渡：Combat → Idle
    commands.spawn_transition_always(combat, idle);

    // ... 初始化状态机 ...
}
```

`spawn_transition::<Idle2Combat>(idle, combat)` 会在 `Idle` 状态上附加一个 `MessageEdge<Idle2Combat>`，当收到匹配的消息时触发到 `Combat` 的转换。

### 发送消息

外部系统通过 `MessageWriter<Idle2Combat>` 发送消息来触发转换：

```rust
/// 当检测到敌对目标进入射程时，触发 Idle → Combat 转换。
fn detect_hostiles_system(
    archers: Query<Entity, (With<Archer>, With<StateMachine>)>,
    mut writer: MessageWriter<Idle2Combat>,
) {
    for machine in &archers {
        // 假设目标检测通过，触发战斗状态
        writer.write(Idle2Combat { machine });
    }
}
```

> `MessageWriter<M>` 是 gearbox 提供的发送消息的 writer 类型，需通过 `app.register_transition::<M>()` 注册后方可使用。

## 状态转换关系

```
                    ┌──────────────────────────────────┐
                    │         状态机 (StateMachine)      │
                    │                                  │
 ┌──────────┐      Idle2Combat 消息      ┌──────────┐ │
 │  Idle    │ ──────────────────────────▶ │  Combat  │ │
 │ (静止)   │                              │ (战斗)   │ │
 └──────────┘                              └──────────┘ │
                    │                                  │
                    └──────────────────────────────────┘
```

- **Idle → Combat**：通过发送 `Idle2Combat` 消息触发，由外部系统决定何时发送。
- **Combat → Idle**：由外部系统酌情处理，当前状态机不预设自动返回过渡。

## 相关文档

- [`ArcherRoleBuilder`](ArcherRoleBuilder.md) — 弓箭手角色构建器，`setup_state_machine` 的使用入口
- [`ArcherPlugin`](ArcherPlugin.md) — 弓箭手插件，包含 `Idle2Combat` 的注册
