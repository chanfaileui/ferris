use serde::Deserialize;
use std::collections::VecDeque;
use std::io;

#[derive(Debug, Deserialize)]
enum Instruction {
    Set(i32),
    Left,
    Right,
    Reset,
}

#[derive(Debug)]
struct Light {
    left: Option<Box<Light>>,
    right: Option<Box<Light>>,
    brightness: i32,
}

fn get_instructions_from_stdin() -> VecDeque<Instruction> {
    let mut instructions = String::new();
    io::stdin().read_line(&mut instructions).unwrap();
    ron::from_str(&instructions).unwrap()
}

fn sum_and_count(light: &Light) -> (i32, i32) {
    let left = if let Some(left) = &light.left {
        sum_and_count(left)
    } else {
        (0, 0)
    };
    let right = if let Some(right) = &light.right {
        sum_and_count(right)
    } else {
        (0, 0)
    };
    (left.0 + right.0 + light.brightness, left.1 + right.1 + 1)
}

fn main() {
    let instructions = get_instructions_from_stdin();
    let mut light = Light {
        left: None,
        right: None,
        brightness: 0,
    };
    
    // println!("{instructions:?}");

    // you're holding the root node
    // you need some sort of current pointer
    // you keep moving the pointer

    let mut curr = &mut light;
    for instruction in instructions {
        match instruction {
            Instruction::Set(value) => {
                curr.brightness = value;
            }
            Instruction::Left => {
                match curr.left {
                    None => {
                        curr.left = Some(Box::new(Light {
                            left: None,
                            right: None,
                            brightness: 0,
                        }));
                        curr = curr.left.as_mut().unwrap();
                        // or
                        // curr = curr.left.as_mut().and_then(|boxed| Some(boxed.as_mut()));
                    }
                    Some(ref mut left_light) => {
                        curr = left_light;
                    }
                }
            }
            Instruction::Right => match curr.right {
                None => {
                    curr.right = Some(Box::new(Light {
                        left: None,
                        right: None,
                        brightness: 0,
                    }));
                    curr = curr.right.as_mut().unwrap();
                }
                Some(ref mut right_light) => {
                    curr = right_light;
                }
            },
            Instruction::Reset => {
                curr = &mut light;
            }
        }
    }

    let (sum, count) = sum_and_count(&light);
    let avg = sum / count;
    println!("{avg}");
    // println!("{light:?}");
}
