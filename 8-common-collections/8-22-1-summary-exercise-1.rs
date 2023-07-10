use std::collections::HashMap;

fn get_modes(integers: &Vec<i8>) {
    fn get_mode_type(length: usize) -> String {
        match length {
            2 => String::from("bimodal"),
            3 => String::from("trimodal"),
            _ => String::from("multimodal"),
        }
    }
    let mut occurrences: HashMap<i8, i8> = HashMap::new();
    for num in integers {
        let count = occurrences.get(num).unwrap_or(&0);
        occurrences.insert(*num, *&count + 1);
    }
    let mut modes: Vec<i8> = vec![];
    let mut max_times = 0;
    for (_, times) in &occurrences {
        if *times > max_times {
            max_times = *times;
        }
    }
    for (num, times) in &occurrences {
        if *times == max_times {
            modes.push(*num);
        }
    }
    modes.sort();
    if modes.len() == integers.len() {
        println!("Conjunto amodal\n");
        return;
    }
    if modes.len() == 1 {
        println!("Conjunto unimodal cuja moda é {0}, com {1} ocorrências\n", modes[0], max_times);
        return;
    }
    let mode_type = get_mode_type(modes.len());
    println!("Conjunto {0} cujas modas são {1:?}, com {2} ocorrências cada.\n", mode_type, modes, max_times);
}

fn get_median(integers: &Vec<i8>) {
    let is_even = integers.len() % 2 == 0;
    if is_even {
        let middle_index = integers.len() / 2;
        let first_middle = integers[middle_index - 1];
        let second_middle = integers[middle_index];
        let median = (first_middle + second_middle) as f32 / 2.0;
        println!("Median is {median}.\n");
        return;
    }
    let middle_index = (integers.len() - 1) / 2;
    println!("Median is {0}.\n", integers[middle_index]);
}

fn main() {
    println!("\nGiven a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.\n");
    let vector_of_vectors = vec![
        vec![1, 2, 0, 5, 3, 5, 4, 4],
        vec![1, 2, 0, 2, 5, 3, 5, 4, 4],
        vec![1, 2, 0, 2, 5, 3, 5, 4, 4, 0],
        vec![1, 2, 0, 2, 5, 3, 5, 4, 4, 0, 0],
        vec![0, 1, 2, 3, 4, 5],
    ];
    for mut vector in vector_of_vectors {
        vector.sort();
        println!("{:?}\n", vector);
        get_modes(&vector);
        get_median(&vector);
        println!("---------\n");
    }
}

