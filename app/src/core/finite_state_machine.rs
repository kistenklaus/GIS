use defmt::*;
use embassy_stm32;
use embassy_stm32::Peripherals;
use heapless::Deque;
use crate::core::finite_state_machine_peripherals::FSMPeripherals;


//Enum holding different states that the FSM can be in
pub enum State {
    Boot,
    EstablishConnection,
    RunConfig,
    Idle,
    HVSystemChecking,
    Levitating,
    Accelerating,
    Cruising,
    LaneSwitch,
    Braking,
    EmergencyBraking,
    Exit,
    Crashing,
}


//All the functionalities states can have like converting to id's or format print statements should go here
// The actual implementation of each state should just be attached to the state machine in separate files
impl State {
    pub fn fmt(&self) {
        match self {
            State::Boot => info!("Boot"),
            State::EstablishConnection => info!("EstablishConnection"),
            State::Idle => info!("Idle"),
            State::HVSystemChecking => info!("HVSystemChecking"),
            State::Levitating => info!("Levitating"),
            State::Accelerating => info!("Accelerating"),
            State::Cruising => info!("Cruising"),
            State::LaneSwitch => info!("LaneSwitching"),
            State::Braking => info!("Braking"),
            State::EmergencyBraking => info!("EmergencyBraking"),
            State::Exit => info!("Quit"),
            State::Crashing => info!("Crashing"),
            _ => info! { "Unknown"},
        }
    }
}


//Enum holding different events that the FSM can react to

pub enum Event {
    DefaultEvent,

    // Boot state related events
    BootingCompleteEvent,
    BootingFailedEvent,

    // Establish Connection state related events
    ConnectionEstablishedEvent,
    ConnectionEstablishmentFailedEvent,

    // Run Config state related events
    RunConfigCompleteEvent,
    RunConfigFailedEvent,

    //Error Handling events
    LevitationErrorEvent,
    PropulsionErrorEvent,
    PowertrainErrorEvent,
    EmergencyBrakeCommand,
    ConnectionLossEvent,

    // Idle state related events
    LVLevitationReadyEvent,
    LVPropulsionReadyEvent,
    LVPowertrainReadyEvent,
    ArmBrakesCommand,
    TurnOnHVCommand,


    // HV System Checking state
    HVPowertrainReadyEvent,
    HVLevitationReadyEvent,
    StartLevitatingCommand,

    // Levitating
    HVPropulsionReadyEvent,
    StartAcceleratingCommand,

    // Accelerating
    DesiredSpeedReachedEvent,
    BrakingPointReachedEvent,

    // Cruising
    LaneSwitchingPointReachedEvent,


    //LaneSwitching
    LaneSwitchingCompleteEvent,

    // Braking
    DirectionChangedEvent,
    RunFinishedEvent,

    // EmergencyBraking
    SystemResetCommand,

    ExitEvent,
}
// Again some functionalities that might be useful for the events to have mut be here
impl Event {
    pub fn fmt(&self) {
        match self {
            Event::BootingCompleteEvent => info!("BootingCompleteEvent"),
            Event::BootingFailedEvent => info!("BootingFailedEvent"),
            Event::ConnectionEstablishedEvent => info!("ConnectionEstablishedEvent"),
            Event::ConnectionEstablishmentFailedEvent => { info!("ConnectionEstablishmentFailedEvent") }
            Event::LVLevitationReadyEvent => info!("LVLevitationReadyEvent"),
            Event::LVPropulsionReadyEvent => info!("LVPropulsionReadyEvent"),
            Event::LVPowertrainReadyEvent => info!("LVPowertrainReadyEvent"),
            Event::HVPowertrainReadyEvent => info!("HVPowertrainReadyEvent"),
            Event::HVLevitationReadyEvent => info!("HVLevitationReadyEvent"),
            Event::StartLevitatingCommand => info!("StartLevitatingCommand"),
            Event::HVPropulsionReadyEvent => info!("HVPropulsionReadyEvent"),
            Event::StartAcceleratingCommand => info!("StartAcceleratingCommand"),
            Event::DesiredSpeedReachedEvent => info!("DesiredSpeedReachedEvent"),
            Event::BrakingPointReachedEvent => info!("BrakingPointReachedEvent"),
            Event::DirectionChangedEvent => info!("DirectionChangedEvent"),
            Event::SystemResetCommand => info!("SystemResetCommand"),
            Event::LevitationErrorEvent => info!("LevitationErrorEvent"),
            Event::PropulsionErrorEvent => info!("PropulsionErrorEvent"),
            Event::PowertrainErrorEvent => info!("PowertrainErrorEvent"),
            Event::EmergencyBrakeCommand => info!("EmergencyBrakeCommand"),
            Event::RunFinishedEvent => info!("RunFinishedEvent"),
            Event::LaneSwitchingPointReachedEvent => info!("LaneSwitchingPointReachedEvent"),
            Event::LaneSwitchingCompleteEvent => info!("LaneSwitchingCompleteEvent"),
            Event::ConnectionLossEvent => info!("ConnectionLossEvent"),
            Event::ArmBrakesCommand => info!("ArmBrakesCommand"),
            Event::ExitEvent => info!("QuitEvent"),
            Event::TurnOnHVCommand => info!("TurnOnHVCommand"),
            Event::RunConfigFailedEvent => info!("Run configuration failed"),
            Event::RunConfigCompleteEvent => info!("RunConfigComplete"),
            Event::DefaultEvent => info!("DefaultEvent"),
            _ => info! {"Unknown"},
        }
    }
}

