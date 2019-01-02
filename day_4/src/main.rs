extern crate regex;
extern crate chrono;

use regex::Regex;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Copy)]
enum Event {
    StartsShift, FallsAsleep, WakesUp
}

impl Event {
    fn get_regex(&self) -> Regex {
        // TODO should avoid recompiling regex every time
        match *self {
            Event::StartsShift => Regex::new(r"\[(.+)\] Guard #(\d+) begins shift").unwrap(),
            Event::FallsAsleep => Regex::new(r"\[(.+)\] falls asleep").unwrap(),
            Event::WakesUp     => Regex::new(r"\[(.+)\] wakes up").unwrap(),
        }
    }
}

impl Clone for Event {
    fn clone(&self) -> Event { *self }
}

#[derive(Copy)]
struct StampedEvent {
    stamp:      NaiveDateTime,
    event:      Event,
    guard_id:   u32,
}

impl Clone for StampedEvent {
    fn clone(&self) -> StampedEvent { *self }
}

fn new_event(input_line: String) -> StampedEvent {
    let event: Event;
    if input_line.contains("begins shift") {
        event = Event::StartsShift;
    } else if input_line.contains("wakes up") {
        event = Event::WakesUp;
    } else if input_line.contains("falls asleep") {
        event = Event::FallsAsleep;
    } else {
        panic!("Dont know how to handle line {}", input_line);
    }
    let groups = event.get_regex().captures(&input_line).unwrap();
    let stamp = NaiveDateTime::parse_from_str(&groups[1], "%Y-%m-%d %H:%M").unwrap();
    let mut guard_id = 0;
    if event == Event::StartsShift {
        guard_id = groups[2].parse().unwrap()
    }
    StampedEvent {
        stamp: stamp,
        event: event,
        guard_id: guard_id,
    }
}

fn main() {
    // Part 1
    let puzzle = include_str!("input.txt");
    let mut events = Vec::new();
    for line in puzzle.lines() {
        events.push(new_event(line.to_string()));
    }
    events.sort_by(|a, b| a.stamp.cmp(&b.stamp));
    let mut previous_event = events[0];
    let mut current_guard_id = 0;
    let mut minutes_asleep = HashMap::new();
    for e in events {
        if e.event == Event::StartsShift {
            current_guard_id = e.guard_id;
            minutes_asleep.entry(e.guard_id).or_insert(vec![0; 60]);
        } else {
            match e.event {
                Event::WakesUp => {
                    for m in previous_event.stamp.minute()..e.stamp.minute() {
                        minutes_asleep.get_mut(&current_guard_id).unwrap()[m as usize] += 1;
                    }
                },
                _ => {}
            }
        }
        previous_event = e;
    }
    let mut max_sum: u64 = 0;
    let mut max_key = minutes_asleep.keys().next().unwrap();
    for (k, v) in &minutes_asleep {
        let sum = v.iter().sum();
        if sum > max_sum {
            max_key = k;
            max_sum = sum;
        }
    }
    let mut max_minute = minutes_asleep[&max_key].iter().enumerate().max_by(|a, b| a.1.cmp(b.1));
    println!("guard: {:?}", max_key);
    println!("minute: {:?}", max_minute);
    // Part 2
    let part_2 = minutes_asleep.iter().max_by(|a, b| {
        a.1.iter().max().cmp(&b.1.iter().max())
    });
    max_minute =  part_2.unwrap().1.iter().enumerate().max_by(|a, b| a.1.cmp(b.1));
    println!("{:?}", *part_2.unwrap().0 as usize * max_minute.unwrap().0);
}
