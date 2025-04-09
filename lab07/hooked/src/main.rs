use std::rc::Rc;

#[derive(Copy, Debug, Clone)]
pub struct InstructionNumber(u32);

impl From<u32> for InstructionNumber {
    fn from(n: u32) -> Self {
        InstructionNumber(n)
    }
}

impl From<InstructionNumber> for u32 {
    fn from(n: InstructionNumber) -> Self {
        n.0
    }
}

#[derive(Clone)]
struct Hook {
    num_left: u32, // number of instructions left to execute before the callback is called
    callback: Rc<dyn Fn(&mut Cpu)>, // reference-counted pointer that allows for shared ownership of the enclosed value
                                    // note that it is a Fn closure, meaning it can be called multiple times
}

impl Hook {
    // TODO: implement the new method
    // function accepts any closure that matches the required signature and wraps it in an Rc
    // Accept any type F that implements Fn(&mut Cpu) and lives for the entire program ('static)
    fn new<F>(num_left: u32, callback: F) -> Self
    where
        F: Fn(&mut Cpu) + 'static,
    {
        Hook {
            num_left,
            callback: Rc::new(callback),
        }
    }

    // TODO: implement a call method
    fn call(&self, cpu: &mut Cpu) {
        // This calls the callback function stored in this Hook
        // The callback is a reference-counted function pointer (Rc<dyn Fn(&mut Cpu)>)
        // that takes a mutable reference to the CPU and performs some operation on it
        (self.callback)(cpu);
    }
}

enum Instruction<F: Fn(&Cpu) -> bool> {
    /// Do nothing
    Nop,
    /// Output the contents of our accumulator
    PrintAccumulator,
    /// JumpIfCondition
    /// InstructionNumber is 1 based
    JumpIfCondition(F, InstructionNumber),
    /// Add to the accumulator
    AddLiteral(u32),
    /// Subtract from the Accumulator
    SubLiteral(u32),
    /// Do an instruction
    /// N instructions in the future
    Callback(Hook),
    /// Exit
    Quit,
}

// You can, and should modify the Cpu struct
// So the CPU needs some way to keep track of hooks, including the hook that should be run
struct Cpu {
    current_instruction: InstructionNumber,
    accumulator: u32,
    hooks: Vec<Hook>,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            current_instruction: InstructionNumber(0),
            accumulator: 0,
            hooks: Vec::new(),
        }
    }

    fn run<F: Fn(&Cpu) -> bool>(&mut self, instructions: Vec<Instruction<F>>) {
        loop {
            let instruction = &instructions[self.current_instruction.0 as usize];
            match instruction {
                Instruction::Nop => {
                    println!("\t...no-op");
                }
                Instruction::PrintAccumulator => {
                    println!("\t...print accumulator");
                    println!("Accumulator: {}", self.accumulator);
                }
                Instruction::AddLiteral(n) => {
                    println!("\t...adding {}", n);
                    self.accumulator += *n;
                }
                Instruction::SubLiteral(n) => {
                    println!("\t...subtracting {}", n);
                    self.accumulator -= *n;
                }
                Instruction::Callback(hook) => {
                    println!("\t...callback instruction");
                    //TODO: implement this
                    self.hooks.push(hook.clone());
                }
                Instruction::JumpIfCondition(condition, n) => {
                    println!("\t...conditional jump");
                    if condition(self) {
                        self.current_instruction = ((u32::from(*n)) - 2).into();
                    }
                }
                Instruction::Quit => {
                    break;
                }
            }
            self.current_instruction.0 += 1;

            // You need to look through your list of hooks (which borrows self immutably)
            // For each hook that's ready, you need to call its callback (which requires borrowing self mutably)
            // Rust doesn't allow these two kinds of borrows to exist at the same time

            // Process hooks
            // After incrementing self.current_instruction.0
            // First: Decrement counters and identify hooks to execute
            let mut hooks_to_execute = Vec::new();
            let mut indices_to_remove = Vec::new();

            for (i, hook) in self.hooks.iter_mut().enumerate() {
                hook.num_left -= 1;
                if hook.num_left == 0 {
                    hooks_to_execute.push(hook.clone());
                    indices_to_remove.push(i);
                }
            }

            // Second: Execute the hooks (this uses mutable borrow of self)
            for hook in hooks_to_execute {
                hook.call(self);
            }

            // Third: Remove executed hooks (in reverse order to maintain indices)
            for &i in indices_to_remove.iter().rev() {
                self.hooks.remove(i);
            }

            // ALTERNATE:
            // // Make a copy of all hooks
            // let hooks_copy = self.hooks.clone();

            // self.hooks.clear(); // Remove all hooks since we'll re-add the ones that aren't ready

            // // Process each hook
            // for mut hook in hooks_copy {
            //     // Decrement the counter
            //     hook.num_left -= 1;

            //     if hook.num_left == 0 {
            //         // Execute the hook if it's ready
            //         hook.call(self);
            //     } else {
            //         // Put it back in the list if it's not ready
            //         self.hooks.push(hook);
            //     }
            // }
        }
    }
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    for arg in args {
        match arg.parse::<i32>() {
            Ok(1) => simple_test(),
            Ok(2) => simple_test_with_hook(),
            Ok(3) => complex_test(),
            Ok(_) | Err(_) => {
                panic!("Unknown test number: '{}'.", arg);
            }
        }
    }
}

fn simple_test() {
    // this test should always pass
    let mut cpu = Cpu::new();
    let instructions = vec![
        Instruction::Nop,
        Instruction::PrintAccumulator,
        Instruction::AddLiteral(1),
        Instruction::PrintAccumulator,
        Instruction::JumpIfCondition(|_| true, InstructionNumber(6)),
        Instruction::Quit,
    ];
    cpu.run(instructions);
}

fn simple_test_with_hook() {
    let mut cpu = Cpu::new();
    let instructions = vec![
        Instruction::Callback(Hook::new(2, |cpu: &mut Cpu| {
            cpu.accumulator += 6991;
        })),
        Instruction::Nop,
        Instruction::Nop,
        Instruction::JumpIfCondition(|_| true, 6.into()),
        Instruction::AddLiteral(1),
        Instruction::PrintAccumulator,
        Instruction::Quit,
    ];
    cpu.run(instructions);
}

fn complex_test() {
    // You should not need to touch any code in main
    let mut cpu = Cpu::new();
    let instructions = vec![
        Instruction::Nop,
        Instruction::PrintAccumulator,
        Instruction::AddLiteral(1),
        Instruction::Callback(Hook::new(2, |cpu: &mut Cpu| {
            cpu.accumulator += 6991;
        })),
        Instruction::Nop,
        Instruction::Nop,
        Instruction::JumpIfCondition(|cpu| cpu.accumulator <= 6991, 3.into()),
        Instruction::SubLiteral(1),
        Instruction::PrintAccumulator,
        Instruction::Quit,
    ];
    cpu.run(instructions);
}
