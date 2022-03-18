use crate::parser::Node;

use std::collections::HashMap;
use std::str::FromStr;
use std::fs;

pub fn get_text_node<'a>(
    tag_name: &str,
    nodes: &'a [Node],
    id: usize,
) -> Result<&'a String, String> {
    match nodes.get(id) {
        Some(&Node::Text(ref s)) => Ok(s),
        _ => Err(format!("({}): Missing text argument", tag_name)),
    }
}

pub fn get_num_node<'a, T: FromStr>(
    tag_name: &str,
    nodes: &'a [Node],
    id: usize,
) -> Result<T, String> {
    get_text_node(tag_name, nodes, id)?
        .parse::<T>()
        .map_err(|_| "Invalid number".to_string())
}

pub fn get_node<'a>(tag_name: &str, nodes: &'a [Node], id: usize) -> Result<&'a Node, String> {
    nodes
        .get(id)
        .ok_or(format!("({}): Missing node argument", tag_name))
}

lazy_static! {
    pub static ref HWMON_NAME_TO_PATH: HashMap<String, String> = {
        let mut hwmon_table = HashMap::new();
        let hwmons = fs::read_dir("/sys/class/hwmon/").expect("This program expects to be able to list the contents of /sys/class/hwmon/.");
        for mon_result in hwmons {
            let mon = mon_result.expect("Entry in /sys/class/hwmon/ was an error.");
            let mon_path = mon.path();
            let name_path = mon_path.join("name");
            let name_res = fs::read_to_string(name_path);
            if name_res.is_ok() {
                let name_key: String = name_res.unwrap().trim().to_string();
                // This program won't work on Windows anyway.
                let mon_path_string: String = mon_path.to_string_lossy().into_owned();
                info!("Adding hwmon label: '{}', '{}'", name_key, mon_path_string);
                hwmon_table.insert(name_key, mon_path_string);
            } else {
                warn!("Found hwmon where 'name' file was not readable: {}", mon_path.to_string_lossy());
            }
        }
        hwmon_table
    };
}
