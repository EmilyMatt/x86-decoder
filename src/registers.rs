use std::{
    fmt::{Display, Formatter},
    io,
};

const REGISTER_CLEAR: u8 = 0b00000011;
const IS_HIGH: u8 = 0b00000100;

#[repr(u8)]
pub enum Register {
    AL,
    AX,

    CL,
    CX,

    DL,
    DX,

    BL,
    BX,

    AH,
    SP,

    CH,
    BP,

    DH,
    SI,

    BH,
    DI,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Register::AL => "AL",
            Register::AX => "AX",

            Register::CL => "CL",
            Register::CX => "CX",

            Register::DL => "DL",
            Register::DX => "DX",

            Register::BL => "BL",
            Register::BX => "BX",

            Register::AH => "AH",
            Register::SP => "SP",

            Register::CH => "CH",
            Register::BP => "BP",

            Register::DH => "DH",
            Register::SI => "SI",

            Register::BH => "BH",
            Register::DI => "DI",
        })
    }
}

#[repr(u8)]
enum InternalRegisterName {
    A = 0b00000000,
    C = 0b00000001,
    D = 0b00000010,
    B = 0b00000011,
}

impl TryFrom<u8> for InternalRegisterName {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & REGISTER_CLEAR {
            value if Self::A as u8 == value => Ok(Self::A),
            value if Self::C as u8 == value => Ok(Self::C),
            value if Self::D as u8 == value => Ok(Self::D),
            value if Self::B as u8 == value => Ok(Self::B),
            _ => Err("Unknown register"),
        }
    }
}

impl Display for InternalRegisterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            InternalRegisterName::A => "A",
            InternalRegisterName::C => "C",
            InternalRegisterName::D => "D",
            InternalRegisterName::B => "B",
        })
    }
}

fn get_register_from_annotation(
    register_annotation: (InternalRegisterName, bool),
    is_wide: bool,
) -> Register {
    match (register_annotation, is_wide) {
        ((InternalRegisterName::A, false), false) => Register::AL,
        ((InternalRegisterName::A, false), true) => Register::AX,
        ((InternalRegisterName::A, true), false) => Register::AH,
        ((InternalRegisterName::A, true), true) => Register::SP,

        ((InternalRegisterName::C, false), false) => Register::CL,
        ((InternalRegisterName::C, false), true) => Register::CX,
        ((InternalRegisterName::C, true), false) => Register::CH,
        ((InternalRegisterName::C, true), true) => Register::BP,

        ((InternalRegisterName::D, false), false) => Register::DL,
        ((InternalRegisterName::D, false), true) => Register::DX,
        ((InternalRegisterName::D, true), false) => Register::DH,
        ((InternalRegisterName::D, true), true) => Register::SI,

        ((InternalRegisterName::B, false), false) => Register::BL,
        ((InternalRegisterName::B, false), true) => Register::BX,
        ((InternalRegisterName::B, true), false) => Register::BH,
        ((InternalRegisterName::B, true), true) => Register::DI,
    }
}

fn get_register_annotation(reg: u8) -> io::Result<(InternalRegisterName, bool)> {
    Ok((
        InternalRegisterName::try_from(reg)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?,
        (reg & IS_HIGH) == IS_HIGH,
    ))
}

pub fn get_registers_from_byte(byte_b: u8, is_wide: bool) -> io::Result<(Register, Register)> {
    Ok((
        get_register_from_annotation(get_register_annotation(byte_b >> 3)?, is_wide),
        get_register_from_annotation(get_register_annotation(byte_b)?, is_wide),
    ))
}
