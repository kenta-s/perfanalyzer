use std::fs::File;
use std::io::{Write, BufRead, BufReader, BufWriter};
use regex::Regex;
use std::collections::HashMap;

struct PerfInfo {
    page: String,
    controller: String,
    action: String,
    duration: f32,
    view: f32,
    db: f32,
    count: u32
}

struct SlowPage {
    page: String,
    average_duration: f32,
    count: u32
}

fn extract_string_from_row(row: &str, regex: Regex) -> String {
    let texts = regex.captures(row).unwrap();

    return String::from(&texts[0]);
}

fn page_from_row(row: &str) -> String {
    let path_regex = Regex::new(r"path=(\S+)").unwrap();
    let method_regex = Regex::new(r"method=(\S+)").unwrap();

    let path = extract_string_from_row(row, path_regex);
    let method = extract_string_from_row(row, method_regex);

    return format!("{} {}", method, path);
}

fn row_to_perf_info(row: &str) -> std::io::Result<PerfInfo> {
    let controller = extract_string_from_row(row, Regex::new(r"controller=(\S+)").unwrap());
    let action = extract_string_from_row(row, Regex::new(r"action=(\S+)").unwrap());

    let duration_regex = Regex::new(r"duration=(\S+)").unwrap();
    let duration_cap = duration_regex.captures(row).unwrap();

    let duration_cap_split: Vec<&str> = duration_cap[0].split("=").collect();
    let duration = duration_cap_split[1].parse::<f32>().unwrap();

    let view_regex = Regex::new(r"view=(\S+)").unwrap();
    let view_cap = view_regex.captures(row).unwrap();
    let view_cap_split: Vec<&str> = view_cap[0].split("=").collect();
    let view = view_cap_split[1].parse::<f32>().unwrap();

    let db_regex = Regex::new(r"db=(\S+)").unwrap();
    let db_cap = db_regex.captures(row).unwrap();
    let db_cap_split: Vec<&str> = db_cap[0].split("=").collect();
    let db = db_cap_split[1].parse::<f32>().unwrap();

    // let page = format!("{} {}", &method_cap[0], &path_cap[0]);
    let page = page_from_row(&row);

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

fn extract_usable_lines() -> std::io::Result<()> {
    let f = File::create("./ready.log")?;
    let mut stream = BufWriter::new(f);

    for result in BufReader::new(File::open("./development.log")?).lines() {
        let row = result?;

        let path_regex = Regex::new(r"path=(\S+)").unwrap();
        if !path_regex.is_match(&row) { continue };

        stream.write(&row.as_bytes());
        stream.write(b"\n")?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // extract_usable_lines();

    let mut perf_map: HashMap<String, PerfInfo> = HashMap::new();
    let mut found_pages = Vec::new();
    let mut slow_pages = Vec::new();

    // TODO: get filename from ARGV
    for result in BufReader::new(File::open("./ready.log")?).lines() {
        let row = result?;

        let path_regex = Regex::new(r"path=(\S+)").unwrap();
        if !path_regex.is_match(&row) { continue };

        let page = page_from_row(&row);
        println!("{}", page);

        let perf_result = row_to_perf_info(&row);

        let perf_info = match perf_result {
          Err(_) => continue,
          Ok(perf) => perf,
        };

        let page = perf_info.page.clone();
        let cloned_page = page.clone();

        found_pages.push(cloned_page);

        let merged_perf_info = match perf_map.get(&page) {
          None => perf_info,
          Some(perf) => merge_perf_info(perf_info, perf)
        };

        perf_map.insert(page, merged_perf_info);
    }

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

    // let f = File::create("./out.log")?;

    // let mut stream = BufWriter::new(f);
    // for page in slow_pages {
    //     stream.write(page.path.as_bytes());
    //     stream.write(b"\n")?;
    // };

    for page in slow_pages {
        println!("slowest page: {}", &page.page);
    };
    // println!("slowest page: {}", &slow_pages[0].path);

    Ok(())
}
