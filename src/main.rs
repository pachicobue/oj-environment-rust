use num::range_step;
fn main() {
    for i in range_step(10, 0, -2) {
        println!("{}", i);
    }
}
