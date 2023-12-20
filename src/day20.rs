use core::fmt;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;
use num::Integer;

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day20_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let press_count = 1000;
    let print_out_pulses = false;

    let mut module_configuration = ModuleConfiguration::parse(&input)?;

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    for i in 1..press_count + 1 {
        let pulses = module_configuration.press_button()?;

        if print_out_pulses {
            println!("button press #{}:", i);
        }
        for pulse in pulses {
            match pulse.state {
                PulseState::Low => low_pulse_count += 1,
                PulseState::High => high_pulse_count += 1,
            }
            if print_out_pulses {
                println!("\t{}", pulse);
            }
        }
    }

    Ok((low_pulse_count * high_pulse_count).to_string())
}

fn part_two() -> Result<String, AdventError> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut modules = ModuleConfiguration::parse(&input)?;

    // ** NOTE: **
    //
    // This solution, unlike all the previous ones, is not generalizable or extensible.
    //
    // This is a cheap hack, based on my specific input. I did not feel like investing time
    // to properly analyze the input in a generic way.
    //
    // This functions ONLY in the case of three very specific assumptions being true, which
    // were hand-picked based on MY specific input. These are the assumptions:
    //
    // 1. The final destination module ("rx") has EXACTLY ONE input, named "df".
    // 2. That penultimate module ("df") is a Conjunction module
    // 3. The output frequencies of all the inputs to "df" can be found by iterating less that 5,000 times
    //
    // This code exploits these assumptions to come to the correct solution in a reasonable time.
    //
    // Also of note: due to these assumptions, this part only works on the input, not an any
    // of the provided examples.

    // get the inputs for the final module
    let inputs_for_final = modules.inputs_for("df")?;

    // find the output frequencies for all the inputs to the final conjunction module
    let modules_to_analyze = inputs_for_final;
    let output_frequencies = modules.output_frequencies_for(
        modules_to_analyze
            .iter()
            .map(|s| s.to_string())
            .collect_vec()
            .as_slice(),
        PulseState::High,
        5_000,
    )?;

    // for (output, freq) in output_frequencies.iter() {
    //     println!("{}: {:?}", output, freq);
    // }

    // make sure we actually got an output frequency from all of them within the limit
    let validated_output_frequencies: Vec<(String, u64)> = output_frequencies
        .into_iter()
        .map(|(k, v)| {
            let validated_freq = v.ok_or(format!("output {} has no frequency", k))?;
            Ok::<(String, u64), String>((k, validated_freq))
        })
        .try_collect()?;

    // get the final frequency, which is the Lowest Common Multiple of all the output frequencies
    let frequency_for_final = validated_output_frequencies
        .into_iter()
        .map(|(_, v)| v)
        .reduce(|acc, freq| acc.lcm(&freq))
        .unwrap();

    Ok(frequency_for_final.to_string())
}

struct ModuleConfiguration {
    modules: HashMap<String, Box<dyn Module>>,
}

impl ModuleConfiguration {
    fn parse(input: &str) -> Result<ModuleConfiguration, String> {
        let mut modules = HashMap::new();
        for line in input.lines() {
            let module: Box<dyn Module> = match line.chars().next().unwrap() {
                'b' => Broadcaster::parse(line)?,
                '%' => FlipFlop::parse(line)?,
                '&' => Conjunction::parse(line)?,
                _ => return Err(format!("invalid line: {}", line)),
            };
            modules.insert(module.name().to_string(), module);
        }

        // get all the outputs for all modules
        let mut outputs = HashMap::new();
        for module in modules.values() {
            outputs.insert(module.name().to_string(), module.outputs().clone());
        }

        // we have to "hook up" the inputs of all the Conjunction modules
        for module in modules.values_mut() {
            let name = module.name().to_string();
            let mut inputs = Vec::new();
            for (potential_input_name, potential_input_outputs) in outputs.iter() {
                if potential_input_outputs.contains(&name) {
                    inputs.push(potential_input_name.clone());
                }
            }
            module.set_inputs(inputs.as_slice());
        }

        Ok(ModuleConfiguration { modules })
    }

    fn press_button(&mut self) -> Result<Vec<Pulse>, String> {
        let initial_pulse = Pulse {
            state: PulseState::Low,
            from: "button".to_string(),
            to: "broadcaster".to_string(),
        };

        let mut pulse_record = vec![];

        let mut pulses_to_resolve = VecDeque::new();
        pulses_to_resolve.push_back(initial_pulse);

        while let Some(pulse) = pulses_to_resolve.pop_front() {
            if let Some(destination) = self.modules.get_mut(&pulse.to) {
                let new_pulses = destination.handle_pulse(&pulse);
                pulses_to_resolve.extend(new_pulses);
            }

            pulse_record.push(pulse);
        }

        Ok(pulse_record)
    }

