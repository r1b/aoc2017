pub fn redistribute(input: &'static str) -> u32 {
    let mut banks: Vec<u32> = read_banks(input);
    let mut memory: Vec<Vec<u32>> = Vec::new();
    let mut steps: u32 = 0;
    let size: usize = banks.len();
    let mut cycles: Option<usize> = None;

    loop {
        let mut max: u32 = 0;
        let mut idx: Option<usize> = None;

        memory.push(banks.clone()); // matthias i can hear you screaming but i swear i need to
        steps +=1;

        for (index, value) in banks.iter().enumerate() {
            if *value > max {
                max = *value;
                idx = Some(index);
            }
        }

        for index in 1..size+1 {
            if banks[idx.unwrap()] == 0 {
                break;
            }
            banks[idx.unwrap()] -= 1;
            banks[(idx.unwrap() + index) % size] += 1;
        }

        for (index, state) in memory.iter().enumerate() {
            if *state == banks {
                cycles = Some(memory.len() - index);
                println!("cycles: {}", cycles.unwrap());
                return steps
            }
        }
    }
}

fn read_banks(input: &'static str) -> Vec<u32> {
    input.trim().split_whitespace().map(|item| u32::from_str_radix(item, 10).unwrap()).collect()
}
