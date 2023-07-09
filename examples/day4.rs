use md5::compute;

// took two days bro
// just had to convert it to string
fn main() {
    // let output = format!("{:x}", md5::compute("abcdef609043"));
    // println!("{:?}", output);
    println!("{}", get_lowest_number_n_time_zero("iwrupvqb", 6));
    println!("{}", get_lowest_number("iwrupvqb"));
}

// NOTE - I think, I am going to write it a bit better
fn get_lowest_number(secret: &str) -> u32 {
    let mut i = 0;
    loop {
        let input = format!("{}{}", secret, i);
        // println!("{}", input);
        // use format! to convert to string
        let output: Vec<char> = format!("{:x}", compute(input)).chars().collect();
        // println!("{:?}", output);
        if output[0] == '0'
            && output[1] == '0'
            && output[2] == '0'
            && output[3] == '0'
            && output[4] == '0'
        {
            println!("{:?}", output);
            break;
        }
        i += 1;
    }

    println!("{} is the answer", i);
    return i;
}

fn get_lowest_number_n_time_zero(secret: &str, n: u8) -> u32 {
    let mut i = 0;
    loop {
        let input = format!("{}{}", secret, i);
        // println!("{}", input);
        // use format! to convert to string
        let mut output: Vec<char> = format!("{:x}", compute(input)).chars().collect();
        output.reverse();
        // println!("{:?}", output);
        let mut j = 0;
        while output.pop().expect("Error while pop") == '0' {
            j += 1;
        }

        if j == n {
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
    use super::{get_lowest_number, get_lowest_number_n_time_zero};

    #[test]
    fn test_1() {
        assert_eq!(get_lowest_number("abcdef"), 609043);
        assert_eq!(get_lowest_number_n_time_zero("pqrstuv", 5), 1048970);
    }
}
