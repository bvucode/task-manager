use std::io;
use std::io::stdin;
use std::io::Write;

fn main() {
    let mut v = vec![];
    println!("\ntask manager\ntop tasks is most important and urgent\n add - text and enter \n show - show task \n take - take task \n exit - exit \n");
    loop {
        print!("enter: ");
        io::stdout().flush();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        if line.trim_end() == "show" {
            println!("\nto do:");
            for (x, i) in v.iter().enumerate() {
                println!("{:?}) {:?}", x + 1, i);
            }
        }
        else if line.trim_end() == "take" {
            println!("{:?} is done.", v[0]);
            v.remove(0);
        }
        else if line.trim_end() == "exit" {
            break;
        }
        else {
            v.push(line);
        }
    }
}