use proconio::input;

#[allow(dead_code)]
pub fn main() {
    input! {
        a: i32,
        b: i32,
    }

    println!("{}", f(a).max(f(b)));
}

fn f(mut n: i32) -> i32 {
    let mut res = 0;
    while n != 0 {
        res += n % 10;
        n /= 10;
    }
    res
}
