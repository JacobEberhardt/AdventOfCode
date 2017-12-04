pub fn calculate_corruption_checksum_star1(input: &str) -> u32 {
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    // parse input
    for l in input.lines() {
        matrix.push(l.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect());
    }
    println!("Matrix: {:?}", matrix);

    // calculate checksum
    let mut checksum: u32 = 0;
    for row in matrix.into_iter(){
        let max = row.clone().into_iter().fold(0,|acc,x|if x>acc {x} else {acc});
        let min = row.clone().into_iter().fold(0,|acc,x|if acc==0 {x} else {if x<acc {x} else {acc}});
        println!("{}, {}",max,min);
        checksum += max-min;
    }
    checksum 
}

pub fn calculate_corruption_checksum_star2(input: &str) -> u32 {
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    // parse input
    for l in input.lines() {
        matrix.push(l.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect());
    }
    println!("Matrix: {:?}", matrix);

    // calculate checksum
    let mut checksum: u32 = 0;
    for row in matrix.into_iter(){
        let mut div_res = 0;
        for fac1 in row.clone().into_iter(){
            for fac2 in row.clone().into_iter(){
                if (fac1!=fac2&&fac1%fac2==0){
                    div_res= fac1/fac2;
                 
                }
                println!("factor 1: {}, factor2: {}, result: {}",fac1,fac2,div_res);
            }
        }
        
        checksum += div_res;
    }
    checksum 
}