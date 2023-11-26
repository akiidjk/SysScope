//* SIMPLE FUNCTION *//
use std::collections::HashMap;
use std::fs::File;
use std::{cmp, fs, io};
use std::io::Write;
use std::path::Path;
use colored::Colorize;
use crate::Results;


pub fn get_hashmap_string(hashmap: &HashMap<&str, Results>) -> String {
    let mut result = String::new();
    for (key, value) in hashmap {
        result.push_str(&format!("{}: ", capitalize_first_letter(key.replace("_"," ")).green().bold()));
        match value {
            Results::Max(val) =>{
                if key.contains("percent"){
                    result.push_str(&format!("{} % \x1B[K \n", val.to_string().cyan()));
                }
                else if key.contains("ram") {
                    result.push_str(&format!("{} MB \x1B[K \n", val.to_string().cyan()));
                }
                else {
                    result.push_str(&format!("{} MB \x1B[K \n", val.to_string().cyan()));
                }
            }
            Results::Average(vals) => {
                if key.contains("percent"){
                    result.push_str(&format!("{} % \x1B[K \n", average(vals).to_string().cyan()));
                }
                else if key.contains("ram") {
                    result.push_str(&format!("{} MB \x1B[K \n", average(vals).to_string().cyan()));
                }
                else {
                    result.push_str(&format!("{} MB \x1B[K \n", average(vals).to_string().cyan()));
                }
            }
        }
    }
    result
}

pub fn clear_last_lines(n: usize) {
    for _ in 0..n {
        print!("\x1B[1A");
        print!("\x1B[2K");
    }
}
pub fn average(vals: &Vec<f32>) -> f32 {
    if vals.is_empty() {
        return 0.0;
    }
    let sum: f32 = vals.iter().sum();
    sum / vals.len() as f32
}
pub fn write_hashmap(hashmap: &HashMap<&str, Results>) -> io::Result<()> {
    let mut file = File::create("out.tmp")?;
    file.write_all(get_hashmap_string(hashmap).as_bytes())?;
    Ok(())
}
pub fn remove_file(){
    let path = "out.tmp";

    match fs::remove_file(path) {
        Ok(()) => (),
        Err(e) => println!("Error while removing temporary file : {}", e),
    }
}
pub fn read_hashmap() -> io::Result<String> {
    let path = Path::new("out.tmp");
    let content = fs::read_to_string(path);
    content
}
pub fn capitalize_first_letter(s: String) -> String {
    s.char_indices()
        .next()
        .map(|(i, c)| c.to_uppercase().collect::<String>() + &s[i + c.len_utf8()..])
        .unwrap_or_else(|| s.to_owned())
}

//true = max false = average
pub fn mod_value(new_value:f32,hashmap: &mut HashMap<&str, Results>,type_:bool,key:&str){
    if type_ {
        if let Some(Results::Max(ref mut max_val)) = hashmap.get_mut(key) {
            let value = new_value;
            if let Some(cmp::Ordering::Greater) = value.partial_cmp(max_val) {
                *max_val = value;
            }
        }
    }
    else{
        if let Some(Results::Average(vec)) = hashmap.get_mut(key) {
            vec.push(new_value);
        }
    }
}