//TODO: Add all the parameters that the FSM might need to have
// This bad boy will be a singleton and it will be passed around everywhere
// Just be careful with the STD's

use core::cmp::{Ordering, PartialEq, Eq};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::priority_channel::PriorityChannel;
use heapless::binary_heap::Max;

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.eq(&Event::EmergencyBrakeCommand) || (self.eq(&Event::LevitationErrorEvent) || (self.eq(&Event::PropulsionErrorEvent) || (self.eq(&Event::PowertrainErrorEvent) || (self.eq(&Event::ConnectionLossEvent)))))){
            return Ordering::Greater;
        }
        else if (other.eq(&Event::EmergencyBrakeCommand) || (other.eq(&Event::LevitationErrorEvent) || (other.eq(&Event::PropulsionErrorEvent) || (other.eq(&Event::PowertrainErrorEvent) || (other.eq(&Event::ConnectionLossEvent)))))){
            return Ordering::Less;
        }
        else {
            return Ordering::Equal;
        }
    }
}



pub struct FSM {
    state: State,
    pub peripherals: FSMPeripherals,
    pub event_queue: PriorityChannel<NoopRawMutex,Event,Max,16>,
}


/**
    * Finite State Machine (FSM) for the DH08 POD -> Helios III
    * This FSM is a singleton and an entity. Its name is Megalo coming from the ancient greek word for "Big" and from the gods Megahni and Gonzalo
**/
impl FSM {
    pub fn new(p : FSMPeripherals, pq :PriorityChannel<NoopRawMutex,Event,Max,16>, ) -> Self {

        //TODO: Decide if main should be dirty with peripheral initialization or if it should be done here

        Self {
            state: State::Boot,
            peripherals:p,
            event_queue: pq
        }
    }

    // pub  fn push_event(&mut self, event: Event) {
    //     self.event_queue.send(event).await;
    // }
    // pub fn pop_event(&mut self) -> Event {
    //     let res = self.event_queue.pop_front();
    //     match res {
    //         Some(event) => {
    //             return event
    //         }
    //         None => {
    //             Event::DefaultEvent
    //         }
    //     }
    // }

/**
    * Function used to transit states of Megalo --> Comes from Megahni and Gonzalo
**/
    pub fn transit(&mut self, next_state: State) {

        info!("Exiting state: ");
        self.state.fmt();
        info!("Entering state: ");
        next_state.fmt();
        self.state = next_state;
        self.entry();
    }
    pub fn entry(&mut self) {
        match self.state {
            State::Boot => {
                self.boot_entry();
            }
            State::EstablishConnection => {
                self.entry_establish_connection();
            }
            State::Idle => {
                self.entry_idle();
            }
            State::RunConfig => {
                self.entry_run_config();
            }
            State::HVSystemChecking => {
                self.entry_hv_system_checking();
            }
            State::Levitating => {
                self.entry_levitating();
            }
            State::Accelerating => {
                self.entry_accelerating();
            }
            State::Cruising => {
                self.entry_cruising();
            }
            State::LaneSwitch => {
                self.entry_lane_switch();
            }
            State::Braking => {
                self.entry_braking();
            }
            State::EmergencyBraking => {
                self.entry_emergency_braking();
            }
            State::Exit => {
                self.entry_exit();
            }
            _ => {
                info!("Im going against a wall for sure this time");//<--- This is what happens if the Crashing state ever gets triggered
            }
        }
    }
    pub(crate) fn react(&mut self, event: Event) {
        match event {
            Event::LevitationErrorEvent|Event::PropulsionErrorEvent|Event::PowertrainErrorEvent |Event::ConnectionLossEvent|Event::EmergencyBrakeCommand=> {
                self.transit(State::EmergencyBraking);
                return;
            }
            _ => {

            }
        }
        match self.state {
            State::Boot => {
                self.react_boot(event);
            }
            State::EstablishConnection => {
                self.react_establish_connection(event);
            }
            State::Idle => {
                self.react_idle(event);
            }
            State::RunConfig => {
                self.react_run_config(event);
            }
            State::HVSystemChecking => {
                self.react_hv_system_checking(event);
            }
            State::Levitating => {
                self.react_levitating(event);
            }
            State::Accelerating => {
                self.react_accelerating(event);
            }
            State::Cruising => {
                self.react_cruising(event);
            }
            State::LaneSwitch => {
                self.react_lane_switch(event);
            }
            State::Braking => {
                self.react_braking(event);
            }
            State::Exit => {
                self.react_exit(event);
            }
            _ => {
                info!("Unknown state"); // <---- Kiko: Im forced to have this here but im against it
            }
        }
    }
}
