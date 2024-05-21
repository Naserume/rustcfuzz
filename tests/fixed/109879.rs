fn main() {

let value_size = (true,false);
let number = 110;
// I test many numbers at "number" variable.
// At 99, 101 value, it worked well. but when I use "100" it start to broke up, and it keep doesn't work,

if number>100 {
    print_msg(value_size.0);
    } else {
        print_msg(value_size.1);
    }
}    

// * Use a function to print the messages
fn print_msg (result : bool) {
    // * Use a match expression to determine which message
    //   to print
    match result {
        true => println!("its big"),
        false => println!("its small"),
    }
    
}
