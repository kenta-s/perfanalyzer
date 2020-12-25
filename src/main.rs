use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

struct PerfInfo {
    method: String,
    path: String,
    controller: String,
    action: String,
    duration: f32,
    view: f32,
    db: f32,
    count: u32
}

struct SlowPage {
    path: String,
    average_duration: f32,
    count: u32
}

fn row_to_perf_info(row: &str) -> std::io::Result<PerfInfo> {
    let path_regex = Regex::new(r"path=(\S+)").unwrap();
    let path_cap = path_regex.captures(row).unwrap();

    let method_regex = Regex::new(r"method=(\S+)").unwrap();
    let method_cap = method_regex.captures(row).unwrap();

    let controller_regex = Regex::new(r"controller=(\S+)").unwrap();
    let controller_cap = controller_regex.captures(row).unwrap();

    let action_regex = Regex::new(r"action=(\S+)").unwrap();
    let action_cap = action_regex.captures(row).unwrap();

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

    let perf_info = PerfInfo {
        method: String::from(&method_cap[0]),
        path: String::from(&path_cap[0]),
        controller: String::from(&controller_cap[0]),
        action: String::from(&action_cap[0]),
        duration: duration,
        view: view,
        db: db,
        count: 1
    };

    return Ok(perf_info);
}

fn main() -> std::io::Result<()> {
    let mut perf_map: HashMap<&str, PerfInfo> = HashMap::new();
    // let mut existing_keys = Vec::new();

    // TODO: get filename from ARGV
    for result in BufReader::new(File::open("./development.log")?).lines() {
        let row = result?;

        let path_regex = Regex::new(r"path=(\S+)").unwrap();

        if path_regex.is_match(&row) {
          let perf_result = row_to_perf_info(&row);
          // println!("{}", &row);

          match perf_result {
            Err(why) => println!("{}", why),
            Ok(perf) => println!("{}", perf.method),
          }

          // let perf_info = match perf_result {
          //   Err(why) => None,
          //   Some(&perf) => Ok(perf,
          // };
          // println!("{} {}", perf_info.method, perf_info.path);

          // let key = format!("{} {}", perf_info.method, perf_info.path);
          // let value = perf_map.get(key.as_str());
          // println!("{}", value.path)
          // // existing_keys.push(key.as_str());
          // match value {
          //    None => perf_map.insert(key.as_str(), perf_info),
          //    _ => perf_map.insert(key.as_str(), perf_info) // TODO: insert perf_info with longer duration
          // };

        };
    }

    // println!("{}", bar.method);
    // println!("{}", bar.path);
    // println!("{}", bar.duration);
    // println!("{}", bar.view);
    // println!("{}", bar.db);

    // for x in &existing_keys {
    //     println!("{}", x);
    // }
    Ok(())
}
