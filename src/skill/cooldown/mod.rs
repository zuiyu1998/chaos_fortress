//! Cooldown feature submodule.
//!
//! Defines [`CooldownFeature`] and [`CooldownFeatureBuilder`] for
//! representing and building cooldown effects on skill entities.
//!
//! Also provides [`tick_cooldown_timer`] — a system that advances the
//! [`CoolingTimer`] each frame and updates the [`SkillRunContext`] when
//! the cooldown expires.

use bevy::prelude::*;

use crate::common::CoolingTimer;

use super::{
    BuildError, FromSkillFeatureDefinition, IntoSkillFeatureDefinition, SkillEvent,
    SkillFeatureBuilder, SkillFeatureBuilderContext, SkillFeatureDefinition,
    SkillFeatureResult, SkillFeatureResultData, SkillRunContext,
};

// ---------------------------------------------------------------------------
// CooldownFeature
// ---------------------------------------------------------------------------

/// A cooldown feature for a skill.
///
/// Carries the cooldown duration in seconds. Implements
/// [`FromSkillFeatureDefinition`] and [`IntoSkillFeatureDefinition`] for
/// bidirectional conversion with [`SkillFeatureDefinition`].
#[derive(Component, Debug, Clone, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CooldownFeature {
    /// Cooldown duration in seconds. Defaults to `1.0`.
    pub cooldown_duration: f32,
}

impl CooldownFeature {
    /// Create a new [`CooldownFeature`] with the default duration of 1.0 s.
    pub fn new() -> Self {
        Self {
            cooldown_duration: 1.0,
        }
    }
}

impl Default for CooldownFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl FromSkillFeatureDefinition for CooldownFeature {
    fn from_feature(def: &SkillFeatureDefinition) -> Option<Self> {
        Some(Self {
            cooldown_duration: def.get("cooldown_duration").unwrap_or(1.0),
        })
    }
}

impl IntoSkillFeatureDefinition for CooldownFeature {
    fn into_feature(self, id: impl Into<String>) -> SkillFeatureDefinition {
        let mut def = SkillFeatureDefinition::new(id);
        def.set("cooldown_duration", self.cooldown_duration);
        def
    }
}

// ---------------------------------------------------------------------------
// CooldownFeatureBuilder
// ---------------------------------------------------------------------------

/// Builder that attaches a [`CoolingTimer`] to the skill entity and sets its
/// parent to the target entity.
///
/// Reads `cooldown_duration` from the feature's numeric dictionary (default 1.0 s)
/// and inserts `CoolingTimer(Timer::from_seconds(cooldown_duration, TimerMode::Once))`
/// on `ctx.skill`, then parents the skill entity to `ctx.target`.
pub struct CooldownFeatureBuilder;

impl SkillFeatureBuilder for CooldownFeatureBuilder {
    fn build(
        &self,
        commands: &mut Commands,
        ctx: SkillFeatureBuilderContext,
    ) -> Result<Entity, BuildError> {
        let cooldown_duration = ctx.feature.get("cooldown_duration").unwrap_or(1.0);

        commands
            .entity(ctx.skill)
            .insert((
                CooldownFeature { cooldown_duration },
                CoolingTimer(Timer::from_seconds(cooldown_duration, TimerMode::Once)),
            ))
            .set_parent_in_place(ctx.target);

        Ok(ctx.skill)
    }
}

// ---------------------------------------------------------------------------
// CooldownResult
// ---------------------------------------------------------------------------

/// Data stored in [`SkillFeatureResult::Ok`] when a cooldown completes.
#[derive(Debug)]
pub struct CooldownResult {
    /// The cooldown duration in seconds.
    pub duration: f32,
}

impl SkillFeatureResultData for CooldownResult {
    fn clone_box(&self) -> Box<dyn SkillFeatureResultData> {
        Box::new(Self {
            duration: self.duration,
        })
    }
}

// ---------------------------------------------------------------------------
// Systems
// ---------------------------------------------------------------------------

/// Advances the [`CoolingTimer`] on skill entities each frame and updates
/// the [`SkillRunContext`] when the cooldown expires.
///
/// When the timer finishes, the `"cooldown"` entry in
/// [`SkillRunContext::feature_results`] is set to
/// [`SkillFeatureResult::Ok`] carrying a [`CooldownResult`].
pub fn tick_cooldown_timer(
    time: Res<Time>,
    mut query: Query<(&CooldownFeature, &mut CoolingTimer, &mut SkillRunContext)>,
) {
    for (feature, mut timer, mut ctx) in query.iter_mut() {
        // Tick the timer forward.
        timer.0.tick(time.delta());

        // If the timer just finished this frame, mark the cooldown as complete.
        if timer.0.just_finished() {
            ctx.record_feature_result(
                "cooldown",
                SkillFeatureResult::Ok(Box::new(CooldownResult {
                    duration: feature.cooldown_duration,
                })),
            );
        }
    }
}

/// Resets the [`CoolingTimer`] on a skill entity when its [`SkillEvent`]
/// is received (all features completed), so the cooldown begins anew.
pub fn reset_cooldown_timer(
    mut events: MessageReader<SkillEvent>,
    mut query: Query<&mut CoolingTimer>,
) {
    for event in events.read() {
        if let Ok(mut timer) = query.get_mut(event.skill) {
            timer.0.reset();
        }
    }
}
