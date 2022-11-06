fn main() {
    // Create the array and fill it with numbers
    let mut data: Vec<i64> = vec![];
    for i in 0..10_000_000 {
        data.push(i as i64)
    }

    // Call the functions
    while_loop(&data);
    counted_loop(&data);
    for_loop(&data);
    iter_loop(&data);
}

// Function to get the current time since epoch in milliseconds
fn get_start_time() -> u128 {
    return std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
}

// While Loop
fn while_loop(data: &[i64]) {
    let start_time: u128 = get_start_time();
    let mut i: i64 = 0;
    while i < data.len().try_into().unwrap() {
        i += 1;
    }
    println!("While Loop: {}ms", get_start_time() - start_time);
}

// Counted For Loop 
fn counted_loop(data: &[i64]) {
    let start_time: u128 = get_start_time();
    let mut i: i64 = 0;
    for _ in 1..data.len() {
        i += 1;
    }
    println!("Counted For Loop: {}ms", get_start_time() - start_time);
}

// For Loop
fn for_loop(data: &[i64]) {
    let start_time: u128 = get_start_time();
    let mut i: i64 = 0;
    for _ in data {
        i += 1;
    }
    println!("For Loop: {}ms", get_start_time() - start_time);
}

// Loop using Iter and ForEach
fn iter_loop(data: &[i64]) {
    let start_time: u128 = get_start_time();
    let mut i: i64 = 0;
    data.iter().for_each(|_| {
        i += 1;
    });
    println!("Iter Loop: {}ms", get_start_time() - start_time);
}
