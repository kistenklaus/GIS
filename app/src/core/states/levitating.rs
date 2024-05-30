use crate::core::finite_state_machine::{State, Fsm};
use crate::{transit, Event};
use defmt::info;

impl Fsm {
    pub fn entry_levitating(&mut self) {
        todo!();
    }

    pub async fn react_levitating(&mut self, event: Event) {
        match event {
            Event::StartAcceleratingCommand => {
                todo!(); // TODO: send message to propulsion to start

                transit!(self, State::MovingST);
            }
            Event::HVPropulsionReadyEvent => {
                todo!();
            }
            _ => {
                info!("The current state ignores {}", event.to_str());
            }
        }
    }
}