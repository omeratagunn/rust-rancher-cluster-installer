#[path = "core/app.rs"] mod app;
#[path = "utils/measurable.rs"] mod measure;
use std::{env};
use colored::Colorize;
use crate::app::app;


fn main(){
   let start = measure::start_time();
   let args: Vec<String> = env::args().collect();
   if args.len() <= 1 {
      help();
      return;
   }
   let parsed_args = match_args(args);
   app(&parsed_args[0], &parsed_args[1]);
   println!("Time taken for installation: {} seconds", measure::finish_time(start))

}

fn match_args(args: Vec<String>) -> [String; 2] {
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

   return [path, k3s_version];


}

fn help() {
   println!("{}", "- Example usage: ./rancherinstall -- yaml <absoluteyamlfilepath>\n- Optionally you can pass k3s version -- k3s_version v1.23.13+k3s1".red().bold())
}
