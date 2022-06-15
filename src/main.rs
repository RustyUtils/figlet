// allow snake name
#![allow(non_snake_case)]
// packages
use std::env;
//use std::string; // dont need it yet
// files
//exmaple #[path = "./command-line-args.rs"] mod commandlineargs;
#[path = "./normal_alphabet.rs"] mod normal_alphabet;

pub fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    let arg_string: String; // combine all strings to make one complete string of arg (it is defined in the if else below as I have modifiers)
    // get the arg length
    let arg_length: u8 = args.len().try_into().unwrap(); // using unsigned as comand line args can go below 0
    // check if length of argument in less than 1 to make error
    if !(arg_length > 1)
    {
        println!("Please enter only 1 or 2 arguments no more or less. You do not have an arg.");
        println!("Usage: ./biglet -h or ./biglet _word_here_");
        println!("Program made by Thamognya Kodi AGPL3.0-or-later");
    }
    // or more than 2
    /* remove this as after -h or -_identifier_ there can be many words that need to be big let
    else if arg_length > 3
    {
        println!("Please enter only 1 or 2 arguments no more or less. You have more than 2 args.");
        println!("Usage: ./biglet -h or ./biglet _word_here_");
        println!("Program made by Thamognya Kodi AGPL3.0-or-later");
    }
    // if not then the code is correct
    */ 
    else
    {
        // asign variables for the args
        let arg1 = &args[1]; // arg 1 can contain -h or etc or just the text to increase
        //let _arg2 = &args[2]; // arg 2 contains only after a modifier from arg1
        // check for -h or any other modifiers before just 
        // make array of arguments based on the arg_lenthg
        if arg1.eq("-h") // if arg1 is equal to -h
        {
            println!("Testing this first");
        }
        else if arg1.eq("-i")
        {
            // will make this italic
            arg_string = args[2..].join(" "); // combine all strings after ./biglet -i to make one complete string of arg
            println!("{}", arg_string);
            println!("Work in progress");
        }
        else if arg1.eq("-b")
        {
            // make this bolded
            arg_string = args[2..].join(" "); // combine all strings after ./biglet -b to make one complete string of arg
            println!("{}", arg_string);
            println!("Work in progress");
        }
        else
        {
            // use the normal alphabet conversion into big letters
            arg_string = args[1..].join(" "); // combine all strings after ./biglet to make one complete string of arg
            println!("{}", arg_string);
            normal_alphabet::alphabet(&arg_string);
        }
    }
}
