use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::collections::{VecDeque,HashSet};

const TARGET_COLOR: &str = "shiny gold";

fn main() -> std::io::Result<()> {
    let arg: Option<String> = args().skip(1).next();
    if arg.is_none() {
        println!("Missing arg");
        return Ok(());
    }
    let file_path = arg.unwrap();
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let rules: Vec<(&str, Vec<(u32, &str)>)> = buffer.split("\n").filter(|l| l.len() > 0).map(|rule| {
        let mut context_and_subject = rule.split(" contain ");
        let context = context_and_subject
            .next()
            .map(|c| c.strip_suffix(" bags"))
            .flatten()
            .unwrap();
        let subject: Vec<(u32, &str)> = context_and_subject
            .next()
            .map(|s| s.strip_suffix("."))
            .flatten()
            .unwrap()
            .split(", ")
            .map(|r| r.strip_suffix(" bag").unwrap_or(r))
            .map(|r| r.strip_suffix(" bags").unwrap_or(r))
            .filter_map(|r| {
                if r == "no other" {
                    return None;
                }
                let mut num_and_subject = r.splitn(2, " ");
                let num = num_and_subject
                    .next()
                    .map(|n| n.parse::<u32>().ok())
                    .flatten()
                    .unwrap();
                let subject = num_and_subject
                    .next()
                    .unwrap();
                Some((num, subject))
            })
            .collect();
        println!("Line: {} -> {:?}", context, subject);
        (context, subject)
    }).collect();

    let mut current_targets: VecDeque<&str> = VecDeque::new();
    let mut acceptable_bags: HashSet<&str> = HashSet::new();
    current_targets.push_back(TARGET_COLOR);
    while let Some(current_target) = current_targets.pop_front() {
        let mut found: Option<&str> = None;
        for (bag, contains) in rules.iter() {
            let can_contain_target = contains
                .iter()
                .find(|(_num, contains_bag)| contains_bag == &current_target)
                .is_some();
            if can_contain_target {
                println!("{} can contain {}", bag, current_target);
                current_targets.push_back(bag);
                acceptable_bags.insert(bag);
            }
        }
    }

    println!("Can contain: {}", acceptable_bags.len());

    Ok(())
}
