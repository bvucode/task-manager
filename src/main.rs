use std::io::stdin;
fn main() {
    let mut v = vec![];
    println!("\ntask manager\ntop tasks is most important and urgent\n add - text and enter \n show - show task \n take - take task \n exit - exit \n");
    loop {
        let mut line = String::new();
        println!(">>> ");
        stdin().read_line(&mut line).unwrap();
        if line == "show\n" {
            println!("\nto do:");
            for (x, i) in v.iter().enumerate() {
                println!("{:?}) {:?}", x + 1, i);
            }
        }
        else if line == "take\n" {
            println!("{:?} is done.", v[0]);
            v.remove(0);
        }
        else if line == "exit\n" {
            break;
        }
        else {
            v.push(line);
        }
    }
}