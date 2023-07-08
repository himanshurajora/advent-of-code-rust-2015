use md5::compute;

fn main() {
    // let output = md5::compute("abcdef609043");
    // println!("{:?}", output);
  println!("{}",  get_lowest_number("abcdef")); 
}

fn get_lowest_number(secret: &str) -> u32 {
    let mut i = 0;
    loop {
        let input = format!("{}{}", secret, i);
        println!("{}", input);
        let output = compute(input);
        if output[0] == 0
            && output[1] == 0
            && output[2] == 0
            && output[3] == 0
            && output[4] == 0
        {
            println!("{:?}", output);
            break;
        }
        i += 1;
    }

    println!("{} is the answer", i);
    return i;
}

#[cfg(test)]
mod test_module {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_lowest_number("abcdef"), 609043);
    }
}
