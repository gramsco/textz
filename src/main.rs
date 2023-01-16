use std::fs;

fn main() {
    const TXT_FILE: &str = "src/yolo.txt";
    let x = fs::read_to_string(TXT_FILE).expect("aie");
    for (width, line) in x.split("\n").enumerate() {
        print!("{:<width$}", "");
        for (_, char) in line.chars().enumerate() {
            let f = format!("{:<2}", char);
            print!("{}", f);
        }
        println!()
    }
}
