use exprflow::flowcontrol::ifelse;
use exprflow::flowcontrol::loops::{loop_assign, loop_nesting, simple_loop};
fn main() {
    ifelse::simple_example(5);
    ifelse::simple_example(-1);
    ifelse::simple_example(0);
    println!("---");
    ifelse::assign_example(5);
    ifelse::assign_example(15);
    println!("------");
    simple_loop();
    println!("---");
    loop_nesting();
    println!("---");
    loop_assign();
}
