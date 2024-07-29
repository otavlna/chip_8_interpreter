#[derive(Debug, PartialEq)]
pub enum OpCode {
    ClearScreen,
    Display { vx: u8, vy: u8, n: u8 },

    Jump { address: u16 },
    Subroutine { address: u16 },
    Return,
    SetIndex { address: u16 },
    JumpPlusV0 { address: u16 },
    AssignRandom { vx: u8, value: u8 },
    AddToIndex { vx: u8 },
    SetIndexToSpriteLocation { vx: u8 },
    StoreBinaryCodedDecimal { vx: u8 },
    StoreToMemory { vx: u8 },
    LoadFromMemory { vx: u8 },

    SkipIfPressed { vx: u8 },
    SkipIfNotPressed { vx: u8 },
    AssignDelayTimerToVariable { vx: u8 },
    WaitForPressAndAssign { vx: u8 },
    SetDelayTimer { vx: u8 },
    SetSoundTimer { vx: u8 },

    SkipIfVariableEqualsValue { vx: u8, value: u8 },
    SkipIfVariableNotEqualsValue { vx: u8, value: u8 },
    SkipIfVariableEqualsVariable { vx: u8, vy: u8 },
    SkipIfVariableNotEqualsVariable { vx: u8, vy: u8 },

    SetValue { vx: u8, value: u8 },
    AddValue { vx: u8, value: u8 },

    AssignVariable { vx: u8, vy: u8 },
    BitwiseOr { vx: u8, vy: u8 },
    BitwiseAnd { vx: u8, vy: u8 },
    BitwiseXor { vx: u8, vy: u8 },
    Add { vx: u8, vy: u8 },
    Subtract { vx: u8, vy: u8 },
    BitShiftRight { vx: u8, vy: u8 },
    AssignDifference { vx: u8, vy: u8 },
    BitShiftLeft { vx: u8, vy: u8 },

    Invalid,
}
