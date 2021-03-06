use std;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

// Interpret brainfuck code
pub fn evaluate(program: String) {
    let tape_length = 30_000;

    let mut buffer = String::new();

    let mut tape: Vec<u8> = vec![0; tape_length];
    let mut ptr = 0;
    let mut is_looping = false;
    let mut loop_stack = vec![];
    let mut inner_loops = 0;

    let mut i = 0;

    while i < program.len() {
        let c = program.chars().nth(i).unwrap();

        if is_looping {
            if c == '[' {
                inner_loops += 1;
            }
            if c == ']' {
                if inner_loops == 0 {
                    is_looping = false;
                } else {
                    inner_loops -= 1;
                }
            }

            i += 1;
            continue;
        }

        match c {
            '+' => tape[ptr] += 1,
            '-' => tape[ptr] -= 1,
            '>' => {
                ptr += 1;
            }
            '<' => {
                ptr -= 1;
            }
            '.' => {
                print!("{}", tape[ptr] as char);
                io::stdout().flush();
            }
            ',' => {
                io::stdin().read_to_string(&mut buffer);
                tape[ptr] = buffer.chars().next().unwrap() as u8;
            }
            ']' => {
                if tape[ptr] != 0 {
                    i = loop_stack[loop_stack.len() - 1];
                } else {
                    loop_stack.pop();
                }
            }
            '[' => {
                if tape[ptr] == 0 {
                    is_looping = true;
                } else {
                    loop_stack.push(i);
                }
            }
            _ => {}
        }

        i += 1;
    }
}

// compile brainfuck code to x86
pub fn compile(program: String) {

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("temp.s")
        .unwrap();

    let mut matching_bracket;

    let mut num_brackets = 0;
    let mut loop_stack = vec![];
    let prologue = concat!(
        ".text\n",
        ".globl _main\n",
        "_main:\n",
        "  pushq %rbp\n",
        "  movq %rsp, %rbp\n",
        "  pushq %r12\n",        // store callee saved register
        "  subq $30008, %rsp\n", // allocate 30,008 B on stack, and realign
        "  leaq (%rsp), %rdi\n", // address of beginning of tape
        "  movl $0, %esi\n",     // fill with 0's
        "  movq $30000, %rdx\n", // length 30,000 B
        "  call _memset\n",      // memset
        "  movq %rsp, %r12\n",
        "\n",
    );

    file.write_all(prologue.as_bytes());

    let mut i = 0;
    while i < program.len() {
        let c = program.chars().nth(i).unwrap();

        match c {
            '+' => { file.write_all(b"  incb (%r12)\n"); },
            '-' => { file.write_all(b"  decb (%r12)\n"); },
            '>' => { file.write_all(b"  inc %r12\n"); },
            '<' => { file.write_all(b"  dec %r12\n"); },
            '.' => {
                // move byte to double word and zero upper bits since putchar takes an int.
                file.write_all(b"  movzbl (%r12), %edi\n");
                file.write_all(b"  call _putchar\n");
            }
            ',' => {
                file.write_all(b"  call _getchar\n");
                file.write_all(b"  movb %al, (%r12)\n");
            }
            ']' => {
                matching_bracket = loop_stack.pop().unwrap(); // Should check if stack underflow
                file.write_all(b"  cmpb $0, (%r12)\n");
                file.write_all(format!("  jne bracket_{}_start\n", matching_bracket).as_bytes());
                file.write_all(format!("bracket_{}_end:\n", matching_bracket).as_bytes());
            }
            '[' => {
                loop_stack.push(num_brackets); // Should check if too much nesting
                file.write_all(b"  cmpb $0, (%r12)\n");
                file.write_all(format!("  je bracket_{}_end\n", num_brackets).as_bytes());
                file.write_all(format!("bracket_{}_start:\n", num_brackets).as_bytes());
                num_brackets += 1;
            }
            _ => {}
        }

        i += 1;
    }

    let epilogue = concat!(
        "  addq $30008, %rsp\n", // clean up tape from stack.
        "  popq %r12\n",         // restore callee saved register
        "  popq %rbp\n",
        "  ret\n"
    );
    file.write_all(epilogue.as_bytes());
}
