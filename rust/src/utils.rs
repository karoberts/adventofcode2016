use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::{HashMap,HashSet};
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

/// these are 2x the speed of the default
#[allow(dead_code)]
pub type HashSetFnv<K> = HashSet<K, BuildHasherDefault<FnvHasher>>;
#[allow(dead_code)]
pub type HashMapFnv<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

#[allow(unused_macros)]
#[macro_export]
macro_rules! fastset {
    () => { utils::HashSetFnv::default() }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! fastmap {
    () => { utils::HashMapFnv::default() }
}

/// allows for concise hashmap construction
/// like this: hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];
#[allow(unused_macros)]
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = fastmap!();
         $( map.insert($key, $val); )*
         map
    }}
}

/// read a file and return an iterator of lines
#[allow(dead_code)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// gets a string out of a regex capture
#[allow(dead_code)]
pub fn cap_to_string(cap: Option<regex::Match>) -> String
{
    cap.expect("capture doesn't exist").as_str().to_owned()
}

/// converts a regular expression capture to string and parses it, panics if there is a problem
#[allow(dead_code)]
pub fn cap_to<T: std::str::FromStr>(cap: Option<regex::Match>) -> T 
{
    match cap.map(|m| m.as_str().parse::<T>() ) {
        Some(s) => match s {
            Ok(ss) => ss,
            Err(_) => panic!("failed to parse {:?}", cap)
        },
        None => panic!("failed to get value {:?}", cap)
    }
}