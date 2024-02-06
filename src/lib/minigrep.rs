
pub mod minigrep {
    use std::env;
    use std::fs;
    use std::process;
    use std::error::Error;
    use std::vec;

    
pub fn run(config:config_command)-> Result<(), Box<dyn Error>>{

    //let file_content = fs::read_to_string(config.filename).expect("Not able to read file");
    //below line are error safe if error occured it return dynamic error 
    let file_content = fs::read_to_string(config.filename)?;
    let mut result_found:bool= false;
        
    if &config.case=="sensitive"{

        for line in search(&config.query, &file_content){
            println!("{}",line);
            if line.len()>0
            {result_found=true;}
            
        }
    }
    else {
        for line in search_case_insensitive(&config.query, &file_content){
            println!("{}",line);
            if line.len()>0
            {result_found=true;}
        }
    }
    if result_found==false{
        println!("no Match found");
    }
   
   // println!("{:?}", file_content);
    return Ok(());

}

pub fn search<'a>( query: &str , filecontent: & 'a str)-> Vec<&'a str> {
    let mut result= Vec::new();
     for line in filecontent.lines(){

        if line.contains(query){

            result.push(line);
        }
     }
     return result;
}

pub struct config_command {
  pub  query: String,
  pub case : String,
    pub filename: String,
}

//we can implement a struct
 impl config_command {
   pub fn new(args: &[String]) -> Result<config_command, &str> {
        if args.len()<3{
            // panic!("Not enough argument passed . Expecting 2 values");
           return Err(" Expecting atleast 2 values");
        }
        
        let mut query: String="".to_string() ;
        let mut filename : String="".to_string();
        let mut case: String="".to_string();
        if args.len()==3{
          
         query = args[1].clone();
         case="sensitive".to_string();
         filename = args[2].clone();  

        }
        else if args.len()==4 {
          
        if args[2]=="-i"{
            
        case="non_sensitive".to_string();
        }
        else {
            
        case="sensitive".to_string();
        }
        query = args[1].clone();
        filename =args[3].clone();  
        }
        return Ok((config_command {
            query: query,
            case:case,
            filename: filename,
        })); 
    }
}

pub fn search_case_insensitive<'a>(query: & str,content: &'a str)-> Vec<&'a str>{

    let mut result= Vec::new();
    let tempquery=query.to_uppercase();
     for line in content.lines(){

        if line.to_uppercase().contains(&tempquery){

            result.push(line);
        }
     }
     return result;
}

#[test]
fn case_insesnstive(){
    let query= "ruSt";
    let content = "\
hello my
rust
but I am not
Trust me.";
    assert_eq!(vec!["rust","Trust me."],search_case_insensitive(query,content));

}
}
