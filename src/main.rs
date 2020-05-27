mod euler1;
mod euler2;
mod euler3;
mod euler4;
mod euler5;
mod euler6;
mod euler7;
mod euler8;
mod euler9;
pub use euler1::euler1::{solve as solve_euler1};
// fn project_euler_2() -> i32 {
//     #![feature(iter_unfold)]
//     let fib = std::iter::unfold(0, |c| {
//         Some(1)
//     });
//     return (0..40000000).filter(|&x| rem(x, 5) == 0 || rem(x, 3) == 0).sum();
// }

fn main() {
    println!("Hello, world! {}", i32::pow(5, 2));
}
