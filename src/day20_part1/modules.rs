use std::collections::{HashMap};
use std::fmt::Debug;
use std::ops::Not;
use std::string::ToString;
use itertools::Itertools;
use PulseType::*;
use SwitchState::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PulseType { Low, High }

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum SwitchState { Off, On }

impl Not for SwitchState {
    type Output = Self;

    fn not(self) -> Self::Output {
        return match self {
            Off => On,
            On => Off
        }
    }
}

pub type PulseOutput = (String, String, PulseType);

pub trait PulseReceiver: Debug {
    fn process_input_pulse(&mut self, source: &String, input_pulse: PulseType) -> (u64, u64, Vec<PulseOutput>);
}

#[derive(Debug)]
pub struct Button {}

impl Button {
    pub fn push(&self) -> (u64, u64, Vec<PulseOutput>) {
        return (1, 0, vec![("".to_string(), Broadcaster::NAME.to_string(), Low)])
    }
}

#[derive(Debug)]
pub struct Broadcaster {
    pub name: String,
    outputs: Vec<String>
}

impl Broadcaster {
    pub const NAME: &'static str = "broadcaster";

    pub fn new(outputs: &Vec<String>) -> Self {
        return Broadcaster {
            name: Broadcaster::NAME.to_string(),
            outputs: outputs.clone() };
    }
}

impl PulseReceiver for Broadcaster {
    fn process_input_pulse(&mut self, _source: &String, input_pulse: PulseType) -> (u64, u64, Vec<PulseOutput>) {
        let mut low_pulses_sent = 0u64;
        let mut high_pulses_sent = 0u64;

        let pulse_queue: Vec<PulseOutput> = self.outputs
            .iter()
            .map(|x| (self.name.clone(), x.clone(), input_pulse))
            .collect();

        match input_pulse {
            Low => low_pulses_sent += self.outputs.len() as u64,
            High => high_pulses_sent += self.outputs.len() as u64
        }

        return (low_pulses_sent, high_pulses_sent, pulse_queue);
    }
}

#[derive(Debug)]
pub struct FlipFlop {
    pub name: String,
    state: SwitchState,
    outputs: Vec<String>,
}

impl FlipFlop {
    pub fn new(name: &str, outputs: &Vec<String>) -> Self {
        const DEFAULT_STATE: SwitchState = Off;

        return FlipFlop {
            name: name.to_string(),
            state: DEFAULT_STATE,
            outputs: outputs.clone() };
    }
    
    pub fn state(&self) -> SwitchState {
        return self.state;
    }
}

impl PulseReceiver for FlipFlop {
    fn process_input_pulse(&mut self, _source: &String, input_pulse: PulseType) -> (u64, u64, Vec<PulseOutput>) {
        let mut low_pulses_sent = 0u64;
        let mut high_pulses_sent = 0u64;
        let mut pulse_queue = Vec::<PulseOutput>::new();

        if input_pulse == Low {
            self.state = !self.state;

            let emitted_pulse = match self.state {
                Off => Low,
                On => High
            };
            pulse_queue.extend(self.outputs.iter().map(|x| (self.name.clone(), x.clone(), emitted_pulse)));

            match emitted_pulse {
                Low => low_pulses_sent += 1,
                High => high_pulses_sent += 1
            }
        }

        return (low_pulses_sent, high_pulses_sent, pulse_queue);
    }
}

#[derive(Debug)]
pub struct Conjunction {
    name: String,
    inputs: HashMap<String, PulseType>,
    outputs: Vec<String>
}

impl Conjunction {
    pub fn new(name: &String, input_names: &Vec<String>, outputs: &Vec<String>) -> Self {
        const DEFAULT_INPUT_STATE: PulseType = Low;

        return Conjunction {
            name: name.to_string(),
            inputs: input_names.iter().map(|x| (x.clone(), DEFAULT_INPUT_STATE)).collect(),
            outputs: outputs.clone() };
    }
}

impl PulseReceiver for Conjunction {
    fn process_input_pulse(&mut self, source: &String, input_pulse: PulseType) -> (u64, u64, Vec<PulseOutput>) {
        let mut low_pulses_sent = 0u64;
        let mut high_pulses_sent = 0u64;

        *self.inputs.get_mut(source).unwrap() = input_pulse;
        let emitted_pulse = if self.inputs.iter().map(|x| x.1).all(|x| *x == High) {
            Low
        } else {
            High
        };

        let pulse_queue = self.outputs
            .iter()
            .map(|x| (self.name.clone(), x.clone(), emitted_pulse))
            .collect_vec();

        match emitted_pulse {
            Low => low_pulses_sent += self.outputs.len() as u64,
            High => high_pulses_sent += self.outputs.len() as u64
        }

        return (low_pulses_sent, high_pulses_sent, pulse_queue);
    }
}
