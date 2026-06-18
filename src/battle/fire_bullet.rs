//! Fire-bullet effect submodule.
//!
//! Defines [`FireBulletEffect`] — a component carrying "fire bullet" skill
//! effect parameters (speed, damage, count) — and [`FireBulletBuilder`],
//! which implements [`SkillEffectBuilder`] to attach [`FireBulletEffect`]
//! to the skill entity based on a [`SkillEffectDefinition`].

use bevy::prelude::*;

use crate::skill::{
    BuildError, FromSkillEffectDefinition, IntoSkillEffectDefinition, SkillEffectBuilder,
    SkillEffectBuilderContext, SkillEffectDefinition,
};

// ---------------------------------------------------------------------------
// FireBulletEffect (Component)
// ---------------------------------------------------------------------------

/// "发射子弹"技能效果的运行时参数。
///
/// 该组件封装了子弹的飞行速度、伤害值和一次释放的子弹数量，
/// 由 [`FireBulletBuilder`] 根据 [`SkillEffectDefinition`] 的参数字典构建，
/// 插入到技能实体上供后续子弹生成系统读取。
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct FireBulletEffect {
    /// 子弹飞行速度（像素/秒）。
    pub speed: f32,
    /// 每颗子弹的伤害值。
    pub damage: f32,
    /// 一次释放发射的子弹数量。
    pub count: u32,
}

impl FireBulletEffect {
    /// Create a new [`FireBulletEffect`] with the given parameters.
    pub fn new(speed: f32, damage: f32, count: u32) -> Self {
        Self {
            speed,
            damage,
            count,
        }
    }
}

impl Default for FireBulletEffect {
    fn default() -> Self {
        Self {
            speed: 200.0,
            damage: 10.0,
            count: 1,
        }
    }
}

impl FromSkillEffectDefinition for FireBulletEffect {
    fn from_effect(def: &SkillEffectDefinition) -> Option<Self> {
        Some(Self {
            speed: def.get("speed").unwrap_or(200.0),
            damage: def.get("damage").unwrap_or(10.0),
            count: def.get("count").unwrap_or(1.0) as u32,
        })
    }
}

impl IntoSkillEffectDefinition for FireBulletEffect {
    fn into_effect(self, id: impl Into<String>) -> SkillEffectDefinition {
        let mut def = SkillEffectDefinition::new(id);
        def.set("speed", self.speed);
        def.set("damage", self.damage);
        def.set("count", self.count as f32);
        def
    }
}

// ---------------------------------------------------------------------------
// FireBulletBuilder
// ---------------------------------------------------------------------------

/// 根据 [`FireBulletEffect`] 参数为技能实体附加组件的构建器。
///
/// 读取效果定义中的 `speed`、`damage`、`count` 参数，构造一个
/// [`FireBulletEffect`] 组件并插入到技能实体上。后续子弹生成系统
/// （如 [`crate::battle::fire_bullet_on_skill`]）读取该组件后
/// 再实际生成子弹实体。
pub struct FireBulletBuilder;

impl SkillEffectBuilder for FireBulletBuilder {
    fn build(
        &self,
        commands: &mut Commands,
        ctx: SkillEffectBuilderContext,
    ) -> Result<Entity, BuildError> {
        let params = FireBulletEffect::from_effect(&ctx.effect).ok_or_else(|| {
            BuildError::BuildFailed(
                format!("missing fire_bullet params for effect '{}'", ctx.effect.id),
            )
        })?;

        commands.entity(ctx.skill).insert(params);

        Ok(ctx.skill)
    }
}
