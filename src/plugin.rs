use super::structs::*;
use super::systems::*;
use super::timeline::Timeline;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use std::time::Duration;

pub struct SplashKitPlugin<S: States>
where
    S: States + FreelyMutableState,
{
    timeline: Timeline,
    next_state: Option<S>,
}
impl<S: States> Plugin for SplashKitPlugin<S>
where
    S: States + FreelyMutableState,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(SequenceState {
            phase: Phase::FadeIn,
            elapsed: Duration::ZERO,
            timeline: self.timeline.clone(),
        })
        .insert_resource(PhaseProgress {
            phase: Phase::FadeIn,
            t: 0.0,
        });

        app.add_systems(Update, advance_phase);

        if let Some(state) = self.next_state.clone() {
            app.insert_resource(TransitionTarget {
                state,
                triggered: false,
            })
            .add_systems(Update, enter_next_state::<S>);
        }
    }
}

impl<S: States> SplashKitPlugin<S>
where
    S: States + FreelyMutableState,
{
    pub fn new(timeline: Timeline) -> Self {
        Self {
            timeline,
            next_state: None,
        }
    }

    pub fn then_enter(mut self, state: S) -> Self {
        self.next_state = Some(state);
        self
    }
}
