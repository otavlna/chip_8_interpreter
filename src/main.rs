use core::time;
use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
    usize,
};

use op_codes::OpCode;

mod op_codes;
mod parser;

fn render(display: &[bool; 64 * 32]) {
    let mut output = String::new();
    for i in 0..32 {
        for j in 0..64 {
            output += format!(
                "{}",
                if display[i * 64 + j] == true {
                    // "\u{25A0}"
                    "\u{2588}"
                } else {
                    " "
                }
            )
            .as_str();
        }
        output += format!("\n").as_str();
    }

    print!("\x1B[2J\x1B[1;1H");
    print!("{}", output);
}

fn main() {
    let mut memory: [u8; 4096] = [0; 4096];
    let mut display: [bool; 64 * 32] = [false; 64 * 32];
    let mut pc: u16 = 0x200;
    let mut index: u16 = 0;
    let mut stack: Vec<u16> = Vec::new();
    let delay_timer = Arc::new(Mutex::new(0 as u8));
    let sound_timer = Arc::new(Mutex::new(0 as u8));
    let mut registers: [u8; 16] = [0; 16];

    let delay_timer_clone = Arc::clone(&delay_timer);
    let sound_timer_clone = Arc::clone(&sound_timer);
    thread::spawn(move || loop {
        sleep(Duration::from_millis(1000 / 60));
        {
            let mut delay_timer = delay_timer_clone.lock().unwrap();
            if *delay_timer > 0 {
                *delay_timer -= 1;
            }
        }
        {
            let mut sound_timer = sound_timer_clone.lock().unwrap();
            if *sound_timer > 0 {
                *sound_timer -= 1;
                print!("\x07");
            }
        }
    });

    let _font_memory = &mut memory[0x50..=0x9F].copy_from_slice(&[
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ]);

    let rom = std::fs::read("./roms/airplane.ch8").unwrap();
    let _program_memory = &mut memory[0x200..0x200 + rom.len()].copy_from_slice(&rom);

    loop {
        let instruction = u16::from_be_bytes([memory[pc as usize], memory[pc as usize + 1]]);
        let op_code = parser::parse_op_code(instruction);
        pc += 2;

        println!("{:?}", op_code);

        match op_code {
            OpCode::ClearScreen => display.fill(false),
            OpCode::Jump { address } => pc = address,
            OpCode::SetValue { vx, value } => registers[vx as usize] = value,
            OpCode::AddValue { vx, value } => {
                registers[vx as usize] = registers[vx as usize].wrapping_add(value)
            }
            OpCode::SetIndex { address } => index = address,
            OpCode::Display { vx, vy, n } => {
                let mut x = registers[vx as usize];
                let mut y = registers[vy as usize];
                x %= 64;
                y %= 32;
                registers[0xF as usize] = 0;
                for i in 0..n {
                    if y + i > 31 {
                        break;
                    }
                    let row = memory[index as usize + i as usize];
                    for ii in 0..8 {
                        if x + ii > 63 {
                            break;
                        }
                        let color = (row >> (7 - ii)) & 1;
                        let display_index = (x + ii) as usize + (y + i) as usize * 64;
                        if color == 1 {
                            if display[display_index] == true {
                                display[display_index] = false;
                                registers[0xF] = 1;
                            } else {
                                display[display_index] = true;
                            }
                        }
                    }
                }
            }
            OpCode::Subroutine { address } => {
                stack.push(pc);
                pc = address;
            }
            OpCode::Return => pc = stack.pop().unwrap(),

            OpCode::SkipIfVariableEqualsValue { vx, value } => {
                if registers[vx as usize] == value {
                    pc += 2
                }
            }
            OpCode::SkipIfVariableNotEqualsValue { vx, value } => {
                if registers[vx as usize] != value {
                    pc += 2
                }
            }
            OpCode::SkipIfVariableEqualsVariable { vx, vy } => {
                if registers[vx as usize] == registers[vy as usize] {
                    pc += 2
                }
            }
            OpCode::SkipIfVariableNotEqualsVariable { vx, vy } => {
                if registers[vx as usize] != registers[vy as usize] {
                    pc += 2
                }
            }

            OpCode::AssignVariable { vx, vy } => registers[vx as usize] = registers[vy as usize],
            OpCode::BitwiseOr { vx, vy } => registers[vx as usize] |= registers[vy as usize],
            OpCode::BitwiseAnd { vx, vy } => registers[vx as usize] &= registers[vy as usize],
            OpCode::BitwiseXor { vx, vy } => registers[vx as usize] ^= registers[vy as usize],
            OpCode::Add { vx, vy } => {
                let (result, overflowed) =
                    registers[vx as usize].overflowing_add(registers[vy as usize]);
                registers[vx as usize] = result;
                registers[0xF as usize] = overflowed as u8;
            }
            OpCode::Subtract { vx, vy } => {
                let (result, overflowed) =
                    registers[vx as usize].overflowing_sub(registers[vy as usize]);
                registers[vx as usize] = result;
                registers[0xF as usize] = !overflowed as u8;
            }
            OpCode::BitShiftRight { vx, vy } => {
                registers[0xF] = registers[vx as usize] & 0x1;
                registers[vx as usize] >>= 1;
            }
            OpCode::AssignDifference { vx, vy } => {
                let (result, overflowed) =
                    registers[vy as usize].overflowing_sub(registers[vx as usize]);
                registers[vx as usize] = result;
                registers[0xF as usize] = !overflowed as u8;
            }
            OpCode::BitShiftLeft { vx, vy } => {
                registers[0xF] = registers[vx as usize] & 0x1;
                registers[vx as usize] <<= 1;
            }

            OpCode::StoreToMemory { vx } => {
                for i in 0..=vx {
                    memory[(index + i as u16) as usize] = registers[i as usize];
                }
            }
            OpCode::LoadFromMemory { vx } => {
                for i in 0..=vx {
                    registers[i as usize] = memory[(index + i as u16) as usize];
                }
            }
            OpCode::StoreBinaryCodedDecimal { vx } => {
                let hundreads = registers[vx as usize] / 100;
                let tens = (registers[vx as usize] - hundreads * 100) / 10;
                let ones = (registers[vx as usize] - hundreads * 100 - tens * 10) / 1;

                memory[index as usize] = hundreads;
                memory[(index + 1) as usize] = tens;
                memory[(index + 2) as usize] = ones;
            }
            OpCode::AddToIndex { vx } => index += registers[vx as usize] as u16,

            OpCode::SetIndexToSpriteLocation { vx } => {
                index = 0x50 + 5 * registers[vx as usize] as u16
            }
            OpCode::AssignDelayTimerToVariable { vx } => {
                registers[vx as usize] = *delay_timer.lock().unwrap()
            }
            OpCode::SetDelayTimer { vx } => *delay_timer.lock().unwrap() = registers[vx as usize],
            OpCode::SetSoundTimer { vx } => *sound_timer.lock().unwrap() = registers[vx as usize],

            // TODO
            OpCode::SkipIfPressed { vx } => (),
            OpCode::SkipIfNotPressed { vx } => (),

            _ => break,
        }

        render(&display);

        std::thread::sleep(time::Duration::from_millis(1));
    }
}
