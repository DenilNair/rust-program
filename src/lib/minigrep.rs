
pub mod minigrep {
    use std::env;
    use std::fs;
    use std::process;
    use std::error::Error;

    
pub fn run(config:config_command)-> Result<(), Box<dyn Error>>{

    //let file_content = fs::read_to_string(config.filename).expect("Not able to read file");
    //below line are error safe if error occured it return dynamic error 
    let file_content = fs::read_to_string(config.filename)?;
    println!("{:?}", file_content);
    return Ok(());

}
pub struct config_command {
  pub  syntax: String,
    pub filename: String,
}

//we can implement a struct
 impl config_command {
   pub fn new(args: &[String]) -> Result<config_command, &str> {
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

}
