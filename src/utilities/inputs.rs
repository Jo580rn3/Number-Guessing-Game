use crate::{utilities::cls_scr::cls_title, Comunication, RuntimeFunctionBlob};
use std::io;

pub fn numeric_input(msg: &String) -> u32 {
    let mut wrong: bool = false;
    let mut user_in_alpha: String = String::new(); //

    loop {
        match wrong {
            false => {
                println!("{}", msg);
            }
            true => {
                cls_title();
                println!(
                    "{}\n'{}' isn't a valid input. Please try again.",
                    msg,
                    user_in_alpha.trim()
                );
                user_in_alpha = String::new();
            }
        };

        io::stdin()
            .read_line(&mut user_in_alpha)
            .expect("Failed to read line");

        if let Ok(user_in_u32) = user_in_alpha.trim().parse::<u32>() {
            return user_in_u32;
        } else if user_in_alpha.trim().parse::<String>().unwrap() == "" {
            return 0;
        } else {
            wrong = true
        }
    }
}

pub fn yes_no_else_input(comunication: &Comunication, wrong: &bool) -> String {
    let mut user_in_alpha: String = String::new();
    match wrong {
        false => println!("{}", comunication.msg),
        true => {
            println!(
                "{}\n'{}' isn't a valid input. Please try again.",
                comunication.msg, comunication.user_in_alpha
            );
        }
    };

    io::stdin()
        .read_line(&mut user_in_alpha)
        .expect("Failed to read line");

    user_in_alpha.trim().to_string()
}

pub fn name_input(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    let mut wrong: bool = false;
    let mut user_in_alpha_num: String;
    runtime_blob.comunication.msg = format!("\t\tPlease enter a 4 letter name :\n\n\t\t\t- ");
    while wrong {
        cls_title();
        user_in_alpha_num = yes_no_else_input(&runtime_blob.comunication, &wrong);
        wrong = match user_in_alpha_num.chars().count() {
            4 => false,
            _ => true,
        };
    }
    return runtime_blob;
}
