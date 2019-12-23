
# Brainfuck

------------------

A brainfuck interpreter

```rust
use br4infuck::compile;

fn main() {
    compile("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.".to_string());
}⏎                                                                                                                                                 

// Prints: "Hello World"
```
