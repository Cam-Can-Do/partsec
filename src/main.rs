use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "")]
    partition: String,

    #[arg(long, default_value = "")]
    volume: String,

    #[arg(long, default_value = "")]
    password: String,

    #[arg(long, default_value = "0")]
    pim: u32,

    #[arg(short, long, default_value = "")]
    keyfiles: String,
}

fn main() {
    let mut args = Args::parse();

    if args.volume.is_empty() {
        println!("Volume is required");
        print!("Volume: ");
        std::io::stdout().flush().unwrap();
        let mut volume = String::new();
        io::stdin().read_line(&mut volume).unwrap();
        args.volume = volume.trim().to_string();
    }

    if args.partition.is_empty() {
        println!("Partition is required");
        print!("Partition: ");
        std::io::stdout().flush().unwrap();
        let mut partition = String::new();
        io::stdin().read_line(&mut partition).unwrap();
        args.partition = partition.trim().to_string();
    }

    if args.password.is_empty() {
        println!("Password is required");
        print!("Password: ");
        std::io::stdout().flush().unwrap();
        args.password = rpassword::read_password().unwrap();
    }

    if args.pim.eq(&0) {
        println!("PIM is required");
        print!("PIM: ");
        std::io::stdout().flush().unwrap();
        let input = rpassword::read_password().unwrap();
        match input.parse::<u32>() {
            Ok(pim) => args.pim = pim,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid number for PIM.");
                std::process::exit(1);
            }
        }
    }

    if args.keyfiles.is_empty() {
        println!("Keyfile is required");
        print!("Keyfile: ");
        std::io::stdout().flush().unwrap();
        let mut keyfile = String::new();
        io::stdin().read_line(&mut keyfile).unwrap();
        args.keyfiles = keyfile.trim().to_string();
    }


    /*
    println!("The volume is: '{}'", args.volume);
    println!("The partition is: '{}'", args.partition);
    println!("The password is: '{}'", args.password);
    println!("The PIM is: '{}'", args.pim);
    println!("The keyfile is: '{}'", args.keyfiles);
    */

    match fs::create_dir(&args.volume) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    let _output = Command::new("veracrypt")
        .arg("--text")
        .arg("--mount")
        .arg(args.partition)
        .arg(args.volume)
        .arg("-p")
        .arg(args.password)
        .arg("--pim")
        .arg(args.pim.to_string())
        .arg("--keyfiles")
        .arg(args.keyfiles)
        .arg("--protect-hidden")
        .arg("no")
        .spawn()
        .expect("failed to execute process");
}
