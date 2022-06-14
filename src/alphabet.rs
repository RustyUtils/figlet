use std::str;
//use std::string;

pub fn alphabet(string: &str)
{
    for i in string.chars()
    {
        // my idea get rowss (horizontal) and print them
        if i == 'a'
        {
            println!("  /\\ ");
            println!(" /  \\ ");
            println!("/    \\");
        }
        else
        {
            println!("{}", i);
        }
    }
}
