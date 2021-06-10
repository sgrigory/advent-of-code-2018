extern crate regex;

use std::fs;
use std::collections::HashMap;
use regex::Regex;
use chrono::NaiveDateTime;
use chrono::Timelike;



struct Entry {
	time: NaiveDateTime,
	text: String,
}

fn get_entry(s: &str) -> Entry {
	// input looks like: [1518-11-01 00:00] Guard #10 begins shift
	let entry_re = Regex::new(r"\[(.+)\] (.+)$").unwrap();

	let parsed = entry_re.captures(s).unwrap();
	let time_unparsed: &str = parsed.get(1).map_or("", |m| m.as_str());//.parse().unwrap(); // 1518-04-30 00:00
	let time = NaiveDateTime::parse_from_str(&time_unparsed, "%Y-%m-%d %H:%M").unwrap();

	let type_unparsed = parsed.get(2).map_or("", |m| m.as_str());
	
	
	Entry {time: time, text: type_unparsed.to_string()}
}

fn write_interval(timetable: &mut HashMap<i32, [i32;60]>, fall_asleep: NaiveDateTime, wake_up: NaiveDateTime, guard: i32) -> () {
	let minutes_start = fall_asleep.time().minute() as usize;
	let minutes_end = wake_up.time().minute() as usize;
	for i in minutes_start..minutes_end {
		if timetable.contains_key(&guard) {
			(*timetable.get_mut(&guard).unwrap())[i] += 1
		}
		else {
			timetable.insert(guard, [0;60]);
		}
	};
}

fn main() {

	let file_content = fs::read_to_string("inputs/day4_input.txt").expect("error");
	let mut entries = file_content.trim().split("\n").map(get_entry).collect::<Vec<Entry>>();
	// Sort entries chronologically
	entries.sort_by(|a, b | a.time.cmp(&b.time));
	
	// Current guard
	let mut last_guard = 0;
	// Time of the last registered "fall asleep" event
	let mut last_fall_asleep = NaiveDateTime::parse_from_str("2021-01-01 21:00", "%Y-%m-%d %H:%M").unwrap();
	// Regex to parse new guard entries
	let new_guard_re = Regex::new(r"Guard #(\d+) begins shift$").unwrap();

	// Keys are guards, indices of the array are minutes, values of the array are amounts of sleep
	let mut timetable: HashMap<i32, [i32;60]> = HashMap::new();

	// Go over all entries, fill in the timetable
	for entry in entries {
		match entry.text.as_str() {
			"wakes up" => write_interval(&mut timetable, last_fall_asleep, entry.time, last_guard),
			"falls asleep" => last_fall_asleep = entry.time,
			_ => last_guard = { 
				     			new_guard_re.captures(&entry.text).unwrap().get(1).map_or("", |m| m.as_str()).parse().unwrap()
							},
		}
	};

	let sums = timetable.iter().map(|x| (*x.0, x.1.iter().sum::<i32>()));

	// Find guard with max total sleep
	let mut max_sleep = 0;
	let mut chosen_guard = 0;
	for (k, v) in sums {
		if v > max_sleep {
			max_sleep = v;
			chosen_guard = k;
		}
	}
	println!("chosen_guard = {}", chosen_guard);

	// Find minute on which the chosen guard had the most sleep
	let minute_max_sleep = timetable[&chosen_guard].iter().enumerate().max_by_key(|x| x.1).unwrap().0;

	println!("minute_max_sleep = {}", minute_max_sleep);

	println!("answer to part 1 = {}", minute_max_sleep as i32 * chosen_guard);

	// Find minute with max total sleep per guard
	let max_by_guard = timetable.iter().map(|x| (x.0, x.1.iter().enumerate().max_by_key(|x| x.1).unwrap()));
	// Find the guard and the minute with max total sleep overall
	let max_overall = max_by_guard.max_by_key(|x| x.1.1).unwrap();

	println!("answer to part 2 = {}", max_overall.0 * max_overall.1.0 as i32);

}