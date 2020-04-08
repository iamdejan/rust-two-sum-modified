use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

fn get_line() -> String {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read from STDIN");
    return line.trim().to_owned();
}

fn create_pair(i1: usize, i2: usize) -> (usize, usize) {
    if i1 < i2 {
        return (i1, i2);
    }
    return (i2, i1);
}

fn get_all_two_sums(numbers: Vec<i32>, target: i32) -> Result<Vec<(usize, usize)>, String> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in 0..numbers.len() {
        map.insert(numbers[i], i);
    }

    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut inserted_set: HashSet<(usize, usize)> = HashSet::new();
    for i1 in 0..numbers.len() {
        let complement: i32 = target - numbers[i1];
        if map.contains_key(&complement) {
            let i2: usize = *map.get(&complement).unwrap();
            if i1 == i2 {
                continue;
            }

            let pair = create_pair(i1, i2);
            if inserted_set.contains(&pair) {
                continue;
            }

            pairs.push(pair);
            inserted_set.insert(pair);
        }
    }

    if pairs.len() > 0 {
        return Ok(pairs);
    }

    return Err("no pairs found".to_owned());
}

fn main() {
    let numbers: Vec<i32> = get_line().split(',').map(|n| n.parse().unwrap()).collect();
    let target: i32 = get_line().parse().unwrap();
    match get_all_two_sums(numbers, target) {
        Ok(v) => {
            println!("{:?}", v);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
    io::stdout().flush().unwrap();
}
