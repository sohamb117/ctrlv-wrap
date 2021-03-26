use clap::Clap;                                                                          
use std::path::PathBuf;                                                                  
                                                                                         
#[derive(Clap)]                                                                          
#[clap(name = "ctrlv-wrap", about = "Simple Wrapper for Ctrl-V")]                                   
pub struct Args {                                                                        
    /// Input file                                                                       
    #[clap(required=false, parse(from_os_str))]                                                          
    pub file: PathBuf,
    /// Title of the file                                                  
    #[clap(short, long)]                                                                 
    pub title: String,                                                      
    /*/// Input language                                                                   
    #[clap(long, short, required=false)]                                                
    pub lang: String,                                                        
    /// Expiry date                                                           
    #[clap(short, long, required=false)]                                                           
    pub expiry: String,                                                        
    /// Password                                                           
    #[clap(short, long, required=false)]                                                           
    pub pass: String,*/                                                     
} 
