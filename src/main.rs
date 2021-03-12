use std::marker::PhantomData;

const N_CYCLES_MAINTENANCE: u64 = 1000;

fn main() {
    let red_light = TrafficLight::<Red>::turn_on();
    let mut green_light = red_light.to_green();
    if green_light.requires_maintenance() {
        green_light.reset_cycles()
    }
    let yellow_light = green_light.to_yellow();
    let red_light = yellow_light.to_red();
    red_light.turn_off();
}

pub struct TrafficLight<State>
where
    State: TrafficLightState,
{
    cycles: u64,
    // since state is not read
    state: PhantomData<State>,
}

pub struct Green;
pub struct Yellow;
pub struct Red;

mod private {
    use crate::{Green, Red, Yellow};
    pub trait Private {}
    impl Private for Green {}
    impl Private for Yellow {}
    impl Private for Red {}
}

pub trait TrafficLightState: private::Private {}

impl TrafficLightState for Green {}
impl TrafficLightState for Yellow {}
impl TrafficLightState for Red {}

pub trait GreenState {
    fn to_yellow(self) -> TrafficLight<Yellow>;
}
pub trait YellowState {
    fn to_red(self) -> TrafficLight<Red>;
}
pub trait RedState {
    fn to_green(self) -> TrafficLight<Green>;
    fn turn_on() -> TrafficLight<Red>;
    fn turn_off(self);
}

impl GreenState for TrafficLight<Green> {
    fn to_yellow(self) -> TrafficLight<Yellow> {
        TrafficLight::<Yellow> {
            cycles: self.cycles,
            state: PhantomData,
        }
    }
}

impl YellowState for TrafficLight<Yellow> {
    fn to_red(self) -> TrafficLight<Red> {
        TrafficLight::<Red> {
            // increment the cycle
            cycles: self.cycles + 1,
            state: PhantomData,
        }
    }
}

impl RedState for TrafficLight<Red> {
    fn to_green(self) -> TrafficLight<Green> {
        TrafficLight::<Green> {
            cycles: self.cycles,
            state: PhantomData,
        }
    }

    fn turn_on() -> TrafficLight<Red> {
        TrafficLight::<Red> {
            cycles: 0,
            state: PhantomData,
        }
    }

    fn turn_off(self) {
        // ... consume
    }
}

impl<State> TrafficLight<State>
where
    State: TrafficLightState,
{
    fn requires_maintenance(&self) -> bool {
        self.cycles > N_CYCLES_MAINTENANCE
    }

    fn reset_cycles(&mut self) {
        self.cycles = 0;
    }
}
