use std::collections::HashMap;

pub fn mean(arr: &Vec<i32>) -> f32 {
    let mut sum = 0.;
    for i in arr {
        sum += *i as f32;
    }
    sum / arr.len() as f32
}

pub fn median(arr: &Vec<i32>) -> i32 {
    let mut copy = arr.clone();
    copy.sort();
    copy[copy.len() / 2]
}

pub fn mode(arr: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for i in arr {
        let total = map.entry(i).or_insert(0);
        *total += 1;
    }
    // TODO: Assumes a mode exists, returning the first element of a unique array
    let key_with_max_value = map.iter().max_by_key(|entry| entry.1).unwrap();

    return **key_with_max_value.0
}