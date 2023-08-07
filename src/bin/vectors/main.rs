use std::collections::HashMap;

fn main() {
    let mut v = vec![5, 1, 10, 5];

    v.sort();

    println!("The median is {}", &v[((v.len() / 2) as f32).floor() as usize]);

    let mut counts = HashMap::new();

    for i in &v {
        if counts.get(i).copied().unwrap_or(0) > 0 {
            counts.insert(i, counts.get(i).unwrap() + 1);
        } else {
            counts.insert(i, 1);
        }
    }

    let mut largest_count = 0;

    for (key, value) in &counts {
        if value > &counts.get(&largest_count).copied().unwrap_or(0) {
            largest_count = **key;
        }
    }

    println!("The mode is {}", largest_count);
}