    fn _output_frequencies_for_all(
        &mut self,
        state: PulseState,
        limit: u64,
    ) -> Result<HashMap<String, Option<u64>>, String> {
        let all_modules = self.modules.keys().map(|x| x.clone()).collect_vec();
        self.output_frequencies_for(all_modules.as_slice(), state, limit)
    }

    fn output_frequencies_for(
        &mut self,
        modules: &[String],
        state: PulseState,
        limit: u64,
    ) -> Result<HashMap<String, Option<u64>>, String> {
        let mut cache = HashMap::new();
        for name in modules {
            cache.insert(name.clone(), None);
        }

        self.output_frequencies(cache, state, limit)

        // let module = self
        //     .modules
        //     .get(name)
        //     .ok_or(format!("module not found: {}", name))?;

        // Ok(module.pulse_frequency(state, &self.modules))
    }

    fn output_frequencies(
        &mut self,
        mut cache: HashMap<String, Option<u64>>,
        state: PulseState,
        limit: u64,
    ) -> Result<HashMap<String, Option<u64>>, String> {
        // let mut output_freqs = HashMap::new();
        // output_freqs.insert("button".to_string(), None);
        // for name in self.modules.keys() {
        //     output_freqs.insert(name.clone(), None);
        // }

        let mut press_count = 1;
        loop {
            let pulses = self.press_button()?;
            for pulse in pulses {
                if pulse.state == state {
                    if let Some(output_freq) = cache.get_mut(&pulse.from) {
                        if output_freq.is_none() {
                            *output_freq = Some(press_count);
                        }
                    }
                }
            }

            press_count += 1;
            if press_count > limit || cache.values().all(|x| x.is_some()) {
                break;
            }
        }
        Ok(cache)
    }

    fn inputs_for(&self, name: &str) -> Result<Vec<&String>, String> {
        let module = self
            .modules
            .get(name)
            .ok_or(format!("module not found: {}", name))?;

        Ok(module.inputs())
    }

    fn _outputs_for(&self, name: &str) -> Result<&Vec<String>, String> {
        let module = self
            .modules
            .get(name)
            .ok_or(format!("module not found: {}", name))?;

        Ok(module.outputs())
    }
}

trait Module {
    fn name(&self) -> &str;
    fn inputs(&self) -> Vec<&String>;
    fn outputs(&self) -> &Vec<String>;
    fn set_inputs(&mut self, inputs: &[String]);
    fn pulse_frequency(&self, state: PulseState, modules: &HashMap<String, Box<dyn Module>>)
        -> u64;
    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pulse {
    state: PulseState,
    from: String,
    to: String,
}

impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -{}-> {}", self.from, self.state, self.to)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PulseState {
    High,
    Low,
}

impl fmt::Display for PulseState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PulseState::High => write!(f, "high"),
            PulseState::Low => write!(f, "low"),
        }
    }
}

struct Broadcaster {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Broadcaster {
    fn parse(line: &str) -> Result<Box<dyn Module>, String> {
        let (declaration, outputs) = line
            .split_once(" -> ")
            .ok_or(format!("invalid line: {}", line))?;

        let name = declaration.to_string();
        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();

        Ok(Box::new(Broadcaster {
            name,
            inputs: Vec::new(),
            outputs,
        }))
    }

    fn pulse_input_frequency(
        &self,
        state: PulseState,
        modules: &HashMap<String, Box<dyn Module>>,
    ) -> u64 {
        let mut min_input_freq = u64::MAX;
        for input in &self.inputs {
            let input_freq = modules.get(input).unwrap().pulse_frequency(state, modules);
            if input_freq < min_input_freq {
                min_input_freq = input_freq;
            }
        }
        min_input_freq
    }
}

impl Module for Broadcaster {
    fn name(&self) -> &str {
        &self.name
    }

    fn inputs(&self) -> Vec<&String> {
        self.inputs.iter().collect_vec()
    }

    fn outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn set_inputs(&mut self, inputs: &[String]) {
        self.inputs = inputs.to_vec();
    }

