mod instructions;
mod registers;

use crate::{
    instructions::{get_direction_and_wide, EightySixInstruction, EightySixInstructionType},
    registers::get_registers_from_byte,
};
use log::Level;
use std::{fs, io};

fn parse_instructions(instruction_stream: Vec<u8>) -> io::Result<Vec<EightySixInstruction>> {
    let mut parsed_instructions = vec![];
    let mut instruction_read_idx = 0;
    while instruction_read_idx < instruction_stream.len() {
        let &byte_a = instruction_stream
            .get(instruction_read_idx)
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Could not get instruction byte",
            ))?;

        let instruction_type = EightySixInstructionType::try_from(byte_a)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        let bytes_jump; // Not needed right now, but different instructions might have different lengths
        match instruction_type {
            EightySixInstructionType::MOV => {
                bytes_jump = 2;

                // First make sure we have the second bit for this instruction
                let &byte_b =
                    instruction_stream
                        .get(instruction_read_idx + 1)
                        .ok_or(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "Could not get second byte of MOV instruction",
                        ))?;

                // These are the DW byte
                let (direction, is_wide) = get_direction_and_wide(byte_a);
                // We parse the register using the 86 map, and apply the wide modifier
                let (mut register_a, mut register_b) = get_registers_from_byte(byte_b, is_wide)?;

                // If direction is reversed we swap the memory
                if !direction {
                    std::mem::swap(&mut register_a, &mut register_b);
                }

                // Push the parsed instruction into the vec
                parsed_instructions.push(EightySixInstruction::MOV {
                    destination: register_a,
                    source: register_b,
                });
            }
        }

        // Jump the relevant number of bytes for this instruction
        instruction_read_idx += bytes_jump;
    }

    Ok(parsed_instructions)
}

fn print_out_instructions_for_file(file_name: &str) -> io::Result<()> {
    log::info!("File: `{file_name}`");
    let file_data = fs::read(file_name)?;
    let instructions = parse_instructions(file_data)?;
    for (idx, instruction) in instructions.iter().enumerate() {
        log::info!("#{idx} {}", instruction.to_string());
    }
    Ok(())
}

fn main() -> io::Result<()> {
    simple_logger::init_with_level(Level::Info).expect("Could not init logger");

    print_out_instructions_for_file("./resources/listing_0037_single_register_mov")?;
    print_out_instructions_for_file("./resources/listing_0038_many_register_mov")?;

    Ok(())
}
