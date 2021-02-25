use std::fs;
use std::env;
use glob::glob;

fn del(file: String){
    let rm = fs::remove_file(&file);
    if rm.is_err() {
        eprintln!("{}: {}", file, rm.err().unwrap());
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    for file in args{
        if file.contains("*") {
            for entry in glob(&file).unwrap(){
                del(String::from(entry.unwrap().to_str().unwrap()));
            }
        }else {
            del(file);
        }
       
    }
}
