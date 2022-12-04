#[path = "core/app.rs"] mod app;

use std::{env, process};
use colored::Colorize;
use openssl_sys::exit;
use crate::app::app;




fn main(){
   let args: Vec<String> = env::args().collect();
   match args.len() {
      1 => {
         println!("Try passing some arguments!");
      },
      // one argument passed
      2 => {
         match args[1].parse() {
            Ok(42) => println!("This is the answer!"),
            _ => help(),
         }
      },
      // one command and one argument passed
      3 => {
         let mut filepath = &args[2];
         println!("Given file path {}", filepath);
         let mut path = String::from(filepath);
         app(path)
      },
      // all the other cases
      _ => {
         // show a help message
         help();
      }
   }

}

fn help() {
   println!("{}", "Usage: ./rancherinstall -- config <absoluteyamlfilepath>".red().bold())
}
