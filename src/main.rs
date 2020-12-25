// use std::fs::File;
// use std::io::{self, BufRead, BufReader};
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
}

fn row_to_perf_info(row: &str) -> PerfInfo {
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

    return PerfInfo {
        method: String::from(&method_cap[0]),
        path: String::from(&path_cap[0]),
        controller: String::from(&controller_cap[0]),
        action: String::from(&action_cap[0]),
        duration: duration,
        view: view,
        db: db,
    };
}

fn main() -> std::io::Result<()> {
    let mut perf_map: HashMap<&str, PerfInfo> = HashMap::new();
    let mut existing_keys = Vec::new();
    let row = "[dc4889ee-3ff7-4fea-9c67-a184de666108] method=GET path=/sign_in format=html controller=Admin::SessionsController action=new status=200 duration=15867.41 view=15866.25 db=0.00 host=DESKTOP-SVH2LA9 remote_ip=::1 request_host=localhost params={}";
    let path_regex = Regex::new(r"path=(\S+)").unwrap();
    if path_regex.is_match(row) {
      let perf_info = row_to_perf_info(row);

      let key = format!("{} {}", perf_info.method, perf_info.path);
      existing_keys.push(key.as_str());
      let value = perf_map.get(key.as_str());
      match value {
         None => perf_map.insert(key.as_str(), perf_info),
         _ => perf_map.insert(key.as_str(), perf_info) // TODO: insert perf_info with longer duration
      };

      // println!("{}", bar.method);
      // println!("{}", bar.path);
      // println!("{}", bar.duration);
      // println!("{}", bar.view);
      // println!("{}", bar.db);
    };

    // for x in &existing_keys {
    //     println!("{}", x);
    // }
    Ok(())
}
