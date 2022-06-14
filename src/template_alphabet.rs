// just a template for getting the string and for looping throught the string for and check for chars and print the output

use std::str;

pub fn alphabet(inputstring: &str)
{
    // make rows to print out and also make it mutable
    let mut row0 = "".to_string();
    let mut row1 = "".to_string();
    let mut row2 = "".to_string();
    let mut row3 = "".to_string();
    let mut row4 = "".to_string();
    let mut row5 = "".to_string();
    // for loop through the chars of the string
    // for the ascii text just use figlet as this is a figlet clone in rust
    for i in inputstring.chars()
    {
        // small case
        if i == 'a'
        {
        }
        else if i == 'b'
        {
        }
        else if i == 'c'
        {
        }
        else if i == 'd'
        {
        }
        else if i == 'e'
        {
        }
        else if i == 'f'
        {
        }
        else if i == 'g'
        {
        }
        else if i == 'h'
        {
        }
        else if i == 'i'
        {
        }
        else if i == 'j'
        {
        }
        else if i == 'k'
        {
        }
        else if i == 'l'
        {
        }
        else if i == 'm'
        {
        }
        else if i == 'n'
        {
        }
        else if i == 'o'
        {
        }
        else if i == 'p'
        {
        }
        else if i == 'q'
        {
        }
        else if i == 'r'
        {
        }
        else if i == 's'
        {
        }
        else if i == 't'
        {
        }
        else if i == 'u'
        {
        }
        else if i == 'v'
        {
        }
        else if i == 'x'
        {
        }
        else if i == 'w'
        {
        }
        else if i == 'y'
        {
        }
        else if i == 'z'
        {
        }
        else if i == 'A'
        {
        }
        else if i == 'B'
        {
        }
        else if i == 'C'
        {
        }
        else if i == 'D'
        {
        }
        else if i == 'E'
        {
        }
        else if i == 'F'
        {
        }
        else if i == 'G'
        {
        }
        else if i == 'H'
        {
        }
        else if i == 'I'
        {
        }
        else if i == 'J'
        {
        }
        else if i == 'K'
        {
        }
        else if i == 'L'
        {
        }
        else if i == 'M'
        {
        }
        else if i == 'N'
        {
        }
        else if i == 'O'
        {
        }
        else if i == 'P'
        {
        }
        else if i == 'Q'
        {
        }
        else if i == 'R'
        {
        }
        else if i == 'S'
        {
        }
        else if i == 'T'
        {
        }
        else if i == 'U'
        {
        }
        else if i == 'V'
        {
        }
        else if i == 'W'
        {
        }
        else if i == 'X'
        {
        }
        else if i == 'Y'
        {
        }
        else if i == 'Z'
        {
        }
        else if i == ' '
        {
        }
        else
        {
            row0 = row0 + &i.to_string();
        }
    }
    println!("{}", row0);
}
