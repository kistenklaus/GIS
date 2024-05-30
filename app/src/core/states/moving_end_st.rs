use crate::core::finite_state_machine::{State, Fsm};
use crate::{transit, Event};
use defmt::info;

impl Fsm {
    pub fn entry_end_st(&mut self) {
        todo!();
    }

    pub async fn react_end_st(&mut self, event: Event) {
        match event {
            Event::DirectionChangedEvent => {
                todo!();
            }
            Event::RunFinishedEvent => {
                #[cfg(debug_assertions)]
                info!("Run finished");

                transit!(self, State::Exit);
            }
            /// This is commented out because it was refactored to be handled by the default react ///
            // Event::LevitationErrorEvent|Event::PropulsionErrorEvent|Event::PowertrainErrorEvent |Event::ConnectionLossEvent|Event::EmergencyBrakeCommand=> {
            //
            //     todo!();
            //
            //     self.transit(State::EmergencyBraking)
            // }
            _ => {
                info!("The current state ignores {}", event.to_str());
            }
        }
    }
}