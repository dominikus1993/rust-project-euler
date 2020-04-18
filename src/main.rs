mod euler1;
pub use euler1::euler1::{solve as solve_euler1};
// fn project_euler_2() -> i32 {
//     #![feature(iter_unfold)]
//     let fib = std::iter::unfold(0, |c| {
//         Some(1)
//     });
//     return (0..40000000).filter(|&x| rem(x, 5) == 0 || rem(x, 3) == 0).sum();
// }

fn main() {
    let a = solve_euler1();
    println!("Hello, world! {:?}", a);
}
