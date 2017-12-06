pub fn count_cycles_until_known_state(input: &str) -> u32 {
    // parse & init state
    let mut memory: Vec<u32> = input.split_whitespace().map(|x|x.parse::<u32>().unwrap()).collect();
    let mut snapshots: Vec<Vec<u32>> = Vec::new();

    // execute redistribution cycles
    let mut cycles = 0;

    loop{
        // Exit condition
        if snapshots.contains(&memory) {
            break;
        }

        // store memory snapshot    
        snapshots.push(memory.clone());

        // determine block to redistribute
        let mut max_index: usize = 0;
        let mut max_value: u32 = 0;
        for (index,value) in memory.iter().enumerate() {
            if *value > max_value {
                max_value = *value;
                max_index = index;
            }
        }

        // redistribute block
        let mut remaining = max_value;
        let mut index_counter = max_index;
        let memory_size = memory.len();
        memory[max_index] = 0;
        loop{
            if remaining == 0 {
                break;
            } else {
                memory[(index_counter +1) % memory_size] += 1;
                index_counter += 1;
                remaining -= 1;
            }
        }
        cycles += 1;
    }
    cycles
}

pub fn determine_loop_size(input: &str) -> usize {
    // parse & init state
    let mut memory: Vec<u32> = input.split_whitespace().map(|x|x.parse::<u32>().unwrap()).collect();
    let mut snapshots: Vec<Vec<u32>> = Vec::new();

    // execute redistribution cycles
    let mut cycles = 0;

    loop{
        // Exit condition
        if snapshots.contains(&memory) {
            break;
        }

        // store memory snapshot    
        snapshots.push(memory.clone());

        // determine block to redistribute
        let mut max_index: usize = 0;
        let mut max_value: u32 = 0;
        for (index,value) in memory.iter().enumerate() {
            if *value > max_value {
                max_value = *value;
                max_index = index;
            }
        }

        // redistribute block
        let mut remaining = max_value;
        let mut index_counter = max_index;
        let memory_size = memory.len();
        memory[max_index] = 0;
        loop{
            if remaining == 0 {
                break;
            } else {
                memory[(index_counter +1) % memory_size] += 1;
                index_counter += 1;
                remaining -= 1;
            }
        }
        cycles += 1;
    }
    
    let mut memory_index = 0;
    for (index,value) in snapshots.iter().enumerate() {
        if *value == memory {
            memory_index = index;
        }
    }
    cycles - memory_index
}
