pub fn calculate_reverse_captcha(input: &str ) -> u32 {
    let mut sum = 0;

    let chars:Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        if chars[i] == chars[ (i+1) % chars.len()] {
            sum += chars[i].to_digit(10).unwrap();
        } 
    }
    sum
}