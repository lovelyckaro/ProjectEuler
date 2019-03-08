# Project Euler
Smallest number divisible by all numbers from 1 to N

## Method:
1. Find the prime factorization of every number between 2 and N<br>
   12 would become `2^2 * 3`, 8 would become `2^3` 
2. Combine prime factors, merge using biggest exponent<br>
   Merging `2^2 * 3` and `2^3` would be `2^3 * 3`
3. (Optional) Calculate product of all factors

## Compilation:
Compile using cargo, no external libraries are needed<br>
```
`cargo build --release`
```
## Usage:
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