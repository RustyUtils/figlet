// allow snake name
#![allow(non_snake_case)]
// packages
use std::env;
//use std::string; // dont need it yet
// files
//exmaple #[path = "./command-line-args.rs"] mod commandlineargs;
#[path = "./normal_alphabet.rs"] mod normal_alphabet;

pub fn main() 
{
    // get command line arguments
    let args: Vec<String> = env::args().skip(1).collect(); // skip ./biglet
    let args_modifer: Vec<String> = env::args().skip(2).collect(); // skip ./biglet
    let arg_length: u8 = args.len().try_into().unwrap();

    // check if length of argument in less than 1 to make error
    if !(arg_length > 0)
    {
        println!("Please enter only 1 or 2 arguments no more or less. You do not have an arg.");
        println!("Help: ./biglet -h or ./biglet --help");
        println!("-----------------------");
        println!("Program made by Thamognya Kodi AGPL3.0-or-later");
        println!("Source Code: https://git.thamognya.com/Thamognya/BigLet or https://github.com/Thamognya/BigLet");
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
        // let arg1 = &args[1]; // arg 1 can contain -h or etc or just the text to increase. We can just use args[1]
        // check for -h or any other modifiers before just 
        if args[0].eq("-h") || args[0].eq("--help") // if arg1 is equal to -h
        {
            println!("USAGE: \n\t./biglet [options(optional)] [args]");
            println!("ARGS: \n\t<args>...");
            println!("OPTIONS: ");
            println!("\t-h or --help: Help sheet");
            println!("\t-i or --italic: italic text");
            println!("\t-b or --bold: bold text");
            println!("EXAMPLES: ");
            println!("\tbiglet Hello \n\tbiglet -i Hello");
            println!("-----------------------");
            println!("Program made by Thamognya Kodi AGPL3.0-or-later");
            println!("Source Code: https://git.thamognya.com/Thamognya/BigLet or https://github.com/Thamognya/BigLet");
        }
        else if args[0].eq("-i") || args[0].eq("--italic")
        {
            for chunk in args_modifer.chunks(4)
            {
                normal_alphabet::alphabet(&chunk.join(" "));
            }
        }
        else if args[0].eq("-b") || args[0].eq("--bold")
        {
            for chunk in args_modifer.chunks(4)
            {
                normal_alphabet::alphabet(&chunk.join(" "));
            }
        }
        else
        {
            for chunk in args.chunks(4)
            {
                normal_alphabet::alphabet(&chunk.join(" "));
            }
        }
    }
}
