#[path = "core/app.rs"] mod app;

use std::{env};
use colored::Colorize;
use crate::app::app;


fn main(){
   let args: Vec<String> = env::args().collect();
   if args.len() <= 1 {
      help();
      return;
   }
   match_args(args);

}

fn match_args(args: Vec<String>){
   let mut path = String::new();
   let mut k3s_version = String::new();
   for (i, arg) in args.iter().enumerate(){
      if arg == "config" && args[i + 1].contains("/")  {
         path.push_str((&args[i + 1]));
      }
      if arg == "k3s_version" && args[i + 1].contains("v")  {
         k3s_version.push_str((&args[i + 1]));
      }
   }
   if k3s_version.len() == 0 {
      k3s_version.push_str("v1.23.13+k3s1")
   }

   app(path, k3s_version);
}

fn help() {
   println!("{}", "- Example usage: ./rancherinstall -- yaml <absoluteyamlfilepath>\n- Optionally you can pass k3s version -- k3s_version v1.23.13+k3s1".red().bold())
}
