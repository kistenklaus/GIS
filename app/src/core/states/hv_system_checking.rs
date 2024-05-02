use defmt::info;
use crate::core::finite_state_machine::*;
use crate::Event;

impl FSM{
    pub fn entry_hv_system_checking(&mut self) {
        todo!();
    }

    pub fn react_hv_system_checking(&mut self, event: Event) {
        match event {
            Event::HVLevitationReadyEvent => {

                self.status.check_levitation();


            }
            Event::HVPowertrainReadyEvent => {

                self.status.check_powertrain();


            }
            Event::HVPropulsionReadyEvent => {

                self.status.check_propulsion();


            }
            Event::StartLevitatingCommand => {

                if (self.status.check_all()) {


                    self.transit(State::Levitating);
                }
                todo!();

                self.transit(State::Levitating)
            }
            /// This is commented out because it was refactored to be handled by the default react ///
            // /// Error Events that are core from all states that HV is on
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