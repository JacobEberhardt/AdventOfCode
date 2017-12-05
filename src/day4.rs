use std::collections::HashSet;

pub fn count_valid_passphrases_star1(input: &str)->u32{
    let mut valid_passphrase_counter = 0;
    
    for l in input.lines().collect::<Vec<&str>>() {
        let mut dedup_tokens: HashSet<&str> = HashSet::new();
        let tokens = l.split_whitespace().collect::<Vec<&str>>();
        for t in &tokens {
            dedup_tokens.insert(t);
        }
        if tokens.len() == dedup_tokens.len(){
            valid_passphrase_counter += 1;
        }
    }
    
    valid_passphrase_counter
}

pub fn count_valid_passphrases_star2(input: &str)->u32{
    let mut valid_passphrase_counter = 0;
    
    for l in input.lines().collect::<Vec<&str>>() {
        let mut dedup_tokens: HashSet<String> = HashSet::new();
        let tokens = l.split_whitespace().collect::<Vec<&str>>();
        for t in &tokens {
            let mut chars: Vec<char> = t.chars().collect();
            chars.sort();
            let sorted = chars.into_iter().collect::<String>();
            dedup_tokens.insert(sorted.clone());
        }
        
        if tokens.len() == dedup_tokens.len(){
            valid_passphrase_counter += 1;
        }
    }
    
    valid_passphrase_counter
}