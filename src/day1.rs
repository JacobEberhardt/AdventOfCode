pub fn calculate_reverse_captcha_star1(input: &str ) -> u32 {
    let mut sum = 0;

    let chars:Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        if chars[i] == chars[ (i+1) % chars.len()] {
            sum += chars[i].to_digit(10).unwrap();
        } 
    }
    sum
}

pub fn calculate_reverse_captcha_star2(input: &str ) -> u32 {
    let mut sum = 0;

    let chars:Vec<char> = input.chars().collect();
    assert!(chars.len() % 2 == 0); // input length is even
    let offset = chars.len()/2;

    for i in 0..chars.len() {
        if chars[i] == chars[ (i+offset) % chars.len()] {
            sum += chars[i].to_digit(10).unwrap();
        } 
    }
    sum
}