use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::str;

fn main() -> Result<(), io::Error> {
    // We will use a `HashMap` to map from fingerprints
    // to collections (`Vec`s) of names
    let mut fgroups: HashMap<String, Vec<String>> = HashMap::new();
    // Keep track of line number for nicer errors
    let mut linenum = 1;
    for line in io::stdin().lock().lines() {
        let l = line?;
        // Break each line, on first whitespace, into a
        // fingerprint and a name
        let parts: Vec<&str> = l.splitn(2, ws).collect();
        if parts.len() < 2 {
            eprintln!("Badly formed input (no name) on line {}", linenum);
            continue;
        }
        let fp = parts[0].to_string();
        let name = parts[1].trim_start_matches(ws);
        if fp.is_empty() || name.is_empty() {
            eprintln!(
                "Badly formed input (empty fingerprint or name) on line {}",
                linenum
            );
            continue;
        }
        match fgroups.get_mut(&fp) {
            None => {
                fgroups.insert((&fp).to_string(), vec![(&name).to_string()]);
            }
            Some(names) => {
                names.push((&name).to_string());
            }
        }
        linenum += 1;
    }
    // this `sep` indicates the separator to be printed
    // before each group; it is in outer scope because
    // it must maintain state.
    let mut sep: String = "".to_string();

    // It turns out it makes no difference in memory or time
    // if we create the vector strictly:

    // let fgroupss: Vec<&Vec<String>> = fgroups.values().filter(|&names| names.len() > 1).collect();
    // fgroupss.iter().for_each(|fg| print_group(&mut sep, fg));

    // or use lazy iteration:
    fgroups
        .values()
        .filter(|&names| names.len() > 1)
        .for_each(|fg| print_group(fg, &mut sep));
    Ok(())
}

fn ws(c: char) -> bool {
    c == ' ' || c == '\t'
}

// this relies on maintaining a state (sep) indicating
// whether a separator (presumably newline) should be
// printed before each group
fn print_group(fgroup: &[String], sep: &mut String) {
    println!("{}{}", sep, fgroup.join("\n"));
    *sep = "\n".to_string();
}
