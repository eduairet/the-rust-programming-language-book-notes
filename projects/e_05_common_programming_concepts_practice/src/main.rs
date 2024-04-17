fn main() {
    let brisket_temp_f = 225.0;
    let brisket_temp_c = fahrenheit_to_celsius(brisket_temp_f);
    println!(
        "The brisket is {}°F or {}°C",
        brisket_temp_f, brisket_temp_c
    );

    let n = 10;
    let nth_fib = nth_fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, nth_fib);

    christmas_carol(12);
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn nth_fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    return a;
}

fn christmas_carol(n: u32) {
    for i in 0..n {
        println!("{} days of Christmas, my true love gave to me", i + 1);
    }
}
