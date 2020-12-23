use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::collections::{VecDeque,HashMap};

const TARGET_COLOR: &str = "shiny gold";

fn get_bags_within(rules: &Vec<(&str, Vec<(u32, &str)>)>, target: &str) -> u32 {
    let target_contains: &Vec<(u32, &str)> = rules
        .iter()
        .find(|(bag, _contains)| bag == &target)
        .map(|(_bag, contains)| contains)
        .unwrap();
    return target_contains
        .iter()
        .map(|(num, bag)| num * (1 + get_bags_within(rules, bag)))
        .sum();
}

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

    let bags_within_target = get_bags_within(&rules, TARGET_COLOR);

    println!("Target must contain: {}", bags_within_target);

    Ok(())
}
