#![feature(test)]

extern crate test;

fn main() {

}

#[cfg(test)]
mod tests {
    use test::Bencher;

    pub fn let_str_var() {
        let string: &str = "Rust1";
        println!("{}", string);
    }

    pub fn const_str_var() {
        const STRING: &str = "Rust2";
        println!("{}", STRING);
    }

    pub fn let_stringfrom_var() {
        let string: String = String::from("Rust3");
        println!("{}", string);
    }

    pub fn let_tostring_var() {
        let string: String = "Rust4".to_string();
        println!("{}", string);
    }

    pub fn inline_format() {
        println!("{}", "Rust6");
    }

    pub fn inline() {
        println!("Rust7");
    }

    pub fn inline_format_stringfrom() {
        println!("{}", String::from("Rust8"));
    }

    pub fn inline_format_tostring() {
        println!("{}", "Rust9".to_string());
    }

    pub fn inline_format_ref() {
        println!("{}", &"Rust0");
    }

	const ITERATIONS: i32 = 750;

    #[bench]
    fn bench_let_str_var(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                let_str_var();
            }
        });
    }

    #[bench]
    fn bench_const_str_var(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                const_str_var();
            }
        });
    }

    #[bench]
    fn bench_let_stringfrom_var(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                let_stringfrom_var();
            }
        });
    }

    #[bench]
    fn bench_let_tostring_var(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                let_tostring_var();
            }
        });
    }

    #[bench]
    fn bench_inline_format(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                inline_format();
            }
        });
    }

    #[bench]
    fn bench_inline(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                inline();
            }
        });
    }

    #[bench]
    fn bench_inline_format_stringfrom(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                inline_format_stringfrom();
            }
        });
    }

    #[bench]
    fn bench_inline_format_tostring(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                inline_format_tostring();
            }
        });
    }

    #[bench]
    fn bench_inline_format_ref(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..ITERATIONS {
                inline_format_ref();
            }
        });
    }

}
