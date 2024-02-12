fn main() {
    let y = 6;


    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(10, 'g');
}
fn another_function(x: i32){
    println!("Another function");

    println!("the value of x is: {x}");

}

fn print_labeled_measurement(value: i32, uint_label: char){
    println!("The measurement is: {value}{uint_label}");
}