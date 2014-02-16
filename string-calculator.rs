fn sum(expr:&str) -> int {
    match from_str::<int>(expr) {
        Some(value) => value,
        None => 0 
    }
}

#[cfg(not(test))]
fn main() {
    use std::io::BufferedReader;
    use std::io::stdin;
    
    let mut stdin = BufferedReader::new(stdin());
    let result = stdin.read_line();
    match result {
       Ok(line) => {
           println!("line:{}", line);
           println!("sum:{}", sum(line));
       },
       Err(io_error) => println!("problem reading line: {}", io_error)
    }
}

#[cfg(test)]
#[test]
fn should_add_empty_string_to_zero() {
    assert_eq!(0,sum(""));
}

#[test]
fn should_add_a_number_to_its_number() {
    assert_eq!(1,sum("1"));
}
