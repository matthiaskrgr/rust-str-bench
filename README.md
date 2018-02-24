````
cargo bench

    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/str_bench-4f9f5a68c20995f8

running 9 tests
test tests::bench_const_str_var            ... bench:      78,694 ns/iter (+/- 25,683)
test tests::bench_inline                   ... bench:      46,300 ns/iter (+/- 2,643)
test tests::bench_inline_format            ... bench:      78,796 ns/iter (+/- 10,303)
test tests::bench_inline_format_ref        ... bench:      78,505 ns/iter (+/- 7,720)
test tests::bench_inline_format_stringfrom ... bench:      96,864 ns/iter (+/- 5,593)
test tests::bench_inline_format_tostring   ... bench:      96,050 ns/iter (+/- 3,951)
test tests::bench_let_str_var              ... bench:      79,855 ns/iter (+/- 3,014)
test tests::bench_let_stringfrom_var       ... bench:      96,284 ns/iter (+/- 3,811)
test tests::bench_let_tostring_var         ... bench:      96,146 ns/iter (+/- 2,826)

test result: ok. 0 passed; 0 failed; 0 ignored; 9 measured; 0 filtered out

````
