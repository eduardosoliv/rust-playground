use exprflow::flowcontrol::forloops;
use exprflow::flowcontrol::ifelse;
use exprflow::flowcontrol::loops;
use exprflow::flowcontrol::whiles;
use exprflow::lettomatch::iflet;
use exprflow::lettomatch::letelse;
use exprflow::lettomatch::whilelet;
use exprflow::patternmatching::basicmatch;
use exprflow::patternmatching::binding;
use exprflow::patternmatching::destruct;
use exprflow::patternmatching::guard;
use exprflow::patternmatching::pointersref;
use exprflow::patternmatching::structs;

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
    println!("------");

    basicmatch::match_number();
    println!("---");
    basicmatch::match_bool_assign();
    println!("------");

    destruct::destruct_tuples();
    println!("---");
    destruct::destruct_array();
    println!("---");
    destruct::destruct_enums();
    println!("------");

    pointersref::pointers_ref();
    println!("------");

    structs::destruct_struct();
    println!("---");
    structs::destruct_struct_without_match();
    println!("------");

    guard::guard_example();
    println!("---");
    guard::guard_number();
    println!("------");

    binding::binding_example();
    println!("---");
    binding::binding_enum();

    println!("------");
    iflet::awkward_match(Some(7));
    iflet::awkward_match(None);
    println!("---");
    iflet::simple_let(Some(7));
    iflet::simple_let(None);
    println!("---");
    iflet::let_else(Some(10));
    iflet::let_else(None);
    println!("---");
    iflet::let_if_else();
    println!("---");
    iflet::if_let_match_enum();
    println!("---");

    assert_eq!(letelse::get_count_item("3 chairs"), (3, "chairs"));

    whilelet::while_let_example(Some(5));
}
