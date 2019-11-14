
function compile(program) {

  let tape = Array(10).fill(0);
  let ptr = 0;
  let isLooping = false;
  let loopStack = [];
  let innerLoops = 0;

  for (let i = 0; i < program.length; i++) {

    const char = program[i];

    if (isLooping) {
      if (char === "[")  innerLoops++;
      if (char === "]") {
        if (innerLoops === 0) isLooping = false;
        else innerLoops --;
      }

      continue
    }
    
    switch (char) {
      case "+":
        tape[ptr]++;
        break;
      case "-":
        tape[ptr]--;
        break;
      case ">":
        ptr++;
        tape[ptr] = tape[ptr] || 0
        break;
      case "<":
        ptr--;
        tape[ptr] = tape[ptr] || 0
        break;
      case ".":
        console.log(String.fromCharCode(tape[ptr]));
        break;
      case ",":
        tape[ptr] = prompt()[0].charCodeAt()
        break;
      case "]":
        tape[ptr] !== 0 ? i = loopStack[loopStack.length-1] : loopStack.pop()
        break;
      case "[":
        tape[ptr] === 0 ? isLooping = true : loopStack.push(i)
        break;
      default:
        break;
    }
  }
}

console.log(compile("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."))