    fn pulse_frequency(
        &self,
        state: PulseState,
        modules: &HashMap<String, Box<dyn Module>>,
    ) -> u64 {
        println!("[BC] finding frequency for {} | {}", self.name(), state);
        1 * self.pulse_input_frequency(state, modules)
    }

    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        let mut out_pulses = Vec::new();
        for output in &self.outputs {
            out_pulses.push(Pulse {
                state: pulse.state,
                from: self.name().to_string(),
                to: output.clone(),
            })
        }
        out_pulses
    }
}

struct FlipFlop {
    name: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    is_on: bool,
}

impl FlipFlop {
    fn parse(line: &str) -> Result<Box<dyn Module>, String> {
        let (declaration, outputs) = line
            .split_once(" -> ")
            .ok_or(format!("invalid line: {}", line))?;

        let name = declaration[1..].to_string();
        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();

        Ok(Box::new(FlipFlop {
            name,
            inputs: Vec::new(),
            outputs,
            is_on: false,
        }))
    }

    fn pulse_input_frequency(
        &self,
        state: PulseState,
        modules: &HashMap<String, Box<dyn Module>>,
    ) -> u64 {
        let mut min_input_freq = u64::MAX;
        for input in &self.inputs {
            let input_freq = modules.get(input).unwrap().pulse_frequency(state, modules);
            if input_freq < min_input_freq {
                min_input_freq = input_freq;
            }
        }
        min_input_freq
    }
}

impl Module for FlipFlop {
    fn name(&self) -> &str {
        &self.name
    }

    fn inputs(&self) -> Vec<&String> {
        self.inputs.iter().collect_vec()
    }

    fn outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn set_inputs(&mut self, inputs: &[String]) {
        self.inputs = inputs.to_vec();
    }

    fn pulse_frequency(
        &self,
        state: PulseState,
        modules: &HashMap<String, Box<dyn Module>>,
    ) -> u64 {
        println!("[FF] finding frequency for {} | {}", self.name(), state);
        let self_frequency = if state == PulseState::High { 1 } else { 2 };
        self_frequency * self.pulse_input_frequency(state, modules)
    }

    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        match pulse.state {
            PulseState::High => {
                vec![] // ignored
            }
            PulseState::Low => {
                self.is_on = !self.is_on;

                let out_pulse_state = if self.is_on {
                    PulseState::High
                } else {
                    PulseState::Low
                };

                let mut out_pulses = Vec::new();
                for output in &self.outputs {
                    out_pulses.push(Pulse {
                        state: out_pulse_state,
                        from: self.name().to_string(),
                        to: output.clone(),
                    })
                }
                out_pulses
            }
        }
    }
}

struct Conjunction {
    name: String,
    memory: HashMap<String, PulseState>,
    outputs: Vec<String>,
}

impl Conjunction {
    fn parse(line: &str) -> Result<Box<dyn Module>, String> {
        let (declaration, outputs) = line
            .split_once(" -> ")
            .ok_or(format!("invalid line: {}", line))?;

        let name = declaration[1..].to_string();
        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();

        Ok(Box::new(Conjunction {
            name,
            memory: HashMap::new(),
            outputs,
        }))
    }
}

impl Module for Conjunction {
    fn name(&self) -> &str {
        &self.name
    }

    fn inputs(&self) -> Vec<&String> {
        self.memory.keys().collect_vec()
    }

    fn outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn set_inputs(&mut self, inputs: &[String]) {
        for input in inputs {
            self.memory.insert(input.clone(), PulseState::Low);
        }
    }

    fn pulse_frequency(
        &self,
        state: PulseState,
        modules: &HashMap<String, Box<dyn Module>>,
    ) -> u64 {
        println!("[CJ] finding frequency for {} | {}", self.name(), state);
        let mut input_freqs = Vec::new();
        for input in self.memory.keys() {
            input_freqs.push(modules.get(input).unwrap().pulse_frequency(state, modules));
        }
        input_freqs
            .into_iter()
            .reduce(|acc, freq| acc.lcm(&freq))
            .unwrap_or(1)
    }

    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        // first, update memory
        *self.memory.get_mut(&pulse.from).unwrap() = pulse.state;

        let out_pulse_state = if self.memory.values().all(|&v| v == PulseState::High) {
            PulseState::Low
        } else {
            PulseState::High
        };

        let mut out_pulses = Vec::new();
        for output in &self.outputs {
            out_pulses.push(Pulse {
                state: out_pulse_state,
                from: self.name().to_string(),
                to: output.clone(),
            })
        }
        out_pulses
    }
}
