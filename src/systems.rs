use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

use super::structs::*;
use std::time::Duration;

pub fn advance_phase(
    time: Res<Time>,
    mut state: ResMut<SequenceState>,
    mut progress: ResMut<PhaseProgress>,
) {
    state.elapsed += time.delta();

    let duration = match state.phase {
        Phase::FadeIn => state.timeline.fade_in,
        Phase::Hold => state.timeline.hold,
        Phase::FadeOut => state.timeline.fade_out,
        Phase::Finished => return,
    };

    progress.t = (state.elapsed.as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0);

    if progress.t >= 1.0 {
        state.elapsed = Duration::ZERO;
        state.phase = match state.phase {
            Phase::FadeIn => Phase::Hold,
            Phase::Hold => Phase::FadeOut,
            Phase::FadeOut => Phase::Finished,
            Phase::Finished => Phase::Finished,
        };
        progress.phase = state.phase;
    }
}

pub fn enter_next_state<S: States + FreelyMutableState>(
    progress: Res<PhaseProgress>,
    target: Option<ResMut<TransitionTarget<S>>>,
    mut next_state: ResMut<NextState<S>>,
) {
    let Some(mut target) = target else {
        return;
    };

    if target.triggered {
        return;
    }

    if progress.phase == Phase::Finished {
        next_state.set(target.state.clone());
        target.triggered = true;
    }
}
