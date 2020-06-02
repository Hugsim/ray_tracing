use std::io::{self, Write};

mod vec3;

fn println(text: &str) {
    print(&format!("{}\n", text));
}

fn print(text: &str) {
    io::stdout().write_all(text.as_bytes()).expect("Failed writing to stdout!?");
}

fn print_stderr(text: &str) {
    io::stderr().write_all(text.as_bytes()).expect("Failed writing to stderr!?");
}

fn main() {
    const IMAGE_HEIGHT: usize = 256;
    const IMAGE_WIDTH: usize = 256;

    println("P3");
    println(&format!("{} {}", IMAGE_HEIGHT, IMAGE_WIDTH));
    println("255");

    for i in 0..256 {
        print_stderr(&format!("Scanlines remaining: {}\n", 256 - i));
        for j in 0..256 {
            let r = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let g = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let b = 0.25 as f64;
            
            let r = r * 255.999;
            let g = g * 255.999;
            let b = b * 255.999;

            println(&format!("{} {} {}", r.floor() as i64, g.floor() as i64, b.floor() as i64));
        }
    }
    print_stderr("Done!");
}
