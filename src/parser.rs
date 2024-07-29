use crate::op_codes::OpCode;

pub fn parse_op_code(op_code: u16) -> OpCode {
    let nnn = op_code & 0x0FFF;
    let nn: u8 = (op_code & 0x00FF).try_into().unwrap();
    let n: u8 = (op_code & 0x000F).try_into().unwrap();
    let x: u8 = ((op_code & 0x0F00) >> 8).try_into().unwrap();
    let y: u8 = ((op_code & 0x00F0) >> 4).try_into().unwrap();

    match op_code {
        0x00E0 => OpCode::ClearScreen,
        0xD000..=0xDFFF => OpCode::Display { vx: x, vy: y, n: n },

        0x1000..=0x1FFF => OpCode::Jump { address: nnn },
        0x00EE => OpCode::Return,
        0x2000..=0x2FFF => OpCode::Subroutine { address: nnn },
        0xA000..=0xAFFF => OpCode::SetIndex { address: nnn },
        0xB000..=0xBFFF => OpCode::JumpPlusV0 { address: nnn },
        0xC000..=0xCFFF => OpCode::AssignRandom { vx: x, value: nn },

        0xE000..=0xEFFF => match nn {
            0x9E => OpCode::SkipIfPressed { vx: x },
            0xA1 => OpCode::SkipIfNotPressed { vx: x },
            _ => OpCode::Invalid,
        },
        0xF000..=0xFFFF => match nn {
            0x07 => OpCode::AssignDelayTimerToVariable { vx: x },
            0x0A => OpCode::WaitForPressAndAssign { vx: x },
            0x15 => OpCode::SetDelayTimer { vx: x },
            0x18 => OpCode::SetSoundTimer { vx: x },
            0x1E => OpCode::AddToIndex { vx: x },
            0x29 => OpCode::SetIndexToSpriteLocation { vx: x },
            0x33 => OpCode::StoreBinaryCodedDecimal { vx: x },
            0x55 => OpCode::StoreToMemory { vx: x },
            0x65 => OpCode::LoadFromMemory { vx: x },
            _ => OpCode::Invalid,
        },

        0x3000..=0x3FFF => OpCode::SkipIfVariableEqualsValue { vx: x, value: nn },
        0x4000..=0x4FFF => OpCode::SkipIfVariableNotEqualsValue { vx: x, value: nn },
        0x5000..=0x5FFF => OpCode::SkipIfVariableEqualsVariable { vx: x, vy: y },
        0x9000..=0x9FFF => OpCode::SkipIfVariableNotEqualsVariable { vx: x, vy: y },

        0x6000..=0x6FFF => OpCode::SetValue { vx: x, value: nn },
        0x7000..=0x7FFF => OpCode::AddValue { vx: x, value: nn },

        0x8000..=0x8FFF => match n {
            0x0 => OpCode::AssignVariable { vx: x, vy: y },
            0x1 => OpCode::BitwiseOr { vx: x, vy: y },
            0x2 => OpCode::BitwiseAnd { vx: x, vy: y },
            0x3 => OpCode::BitwiseXor { vx: x, vy: y },
            0x4 => OpCode::Add { vx: x, vy: y },
            0x5 => OpCode::Subtract { vx: x, vy: y },
            0x6 => OpCode::BitShiftRight { vx: x, vy: y },
            0x7 => OpCode::AssignDifference { vx: x, vy: y },
            0xE => OpCode::BitShiftLeft { vx: x, vy: y },
            _ => OpCode::Invalid,
        },

        _ => OpCode::Invalid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_clear_screen() {
        assert_eq!(parse_op_code(0x00E0), OpCode::ClearScreen);
    }

    #[test]
    fn can_parse_jump() {
        assert_eq!(parse_op_code(0x1567), OpCode::Jump { address: 0x567 });
    }

    #[test]
    fn can_parse_skip_if_variable_x_equals_value() {
        assert_eq!(
            parse_op_code(0x3155),
            OpCode::SkipIfVariableEqualsValue {
                vx: 0x1,
                value: 0x55
            }
        )
    }

    #[test]
    fn can_parse_add_value() {
        assert_eq!(
            parse_op_code(0x7A11),
            OpCode::AddValue {
                vx: 0xA,
                value: 0x11
            }
        )
    }

    #[test]
    fn can_parse_subtract_variables() {
        assert_eq!(parse_op_code(0x8015), OpCode::Subtract { vx: 0x0, vy: 0x1 })
    }

    #[test]
    fn can_parse_invalid() {
        assert_eq!(parse_op_code(0xFFFF), OpCode::Invalid)
    }
}
