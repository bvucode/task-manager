use std::io::stdin;
fn main() {
    let mut v = vec![];
    println!("\ntask manager\ntop tasks is most important and urgent\n add - text and enter \n show - show task \n take - take task \n exit - exit \n");
    let mut line = String::new();
    println!("> ");
    let t1 = stdin().read_line(&mut line).unwrap();
    if line == "show" {
        println!("\nto do:");
        for i in v {
            println!("{:?}", i);
        }
    }
    else if line == "take" {
        println!("{:?} is done.", v[0]);
        v.pop();
    }
    else {
        v.push(line);
    }
}
