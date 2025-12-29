use bevy::prelude::*;
use std::time::Duration;

use super::timeline::Timeline;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    FadeIn,
    Hold,
    FadeOut,
    Finished,
}

#[derive(Resource, Debug)]
pub struct SequenceState {
    pub phase: Phase,
    pub elapsed: Duration,
    pub timeline: Timeline,
}

#[derive(Resource)]
pub struct PhaseProgress {
    pub phase: Phase,
    pub t: f32, // always 0.0 → 1.0
}

// We need a small internal resource so we don’t trigger the transition multiple times.
#[derive(Resource)]
pub struct TransitionTarget<S: States> {
    pub state: S,
    pub triggered: bool,
}
