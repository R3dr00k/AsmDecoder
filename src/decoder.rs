// TODO add the most important instruction (One byte opcode)

pub fn decode(bytes: &[u8]) {
    // May be 32 bit Mode
    let opcode = bytes[0];
    let high = opcode >> 3;
    let mut low = opcode & 0xf;

    if high <= 3 {
        // first four ligne
        if low <= 5 {
            // frist same grp (ADD , ADC, AND, XOR)
            match high {
                0 => print!("ADD "),
                1 => print!("ADC "),
                2 => print!("AND "),
                3 => print!("XOR "),
                _ => (),
            }
        }
        if low <= 0xd && low >= 8 {
            // second same grp (OR, SBB, SUB, CMP)
            match high {
                0 => print!("OR "),
                1 => print!("SBB "),
                2 => print!("SUB "),
                3 => print!("CMP "),
                _ => (),
            }
            low = low - 8
        }
        match low {
            // here we need to go searh operands
            0 => print!("r/m8, r8"),
            1 => print!("r/m16/32, r16/32"), // prefix 0x66, 67
            2 => print!("r8, r/m8"),
            3 => print!("r16/32, r/m16/32"),
            4 => print!("al, imm8"),
            5 => print!("eax, imm8/16/32"),
            _ => (),
        }
    }
    // INSTRUCTION THAT USE ONE LINE ON TAB CONVERTION
    //
    // INC / DEC 16,32
    if high == 4 {
        if low < 8 {
            line_register("INC", low, 0);
        } else {
            line_register("DEC", low - 8, 0);
        }
    // PUSH / POP comp 16,32,64
    } else if high == 5 {
        if low < 8 {
            line_register("PUSH", low, 0);
        } else {
            line_register("POP", low - 8, 0);
        }
    } else if high == 0xb {
        if low < 8 {
            line_register("MOV", low, 0b1001u8)
        } else {
            line_register("MOV", low - 8, 0b1100u8)
        }
    }
    // CONDITIONAL JMP
    if high == 7 {
        // NOTHING TO DO , nothing to parse
        // condjmp(opcode)
    }

    // IMMEDIATE GRP
    if high == 8 && low <= 3 {
        // instruction extention is in MODRM.reg
        match low {
            0 => print!("GRP_IMM r/m8, imm8"),
            1 => print!("GRP_IMM r/m16/32, imm16,32"),
            2 => print!("GRP_IMM r/m8, imm8 "), // not available in 64 bit mode
            3 => print!("GRP_IMM rm16/32, imm8"),
            _ => (),
        }
    }

    // SHIFT GRP
    if high == 0xC && low <= 1 {
        match low {
            0 => print!("GRP_SHIFT r/m8, imm8"),
            1 => print!("GRP_SHIFT r/m16/32, imm8"),
            _ => (),
        }
    } else if high == 0xD && low <= 3 {
        match low {
            0 => print!("GRP_SHIFT r/m8, 1"),
            1 => print!("GRP_SHIFT r/m16/32, 1"),
            2 => print!("GRP_SHIFT r/m8, cl"),
            3 => print!("GRP_SHIFT r/m16/32, cl"),
            _ => (),
        }
    }

    // MOV GRP  use when disp or sib
    if high == 0xc {
        if low == 6 && low == 7 {
            match low {
                6 => print!("GRP_MOV r/m8, imm8"),
                7 => print!("GRP_MOV r/m16/32, imm16/32"),
                _ => (),
            }
        }
    }

    // INC/DEC GRP
    if high == 0xF {
        if low == 0xE {
            print!("INC/DEC r/m8")
        } else if low == 0xF {
            print!("depend on MODRM.reg")
        }
    }
    // POP GRP
    if opcode == 0x8f {
        print!("POP r/m16/32");
    }

    // Unary GRP
    if opcode == 0xF6 || opcode == 0xF7 {
        print!("Depend on MODRM.reg")
    }
    // ============== SINGLE INSTRUCTION ================
    match opcode {
        // TEST
        0x84 => {
            print!("TEST r/m8, reg8");
        }
        0x85 => {
            print!("TEST r/m16/32, reg16, 32")
        }
        0xA8 => {
            print!("TEST AL, imm8")
        }
        0xA9 => {
            print!("TEST AX/16/32, imm16/32")
        }
        // NOP
        0x90 => {
            print!("NOP")
        }
        // MOV
        0xA0 => {
            print!("MOV Al, Moffs8")
        }
        0xA1 => {
            print!("MOV AX/16/32, Moffs16/32")
        }
        0xA2 => {
            print!("MOV Moffs8, AL")
        }
        0xA3 => {
            print!("MOV Moffs16/32, AX/16/32")
        }
        0x88 => {
            print!("MOV r/m8, reg8")
        }
        0x89 => {
            print!("MOV r/m16/32, reg16/32")
        }
        0x8a => {
            print!("MOV reg8, r/m8")
        }
        0x8b => {
            print!("MOV reg16/32, r/m16/32")
        }
        //RET
        0xC2 => print!("RET imm16"),
        0xC3 => print!("RET"),
        0xCa => print!("RET imm16 (far)"),
        0xCb => print!("RET (far)"),
        //JMP
        0xE9 => print!("JMP moffs16/32 (plus eip, near)"),
        0xEA => print!("JMP direct addr32/48/80 (long)"),
        0xEB => print!("JMP moffs8 (plus ip, short)"),
        //CALL
        0xE8 => print!("CALL moffs16/32 (plus eip, near)"),
        // LEAVE
        0xC9 => print!("LEAVE"),
        //PUSH
        0x68 => print!("PUSH r/m8, reg8"),
        0x6a => print!("PUSH imm8"),
        //Imul
        0x69 => print!("IMUL reg16/32, r/m16/32, imm16"),
        0x6b => print!("IMUL reg16/32, r/m16/32, imm8"),
        //LEA
        0x8D => print!("LEA reg16/32, memory only"),
        _ => (),
    }
}

fn line_register(name: &str, low: u8, op: u8) {
    // op1
    //      0 -> reg16/32
    //      1 -> r8
    // op1 >> 2
    //      0 -> None
    //      1 -> Ax 16/32
    //      2 -> imm8
    //      3 -> imm16/32
    print!("name");
    let op1 = op & 0x3;
    let op2 = op >> 2;
    let sizes: &str;

    if op1 == 0 {
        sizes = "16/32";
    } else {
        sizes = "8";
    }
    match low {
        // here we already got the op
        0 => print!("AX {}", sizes), // depend on prefix
        1 => print!("CX {}", sizes),
        2 => print!("DX {}", sizes),
        3 => print!("BX {}", sizes),
        4 => print!("SP {}", sizes),
        5 => print!("BP {}", sizes),
        6 => print!("SI {}", sizes),
        7 => print!("DI {}", sizes),
        _ => (),
    }

    match op2 {
        1 => print!(", AX 16/32"),
        2 => print!(", imm8"),
        3 => print!(", imm 16/32"),
        _ => (),
    }
}
