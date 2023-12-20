use itertools::Itertools;
use num::integer::lcm;
use sscanf::sscanf;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

struct Module {
    r#type: ModuleType,
    id: String,
    destinations: Vec<String>,
    state: Option<bool>,
    memory: Option<HashMap<String, bool>>,
}
fn process_pulse(module: &mut Module, source_id: String, pulse: bool) -> Option<bool> {
    match module.r#type {
        ModuleType::Broadcast => Some(pulse),
        ModuleType::FlipFlop => {
            if pulse {
                None
            } else {
                module.state = Some(!module.state.unwrap());
                module.state
            }
        }
        ModuleType::Conjunction => {
            *module.memory.as_mut().unwrap().get_mut(&source_id).unwrap() = pulse;
            Some(!module.memory.as_ref().unwrap().iter().all(|(_, v)| *v))
        }
    }
}
fn parse_module(raw: &str) -> Module {
    let (mod_type, mod_id, mut mod_dest) =
        sscanf!(raw, "{}{} -> {}", char, String, String).unwrap();
    mod_dest = mod_dest.replace(',', "");
    let dest_ids: Vec<String> = mod_dest.split_whitespace().map_into::<String>().collect();
    match mod_type {
        'b' => Module {
            r#type: ModuleType::Broadcast,
            id: "broadcast".to_string(),
            destinations: dest_ids,
            state: None,
            memory: None,
        },
        '%' => Module {
            r#type: ModuleType::FlipFlop,
            id: mod_id,
            destinations: dest_ids,
            state: Some(false),
            memory: None,
        },
        '&' => Module {
            r#type: ModuleType::Conjunction,
            id: mod_id,
            destinations: dest_ids,
            state: None,
            memory: Some(HashMap::<String, bool>::new()),
        },
        _ => unreachable!(),
    }
}

fn main() {
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut conjunctions_ids: Vec<String> = Vec::new();
    for line in read_to_string("input/20").unwrap().lines() {
        let module = parse_module(line);
        if module.r#type == ModuleType::Conjunction {
            conjunctions_ids.push(module.id.clone());
        }
        modules.insert(module.id.clone(), module);
    }
    let mut conjunctions_sources: Vec<Vec<String>> = vec![Vec::new(); conjunctions_ids.len()];
    for (id_source, mod_source) in modules.iter() {
        for (i, id_dest) in conjunctions_ids.iter().enumerate() {
            if mod_source.destinations.contains(&id_dest) {
                conjunctions_sources[i].push(id_source.clone());
            }
        }
    }
    for (id, modules_source) in conjunctions_ids.iter().zip(conjunctions_sources.iter()) {
        let module = modules.get_mut(id).unwrap();
        for s in modules_source {
            module.memory.as_mut().unwrap().insert(s.clone(), false);
        }
    }

    //rx is fed by who
    let before_rx = modules
        .iter()
        .find(|(_, v)| v.destinations.contains(&"rx".to_string()))
        .unwrap();

    // who is feeding the rx feeder

    let feeding_rx: Vec<String> = before_rx
        .1
        .memory
        .as_ref()
        .unwrap()
        .keys()
        .map(|k| k.clone())
        .collect();
    // So we just need them to go back to beaming high to get the period

    let mut periods: [u64; 4] = [0; 4];

    let mut nb_low = 0;
    let mut nb_high = 0;

    let mut nb_it = 0;
    loop {
        //START FROM BROADCAST
        let mut to_call: Vec<(bool, String, String)> =
            vec![(false, "broadcast".to_string(), "button".to_string())];
        while !to_call.is_empty() {
            let mut new_calls: Vec<(bool, String, String)> = Vec::new();
            for (pulse, id_dest, id_source) in to_call.iter() {
                if *pulse {
                    nb_high += 1;
                } else {
                    nb_low += 1;
                }
                match modules.get_mut(&id_dest.clone()) {
                    None => continue,
                    Some(module) => {
                        let new_pulse = process_pulse(module, id_source.clone(), *pulse);
                        match new_pulse {
                            None => continue,
                            Some(p) => {
                                for d in module.destinations.iter() {
                                    new_calls.push((p, d.clone(), id_dest.clone()));
                                }
                                if p && feeding_rx.contains(&id_dest.clone()) {
                                    periods[feeding_rx
                                        .iter()
                                        .position(|s| *s == id_dest.clone())
                                        .unwrap()] = nb_it + 1;
                                }
                            }
                        }
                    }
                }
            }
            to_call = new_calls;
        }
        nb_it += 1;
        if nb_it == 1000 {
            println!("Part 1: {}", nb_low * nb_high);
        }
        if periods.iter().all(|t| *t != 0) {
            break;
        }
    }
    println!("Part 2: {}", periods.iter().fold(1, |acc, t| lcm(acc, *t)));
}
