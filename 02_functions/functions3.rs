fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(5); 
    //in order to fix we need to add an argument that is unsigened meaning its positive  and it has to be within the range
}
