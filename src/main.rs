use clap::{Arg, ArgAction, Command};
fn main() {
    let matches = Command::new("slice")
        .version("0.1.0")
        .author("Mohammad")
        .about("This program takes text from the user and \'slice\' the first word.\nThe first word is indicated to the program by a space.")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("prints the process of finding the first word by slicing each letter until a space is found.")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    let verbose:bool = matches.get_flag("verbose");
    let text:Vec<String> = matches.get_many("text").unwrap().cloned().collect();
    let text_as_string = text.join(" ");
    let first_word = get_first_word(&text_as_string, verbose);

    if verbose {
        println!("FIRST WORD OF THE STRING \"{text_as_string}\", IS:\n{first_word}");
    } else{
        println!("{first_word}");
    }
    std::process::exit(0);
}
fn get_first_word(s:&String, verbose:bool) -> String{
    let mut fi = String::new();
    if verbose {
        println!("trimming the text using \'trim()\' so if there are spaces in the beging they are ignored safely.");
    }
    let string = s.trim();
    for character in string.chars(){
        if character == ' '{
            if verbose {
                println!("\nfound space. escaping loop.");
            }
            break;
        }
        if verbose {
            print!("char = {character}, ");    
        }
        fi.push_str(&character.to_string());
    }
    fi
}

/* THE FOLLOWING CODE HERE IS COMMENTED OUT. THUS IT DOESN'T CHANGE AYTHING
 * 
 * fn get_string() -> String{
    print!("PLEASE INPUT THE STRING YOU WANT\nTHEN HIT ENTER\nSTRING: ");
    let _ = io::stdout().flush(); // to use this method or function you have to "use std::io::Write;" first.
    let mut string=String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");
    string
}*/

