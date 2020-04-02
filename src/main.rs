
fn rem(a: i32, b:i32) -> i32 {
    return a - (b * (a/b))
}

fn project_euler_1() -> i32 {

    return (0..1000).filter(|&x| rem(x, 5) == 0 || rem(x, 3) == 0).sum();
}

fn main() {
    let a = project_euler_1();
    println!("Hello, world! {:?}", a);
}
