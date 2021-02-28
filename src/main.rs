use std::fs;
use std::env;
use std::io;
use std::io::Write;
use glob::glob;

fn del(file: String){
    let rm = fs::remove_file(&file);
    if rm.is_err() {
        eprintln!("{}: {}", file, rm.err().unwrap());
    }
}

fn glob_dir(glob_val: String){
    for entry in glob(&glob_val).unwrap(){
        del(String::from(entry.unwrap().to_str().unwrap()));
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    for file in args{
        if file.contains("*") || file.contains("?") {
            glob_dir(file);
        }else {
            let metadata = fs::metadata(&file).unwrap();
            if metadata.is_file(){
                del(file);
            }else {
                // eprintln!("{}: is a directory", file);
                let mut line = String::new();
                print!("{} is a directory. Delete {}/*?(Y/N): ", file, file);
                io::stdout().flush();
                let _b1 = io::stdin().read_line(&mut line).unwrap();
                match line.as_str().trim() {
                    "y" | "Y" => glob_dir(file + "/*"),
                    "n" | "N" => println!("Not deleting {}/*", file),
                    _ => println!("Unknown value {}. Not deleting {}/*", line, file)
                }
            }
        }
       
    }
}
