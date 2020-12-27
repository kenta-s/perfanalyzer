use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::env;
mod valid_line_extractor;
use valid_line_extractor::extract_usable_lines;
mod text_extractor;
use text_extractor::{
    extract_string_from_row,
    extract_duration_from_row,
    extract_page_from_row
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct PerfInfo {
    page: String,
    controller: String,
    action: String,
    duration: f32,
    view: f32,
    db: f32,
    count: u32
}

#[derive(Serialize, Deserialize)]
struct SlowPage {
    page: String,
    average_duration: f32,
    count: u32
}

impl PartialEq for SlowPage {
    fn eq(&self, other: &Self) -> bool {
        self.average_duration == other.average_duration
    }
}

impl PartialOrd for SlowPage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.average_duration.partial_cmp(&self.average_duration)
    }
}

#[derive(Serialize, Deserialize)]
struct PageInformation {
    information: HashMap<String, PerfInfo>,
    slow_pages: Vec<SlowPage>
}

fn row_to_perf_info(row: &str) -> Result<PerfInfo, &str> {
    let controller = extract_string_from_row(row, Regex::new(r"controller=(\S+)").unwrap())?;
    let action = extract_string_from_row(row, Regex::new(r"action=(\S+)").unwrap())?;
    let duration = extract_duration_from_row(row, Regex::new(r"duration=(\S+)").unwrap())?;
    let view = extract_duration_from_row(row, Regex::new(r"view=(\S+)").unwrap())?;
    let db = extract_duration_from_row(row, Regex::new(r"db=(\S+)").unwrap())?;

    let page = extract_page_from_row(&row)?;

    let perf_info = PerfInfo {
        page: page,
        controller: controller,
        action: action,
        duration: duration,
        view: view,
        db: db,
        count: 1
    };

    return Ok(perf_info);
}

fn merge_perf_info(perf_info1: PerfInfo, perf_info2: &PerfInfo) -> PerfInfo {
    return PerfInfo {
        page: perf_info1.page,
        controller: perf_info1.controller,
        action: perf_info1.action,
        duration: (perf_info1.duration + perf_info2.duration) / 2.0,
        view: (perf_info1.view + perf_info2.view) / 2.0,
        db: (perf_info1.db + perf_info2.db) / 2.0,
        count: perf_info1.count + perf_info2.count
    };
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 { panic!("filename must be given!") }

    // if args.len() == 1 {
    //    let filename = String::from(&args[0]);
    //    extract_usable_lines(filename).unwrap();
    // }

    let filename = &args[0];

    let mut perf_map: HashMap<String, PerfInfo> = HashMap::new();
    let mut found_pages = Vec::new();
    let mut slow_pages = Vec::new();

    for result in BufReader::new(File::open(filename)?).lines() {
        let row = result?;
        let perf_result = row_to_perf_info(&row);

        let perf_info = match perf_result {
            Err(_) => continue,
            Ok(perf) => perf,
        };

        found_pages.push(perf_info.page.clone());

        if let Some(perf) = perf_map.get_mut(&perf_info.page) {
            *perf = merge_perf_info(perf_info.clone(), perf);
        } else {
            perf_map.insert(perf_info.page.clone(), perf_info);
        }
    }

    found_pages.sort();
    found_pages.dedup();

    for page in found_pages {
        match perf_map.get(&page) {
            None => continue,
            Some(perf) => {
                let slow_page = SlowPage {
                    page: page,
                    average_duration: perf.duration,
                    count: perf.count
                };

                slow_pages.push(slow_page);
            }
        };
    };

    slow_pages.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let page_information = PageInformation {
        information: perf_map,
        slow_pages: slow_pages,
    };

    let serialized = serde_json::to_string(&page_information).unwrap();
    println!("{}", serialized);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_to_perf_info_returns_perf_info_when_valid_row_is_given() {
        let row = "method=GET path=/foo format=html controller=FooController action=index status=200 duration=1915.45 view=1841.20 db=7.93";
        let value = row_to_perf_info(&row).unwrap();

        assert_eq!(String::from("GET /foo"), value.page);
        assert_eq!(String::from("FooController"), value.controller);
        assert_eq!(String::from("index"), value.action);
        assert_eq!(1915.45, value.duration);
        assert_eq!(1841.20, value.view);
        assert_eq!(7.93, value.db);
        assert_eq!(1, value.count);
    }

    #[test]
    fn row_to_perf_info_returns_err_when_invalid_row_is_given() {
        // "action" is missing in this row
        let row = "method=GET path=/foo format=html controller=FooController status=200 duration=1915.45 view=1841.20 db=7.93";
        let value = row_to_perf_info(&row);

        assert_eq!(Err("failed"), value);
    }

    #[test]
    fn merge_perf_info_returns_merged_perf_info() {
        let perf_info1 = PerfInfo {
            page: String::from("GET /foo"),
            controller: String::from("FooController"),
            action: String::from("index"),
            duration: 1000.0,
            view: 100.0,
            db: 10.0,
            count: 40
        };

        let perf_info2 = PerfInfo {
            page: String::from("GET /foo"),
            controller: String::from("FooController"),
            action: String::from("index"),
            duration: 2000.0,
            view: 200.0,
            db: 20.0,
            count: 2
        };

        let value = merge_perf_info(perf_info1, &perf_info2);
        assert_eq!(String::from("GET /foo"), value.page);
        assert_eq!(String::from("FooController"), value.controller);
        assert_eq!(String::from("index"), value.action);
        assert_eq!(1500.0, value.duration);
        assert_eq!(150.0, value.view);
        assert_eq!(15.0, value.db);
        assert_eq!(42, value.count);
    }
}
