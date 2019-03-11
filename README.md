# Project Euler Problem 5
Smallest number divisible by all numbers from 1 to N

## Method:
1. Find the prime factorization of every number between 2 and N<br>
   12 would become `2^2 * 3`, 8 would become `2^3` 
2. Combine prime factors, merge using biggest exponent<br>
   Merging `2^2 * 3` and `2^3` would be `2^3 * 3`
3. (Optional) Calculate product of all factors

## Building:
The entire program is written in Rust, [read more about Rust here](https://www.rust-lang.org/).<br>
Compile using cargo, num-bigint, and num-traits are dependencies<br>
```
`cargo build --release`
```
## Using:
In CLI, run:<br>
```
cargo run N
```
or
```
target/release/ProjectEuler N
```
where N is upper bound. To calculate number, add eval to end
```
cargo run 20 eval
```