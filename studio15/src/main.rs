use std::collections::{BTreeSet, BTreeMap, HashSet};

fn main() {
    // Initialize string
    // let thanks = "Thanks, Rosencrantz and gentle Guildenstern.";
    let shells = "She sells sea shells by the sea shore.";

    // Create vector
    let string_vec: Vec<String>           = shells.split_whitespace().map(str::to_string).collect();

    // Create BTreeSet
    let string_btreeset: BTreeSet<String> = shells.split_whitespace().map(str::to_string).collect();

    // Create HashSet
    let string_hashset: HashSet<String>   = shells.split_whitespace().map(str::to_string).collect();

    // Create BTreeMap
    let mut string_btreemap: BTreeMap<usize, String> = BTreeMap::new();

    // Insert items into the BTreeMap
    string_vec.iter().cloned().enumerate().for_each(|(index, string)| {string_btreemap.insert(index, string);});

    // Create second BTreeMap
    let mut string_btreemap_two: BTreeMap<String, usize> = BTreeMap::new();

    // Insert items into the second BTreeMap
    string_vec.into_iter().enumerate().for_each(|(index, string)| {string_btreemap_two.insert(string, index);});

    // Print collections
    println!("{:?}", string_vec);
    println!("{:?}", string_btreeset);
    println!("{:?}", string_hashset);
    println!("{:?}", string_btreemap);
    println!("{:?}", string_btreemap_two);
}