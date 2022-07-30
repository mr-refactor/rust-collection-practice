use std::collections::HashMap;

pub fn find_median(numbers: &mut Vec<i32>) -> f32 {    
    for i in 0..numbers.len() {
        let mut already_sorted = true;

        for j in 0..numbers.len() - 1 - i {
            if numbers[j] > numbers[j + 1]{
                let temp = numbers[j + 1];
                numbers[j + 1] = numbers[j];
                numbers[j] = temp;
            }

            already_sorted = false
        }

        if already_sorted {
            break;
        }
    }

    if numbers.len() % 2 == 0 {
        let index_i = numbers.len() / 2;
        let index_j = index_i - 1;
        (numbers[index_i] as f32 + numbers[index_j] as f32) / 2f32
    } else {
        let median_index = numbers.len() / 2;
        numbers[median_index] as f32
    }
}

pub fn find_mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut mode = (1, numbers[0]);

    for i in 0..numbers.len() {
        let count = map.entry(numbers[i]).or_insert(0);
        *count += 1;
    
        if count > &mut mode.0 {
            mode.0 = *count;
            mode.1 = numbers[i]
        }
    }
    mode.1
}