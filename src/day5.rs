pub fn count_jumps_to_exit_star1(input: &str)->u32{
    let mut jump_offsets: Vec<i32> = input.lines().map(|x|x.parse::<i32>().unwrap()).collect();
    println!("{:?}", jump_offsets);

    let mut jump_counter = 0;
    let mut current_pos = 0;
    loop{
        let current_index = current_pos as usize; 
        let next_pos = current_pos + jump_offsets[current_index];
        if next_pos < 0 || next_pos >= jump_offsets.len() as i32 {
            break;
        } else {
            jump_offsets[current_index] += 1;
            current_pos = next_pos;
            jump_counter += 1;
        }     
    }
    jump_counter + 1 // +1 since escaping the maze is a step
}

pub fn count_jumps_to_exit_star2(input: &str)->u32{
    let mut jump_offsets: Vec<i32> = input.lines().map(|x|x.parse::<i32>().unwrap()).collect();
    println!("{:?}", jump_offsets);

    let mut jump_counter = 0;
    let mut current_pos = 0;
    loop{
        let current_index = current_pos as usize; 
        let next_pos = current_pos + jump_offsets[current_index];
        if next_pos < 0 || next_pos >= jump_offsets.len() as i32 {
            break;
        } else {
            if next_pos-current_pos >= 3 {
                jump_offsets[current_index] -= 1;
            } else {
                jump_offsets[current_index] += 1;
            }
            current_pos = next_pos;
            jump_counter += 1;
        }   
    }
    jump_counter + 1 // +1 since escaping the maze is a step
}