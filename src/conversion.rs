pub fn decimal_to_binary(num: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut binary_string = String::new();
    let mut n = num;

    while n > 0 {
        let remainder = n % 2;
        binary_string.insert(0, std::char::from_digit(remainder, 10).unwrap());
        n /= 2;
    }

    binary_string
}

/// This would even work even if the was not binary and had numbers that were not 0 or 1 
/// Which is why i labelled it as `buggy_bin_to_dec`
pub fn buggy_bin_to_dec(num: u32) -> String{
    // Return zero if zero is passed
    if num == 0 {
        return "0".to_string();
    }
    
    // Change the binding to a string to elongate the lifetime of the string
    let binding = num.to_string();
    // Change it to a character value
    let split_num  = binding.chars();
    // Reverse it to match the enumerator index multiplication of base 2 in binary conversion
    let split_num_rev = split_num.rev();

    // Create a blank sum to collate the addition of the calculated base 2 numbers
    let mut stored_sum = 0;
    for (index, char) in split_num_rev.into_iter().enumerate(){
        let num = char.to_digit(10).unwrap();
        let value = num * 2_u32.pow(index as u32);
        stored_sum += value
    }
    stored_sum.to_string()
}

/// This however is the better code and runs only with binary allowed numbers
pub fn good_bin_to_dec(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    
    let mut temp_num = num;
    let mut sum = 0;
    let mut power = 0;
    while temp_num > 0 {
        let digit = temp_num % 10;
        if digit > 1 {
            panic!("cannot use a non - binary number");
        }
        sum += digit * 2_u32.pow(power);
        power += 1; 
        temp_num /=10;
    }
    sum
}