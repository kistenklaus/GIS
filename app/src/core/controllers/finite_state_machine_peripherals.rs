use crate::core::controllers::battery_controller::BatteryController;
use crate::core::controllers::breaking_controller::BrakingController;
use crate::core::controllers::can_controller::{CanController, CanPins};
use crate::core::controllers::ethernet_controller::{EthernetController, EthernetPins};
use crate::core::controllers::hv_controller::HVPeripherals;
use crate::core::finite_state_machine::Fsm;
use crate::{DataReceiver, EventSender, InternalMessaging};
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::flash::Error::Size;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals::{FDCAN1, FDCAN2};
use embassy_stm32::{bind_interrupts, can, eth, peripherals, rng, Peripheral, Peripherals};
use embassy_sync::blocking_mutex::raw::{NoopRawMutex, RawMutex, ThreadModeRawMutex};
use embassy_sync::mutex::Mutex;
use embassy_sync::priority_channel::{PriorityChannel, Sender};
use embassy_time::{Duration, Instant, Timer};
use heapless::binary_heap::Max;

pub struct FSMPeripherals {
    pub braking_controller: BrakingController,
    pub eth_controller: EthernetController,
    pub can_controller: CanController,
    pub hv_peripherals: HVPeripherals,
    //  pub lv_controller: BatteryController,
    pub red_led: Output<'static>,
}

impl FSMPeripherals {
    // pub fn new(p : Peripherals, x: &Spawner, q : &PriorityChannel<NoopRawMutex, Event, Max, 16>) -> Self {
    pub async fn new(p: Peripherals, x: &Spawner, i: InternalMessaging) -> Self {
        // let mut init = PInit{p,x,q};
        // let (braking_controller, init) = BrakingController::new(init);
        let braking_controller = BrakingController::new(
            x,
            i.event_sender,
            p.PB8,
            p.PG1,
            p.PF12,
            p.PB0,
            p.PD5,
            p.TIM16,
        )
        .await;

        let mut hv_controller = BatteryController::new(
            i.event_sender,
            0,
            0,
            0,
            0,
            0,
            i.data_sender,
            true,
        ); //TODO <------ This is just to make it build
        let mut lv_controller = BatteryController::new(
            i.event_sender,
            0,
            0,
            0,
            0,
            0,
            i.data_sender,
            false,
        ); //TODO <------ This is just to make it build

        let mut eth_controller = EthernetController::new(
            *x,
            i.event_sender,
            i.data_receiver,
            EthernetPins {
                p_rng: p.RNG,
                eth_pin: p.ETH,
                pg11_pin: p.PG11,
                pb13_pin: p.PB13,
                pg13_pin: p.PG13,
                pc5_pin: p.PC5,
                pc4_pin: p.PC4,
                pa7_pin: p.PA7,
                pc1_pin: p.PC1,
                pa2_pin: p.PA2,
                pa1_pin: p.PA1,
            },
        )
        .await;
        // let mut b = Output::new(p.PB0, Level::High, Speed::High);
        // b.set_high();
        let mut can_controller = CanController::new(
            *x,
            i.event_sender,
            i.data_sender,
            i.data_receiver,
            i.can_one_sender,
            i.can_one_receiver,
            i.can_two_sender,
            i.can_two_receiver,
            CanPins {
                fdcan1: p.FDCAN1,
                fdcan2: p.FDCAN2,
                pd0_pin: p.PD0,
                pd1_pin: p.PD1,
                pb5_pin: p.PB5,
                pb6_pin: p.PB6,
            },
            hv_controller,
            lv_controller,
        )
        .await;

        Self {
            braking_controller,
            eth_controller,
            can_controller,
            hv_peripherals: HVPeripherals::new(p.PB4).await,
            red_led: Output::new(p.PB14, Level::Low, Speed::High),
        }
    }
}