/// This is a sample document for the function `get_coffee_type`.
///
/// `get_coffee_type` gets the type of coffee based on certain params.
///
/// # Examples
/// ```
/// let steamed_milk = 4;
/// let foamed_milk = 1;
/// let coffee_type = cargo_doc_blog::get_coffee_type(steamed_milk, foamed_milk);
///
/// assert_eq!("Latte", coffee_type);
/// ```


pub fn get_coffee_type(steamed_milk: u8, foamed_milk: u8) -> String {
    // Get the type of coffee as per the arguments
    if steamed_milk > 0 {
        if steamed_milk > 2 {
            "Latte".to_string()
        }
        else {
            if foamed_milk > 0 {
                "Cappuccino".to_string()
            }
            else {
                "Flat White".to_string()
            }
        }
    }
    else {
        "Espresso".to_string() // See I did not use an 'X' there ;)
    }
}
