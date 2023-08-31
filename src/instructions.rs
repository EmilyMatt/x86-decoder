use crate::registers::Register;

const INSTRUCTION_CLEAR: u8 = 0b11111100;
const DIRECTION: u8 = 0b00000010;
const IS_WIDE: u8 = 0b00000001;

pub enum EightySixInstruction {
    MOV {
        destination: Register,
        source: Register,
    },
}

impl ToString for EightySixInstruction {
    fn to_string(&self) -> String {
        match self {
            EightySixInstruction::MOV {
                destination,
                source,
            } => {
                format!("MOV {},{}", destination, source)
            }
        }
    }
}

#[repr(u8)]
pub enum EightySixInstructionType {
    MOV = 0b10001000,
}

impl TryFrom<u8> for EightySixInstructionType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & INSTRUCTION_CLEAR {
            cleaned_value if EightySixInstructionType::MOV as u8 == cleaned_value => Ok(Self::MOV),
            _ => Err("Unknown instruction"),
        }
    }
}

pub fn get_direction_and_wide(value: u8) -> (bool, bool) {
    (value & DIRECTION == DIRECTION, value & IS_WIDE == IS_WIDE)
}
