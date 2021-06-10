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

	println!("{}", s);	
	let parsed = entry_re.captures(s).unwrap();
	let time_unparsed: &str = parsed.get(1).map_or("", |m| m.as_str());//.parse().unwrap(); // 1518-04-30 00:00
	let time = NaiveDateTime::parse_from_str(&time_unparsed, "%Y-%m-%d %H:%M").unwrap();

	let type_unparsed = parsed.get(2).map_or("", |m| m.as_str());
	
	
	Entry {time: time, text: type_unparsed.to_string()}
}

fn write_interval(timetable: &mut HashMap<i32, [i32;60]>, fall_asleep: NaiveDateTime, wake_up: NaiveDateTime, guard: i32) -> () {
	let minutes_start = fall_asleep.time().minute() as usize;
	let minutes_end = wake_up.time().minute() as usize;
	println!("{}", guard);
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
	entries.sort_by(|a, b | a.time.cmp(&b.time));
	
	let mut last_guard = 0;
	let mut last_fall_asleep = NaiveDateTime::parse_from_str("2021-01-01 21:00", "%Y-%m-%d %H:%M").unwrap();
	let new_guard_re = Regex::new(r"Guard #(\d+) begins shift$").unwrap();

	let mut timetable: HashMap<i32, [i32;60]> = HashMap::new();

	for entry in entries {
		println!("new entry: {}", entry.text);
		match entry.text.as_str() {
			"wakes up" => write_interval(&mut timetable, last_fall_asleep, entry.time, last_guard),
			"falls asleep" => last_fall_asleep = entry.time,
			_ => last_guard = { println!("last guard entry: {}", entry.text);
				     			new_guard_re.captures(&entry.text).unwrap().get(1).map_or("", |m| m.as_str()).parse().unwrap()
							},
		}
	};

	let sums = timetable.iter().map(|x| (*x.0, x.1.iter().sum::<i32>()));

	let mut max_sleep = 0;
	let mut chosen_guard = 0;
	for (k, v) in sums {
		if v > max_sleep {
			max_sleep = v;
			chosen_guard = k;
		}
	}
	println!("chosen_guard = {}", chosen_guard);

	let minute_max_sleep = timetable[&chosen_guard].iter().enumerate().max_by_key(|x| x.1).unwrap().0;

	println!("minute_max_sleep = {}", minute_max_sleep);

	println!("answer to part 1 = {}", minute_max_sleep as i32 * chosen_guard)

}