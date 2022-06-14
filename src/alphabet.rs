use std::str;
use std::format;
//use std::string;

pub fn alphabet(string: &str)
{
    // make rows to print out and also make it mutable
    let mut row0: &str = "";
    let mut row1: &str = "";
    let mut row2: &str = "";
    let mut row3: &str = "";
    let mut row4: &str = "";
    // tmp
    let tmp = "";
    for i in string.chars()
    {
        // my idea get rowss (horizontal) and print them
        if i == 'a'
        {
            row0 = format!("{}{}", row0, " Hello testing this   ");
            row1 = format!("{}{}", row1, " Hello testing this 1 ");
            row2 = format!("{}{}", row2, " Hello testing this 2 ");
            row3 = format!("{}{}", row3, " Hello testing this 3 ");
            row4 = format!("{}{}", row4, " Hello testing this 4 ");
        }
        else if i == 'A'
        {
            row0 = format!("{}{}", row0, " Hello testing this   ");
            row1 = format!("{}{}", row1, " Hello testing this 1 ");
            row2 = format!("{}{}", row2, " Hello testing this 2 ");
            row3 = format!("{}{}", row3, " Hello testing this 3 ");
            row4 = format!("{}{}", row4, " Hello testing this 4 ");
        }
        else
        {
            // if anything else just print dont print anything
            //row0 = i;
            row0 = row0;
            row1 = row1;
            row2 = row2;
            row3 = row3;
            row4 = row4;
        }
    }
    // return the output 
    println!(row0);
    println!(row1);
    println!(row2);
    println!(row3);
    println!(row4);
}
