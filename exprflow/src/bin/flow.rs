use exprflow::flowcontrol::forloops;
use exprflow::flowcontrol::ifelse;
use exprflow::flowcontrol::loops;
use exprflow::flowcontrol::whiles;

fn main() {
    ifelse::simple_example(5);
    ifelse::simple_example(-1);
    ifelse::simple_example(0);
    println!("---");
    ifelse::assign_example(5);
    ifelse::assign_example(15);
    println!("------");
    loops::simple_loop();
    println!("---");
    loops::loop_nesting();
    println!("---");
    loops::loop_assign();
    println!("------");
    whiles::fizzbuzz();
    println!("------");
    forloops::fizzbuzz_forloop();
    println!("---");
    forloops::fizzbuzz_forloop_inclusive();
    println!("---");
    forloops::for_iter();
    println!("---");
    forloops::for_into_iter();
    println!("---");
    forloops::for_iter_mut();
}
