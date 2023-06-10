use std::io::stdin;
fn main() {
    let mut v:Vec<_> = vec![];
    println!("Hello, world!\ntask manager\ntop tasks is most important and urgent\n add - text and enter \n show - show task \n take - take task \n exit - exit \n");
    let mut line = String::new();
    println!("> ");
    let t1 = stdin().read_line(&mut line).unwrap();
    corefunction(&mut line, &mut v);
    fn corefunction<T>(arg:&mut String, v:&mut Vec<T>) {
        if arg == "show" {
            println!("\nto do:");
            for i in v.len() {
                println!("{}) {}", i + 1, v[i]);
            }
        }
        else if arg == "take" {
            println!("{} is done.", v[0]);
        }
        else {
            v.push(arg);
        }
    }
}
