use std::env;
use std::process;
use lib::minigrep::minigrep::config_command;
mod lib {
    pub mod minigrep;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    //let query= &args[1];
    let filename =  "";
    //let (query, filename) = parse_config(&args);

    //let parse_struct_value = parse_config_struct(&args);

    //to use unwrap_or_else we need to change return type of the method new() to Result
    let parse_impl_struct = config_command::new(&args).unwrap_or_else(|err|{
        println!("Not enough argument passed . {}",err);
        process::exit(1);
    });

    // normal method return tuple
    /*println!("Search String = {:?}",query);
    println!("Filename = {:?}",filename);
    */

    // method return structure data type using seperate method
    /*
    println!("from struct Search String = {:?}",parse_struct_value.syntax);
    println!("from struct Filename = {:?}",parse_struct_value.filename);
    */

    //method return struct data type which is implemented
    //println!("from struct Search String = {:?}", parse_impl_struct.query);
    //println!("from struct Filename = {:?}", parse_impl_struct.filename);

    //run(parse_impl_struct);
    
    //above method will not throw any error . to catch error and display it on screen use below code
    if let Err(e)=lib::minigrep::minigrep::run(parse_impl_struct) {
        println!("Application error {:?}",e);
        process::exit(1);
    } ;
}


// return (&str, &str) is known as tuple in RUST
/*
fn parse_config(args: &[String]) -> (&str, &str) {
    // &args[1] means reference to array not the actual value  Remove & (i.e args[1].clone() to pass directly value String instead of &String)
    let query = &args[1];
    let filename = &args[2];
    return (query, filename);
}
 */

fn parse_config_struct(args: &[String]) -> (config_command) {
    if args.len()<3 {
        panic!("Not enough argument passed . Expecting 2 values");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    return config_command {
        query: query,
        case:"sensitive".to_string(),
        filename: filename,
    };
}
