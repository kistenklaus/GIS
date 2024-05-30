use crate::core::communication::Datapoint;
use crate::core::finite_state_machine::{State, Fsm};
use crate::{transit, Datatype, Event};
use defmt::{error, info};

impl Fsm {
    pub fn boot_entry(&mut self) {
        info!("Entering Boot State");

        if !self.peripherals.braking_controller.arm_breaks() {
            self.react(Event::BootingFailedEvent);
        }
        info!("Booting complete");
        // TODO -> Start also the connection with the sensor hub ||| I think this will be a task that starts can bus 1 and 2
    }

    pub async fn react_boot(&mut self, event: Event) {
        match event {
            Event::BootingCompleteEvent => {
                info!("Booting complete");

                transit!(self, State::EstablishConnection);
            }
            Event::BootingFailedEvent => {
                error!("Booting failed!!");

                transit!(self, State::Exit);
            }
            _ => {
                info!("Booting state ignores {}", event.to_str());
            }
        }
    }
}