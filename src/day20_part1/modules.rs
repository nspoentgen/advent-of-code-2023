use std::collections::{HashMap};
use std::ops::Neg;
use std::string::ToString;
use PulseType::*;
use SwitchState::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum PulseType { Low, High }

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SwitchState { Off, On }

impl Neg for SwitchState {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return match self {
            Off => On,
            On => Off
        }
    }
}

pub type PulseOutput = (String, PulseType);

pub trait Module {
    fn process_pulse(&mut self, source: &String, input_pulse: PulseType) -> (u64, u64, Vec<PulseOutput>);
}

pub struct Button {}

impl Button {
    pub fn push() -> (u64, u64, Vec<PulseOutput>) {
        return (1, 0, vec![(Broadcaster::NAME.to_string(), Low)])
    }
}

pub struct Broadcaster {
    outputs: Vec<String>
}

impl Broadcaster {
    pub const NAME: &'static str = "broadcaster";

    pub fn new(outputs: &Vec<String>) -> Self {
        return Broadcaster { outputs: outputs.clone() };
    }
}

impl Module for Broadcaster {
    fn process_pulse(&mut self, source: &String, input_pulse: PulseType) -> (u64, u64, Vec<PulseOutput>) {
        let mut low_pulses_sent = 0u64;
        let mut high_pulses_sent = 0u64;

        let pulse_queue: Vec<PulseOutput> = self.outputs
            .iter()
            .map(|x| (x.clone(), input_pulse))
            .collect();

        match input_pulse {
            Low => low_pulses_sent += self.outputs.len() as u64,
            High => high_pulses_sent += self.outputs.len() as u64
        }

        return (low_pulses_sent, high_pulses_sent, pulse_queue);
    }
}

pub struct FlipFlop {
    state: SwitchState,
    outputs: Vec<String>,
}

impl FlipFlop {
    pub fn new(outputs: &Vec<String>) -> Self {
        return FlipFlop {state: Off, outputs: outputs.clone() };
    }
    
    pub fn state(&self) -> SwitchState {
        return self.state;
    }
}

impl Module for FlipFlop {
    fn process_pulse(&mut self, source: &String, input_pulse: PulseType) -> (u64, u64, Vec<PulseOutput>) {
        let mut low_pulses_sent = 0u64;
        let mut high_pulses_sent = 0u64;
        let mut pulse_queue = Vec::<PulseOutput>::new();

        if input_pulse == Low {
            self.state != self.state;

            let emitted_pulse = match self.state {
                Off => Low,
                On => High
            };
            pulse_queue.extend(self.outputs.iter().map(|x| (x.clone(), emitted_pulse)));

            match emitted_pulse {
                Low => low_pulses_sent += 1,
                High => high_pulses_sent += 1
            }
        }

        return (low_pulses_sent, high_pulses_sent, pulse_queue);
    }
}

pub struct Conjunction {
    inputs: HashMap<String, PulseType>,
    outputs: Vec<String>
}

impl Conjunction {
    pub fn new(outputs: &Vec<String>) -> Self {
        return Conjunction { inputs: HashMap::<String, PulseType>::new(), outputs: outputs.clone() };
    }
}

impl Module for Conjunction {
    fn process_pulse(&mut self, source: &String, input_pulse: PulseType) -> (u64, u64, Vec<PulseOutput>) {
        let mut low_pulses_sent = 0u64;
        let mut high_pulses_sent = 0u64;
        let mut pulse_queue = Vec::<PulseOutput>::new();

        *self.inputs.get_mut(source).unwrap() = input_pulse;
        let emitted_pulse = if self.inputs.iter().map(|x| x.1).all(|x| *x == High) {
            Low
        } else {
            High
        };

        match emitted_pulse {
            Low => low_pulses_sent += 1,
            High => high_pulses_sent += 1
        }

        return (low_pulses_sent, high_pulses_sent, pulse_queue);
    }
}
