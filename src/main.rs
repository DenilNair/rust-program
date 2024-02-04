use std::env;
use std::fs;
use std::process;
use std::error::Error;

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
    println!("from struct Search String = {:?}", parse_impl_struct.syntax);
    println!("from struct Filename = {:?}", parse_impl_struct.filename);

    //run(parse_impl_struct);
    
    //above method will not throw any error . to catch error and display it on screen use below code
    if let Err(e)=run(parse_impl_struct) {
        println!("Application error {:?}",e);
        process::exit(1);
    } ;

    println!("Hello, world!");
}


fn run(config:config_command)-> Result<(), Box<dyn Error>>{

    //let file_content = fs::read_to_string(config.filename).expect("Not able to read file");
    //below line are error safe if error occured it return dynamic error 
    let file_content = fs::read_to_string(config.filename)?;
    println!("{:?}", file_content);
    return Ok(());

}
struct config_command {
    syntax: String,
    filename: String,
}

//we can implement a struct
impl config_command {
    fn new(args: &[String]) -> Result<config_command, &str> {
        if args.len()<3{
            // panic!("Not enough argument passed . Expecting 2 values");
           return Err(" Expecting 2 values");
        }
        let query: String = args[1].clone();
        let filename = args[2].clone();
        return Ok((config_command {
            syntax: query,
            filename: filename,
        })); 
    }
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
        syntax: query,
        filename: filename,
    };
}
