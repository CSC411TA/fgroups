use std::io::BufRead;
use std::io;
use std::str;
use regex::Regex;
use std::collections::HashMap;

fn main() -> io::Result<()> {
	let mut fgroups:HashMap<String, Vec<String>>  = HashMap::new();
    for line in io::stdin().lock().lines() {
    	let pattern = Regex::new(r"\s+").unwrap();
    	let l = line.unwrap();
	    let parts: Vec<&str> = pattern.splitn(&l, 2).collect::<Vec<&str>>();
	    let fp = parts[0].to_string();
	    let name = parts[1].to_string();
	    match fgroups.get_mut(&fp) {
	    	Some(names) => {
	    		names.push((&name).to_string());
	    	},
	    	None => {
	    		fgroups.insert((&fp).to_string(), vec![(&name).to_string()]);
	    	}
	    }
    }
    let mut sep = "";
	for names in fgroups.values() {
		if names.len() > 1 {
			println!("{}{}", sep, names.join("\n"));
		}
		sep = "\n";
	}
    Ok(())
}
