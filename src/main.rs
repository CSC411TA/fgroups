
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::str;

fn main() {
    let mut fgroups: HashMap<String, Vec<String>> = HashMap::new();
    for line in io::stdin().lock().lines() {
        let l = line.unwrap();
        let parts: Vec<&str> = l.splitn(2, |c| c == ' ' || c == '\t').collect();
        let fp = parts[0].to_string();
        let name = parts[1].trim_start_matches(|c: char| c == ' ' || c == '\t');
        match fgroups.get_mut(&fp) {
            Some(names) => {
                names.push((&name).to_string());
            }
            None => {
                fgroups.insert((&fp).to_string(), vec![(&name).to_string()]);
            }
        }
    }
    let mut sep: String = "".to_string();
    let fgroups: Vec<&Vec<String>> = fgroups.values().filter(|&names| names.len() > 1).collect();
    fgroups.iter().for_each(|fg| print_group(& mut sep, fg));
}

fn print_group(sep: & mut String, fgroup: &[String]) {
	println!("{}{}", sep, fgroup.join("\n"));
    *sep = "\n".to_string();
}