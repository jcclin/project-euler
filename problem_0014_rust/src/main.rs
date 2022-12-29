use std::collections::HashMap;

fn move_once(n:u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}


fn find_num_steps(n: u64, num_step_map: &mut HashMap<u64, usize>) -> usize {
    if num_step_map.contains_key(&n) {
        return *(num_step_map.get(&n).unwrap());
    }
    let num_steps = 1 + find_num_steps(move_once(n), num_step_map);
    num_step_map.insert(n, num_steps);
    return num_steps;
}


const N_MAX: u64 = 1000000 - 1;

fn main() {
    println!("Hello, world!");

    let mut num_step_map: HashMap<u64, usize> = HashMap::new();
    num_step_map.insert(1, 1);

    let mut max_num_steps: usize = 1;
    let mut max_num_steps_n: u64 = 1;
    for n in 1..=N_MAX {
        if n % 10000 == 0 {
            println!("........ {}", n);
        }
        let num_steps = find_num_steps(n, &mut num_step_map);
        if num_steps > max_num_steps {
            max_num_steps = num_steps;
            max_num_steps_n = n;
        }
    }

    println!("find_num_steps({}) = {}", max_num_steps_n, max_num_steps);
}
