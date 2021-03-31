use std::collections::HashMap;

pub fn mean_median_mode() {
    
    let mut ints = vec![2, 6, 5, 7, 3, 6, 8, 2, 2];
    println!("{:?}", ints);

    let mut sum = 0;
    for i in &ints {
        sum += i;
    }
    let mean = (sum as f32) / (ints.len() as f32);
    println!("Mean: {}", mean);

    ints.sort();
    let midpoint = ints.len() / 2;
    let median: f32;
    if ints.len() % 2 == 0 {
        println!("Even");
        let m1 = match ints.get(midpoint) {
            Some(m) => *m,
            None => panic!("Out of bounds"),
        };
        let m2 = match ints.get(midpoint - 1) {
            Some(m) => *m,
            None => panic!("Out of bounds"),
        };
        median = (m1 + m2) as f32 / 2.0
    } else {
        median = match ints.get(midpoint) {
            Some(m) => *m as f32,
            None => panic!("Out of bounds"),
        };
    }
    println!("Median: {}", median);

    let mut map = HashMap::new();
    for i in &ints {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut highest = (0, 0);
    for (key, value) in &map {
        if value > &highest.1 {
            highest = (**key, *value);
        }
    }
    println!("Mode: {}", highest.0);
}
