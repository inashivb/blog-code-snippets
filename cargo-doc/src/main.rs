use cargo_doc_test::get_coffee_type;


fn main() {
    let foamed_milk: u8 = 0;
    let steamed_milk = 2;
    println!("{}", get_coffee_type(steamed_milk, foamed_milk));
}
