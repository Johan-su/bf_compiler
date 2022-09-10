#![allow(unused_parens)]

use std::io::Write;
use std::{env, fs};
use std::fs::File;
use std::process::Command;

#[derive(PartialEq)]
enum TokenType
{
    Unknown,
    Greater,
    Lesser,
    Plus,
    Minus,
    Dot,
    Comma,
    Rightbracket,
    Leftbracket,
}


struct Token
{
    token_type: TokenType,
    row: u64,
    col: u64,
    value: String,
}





fn main()
{
    let args: Vec<String> = env::args().collect();
    let pwd: &String = &args[0];

    if (args.len() < 3) {
        eprintln!("Usage: {} <input file path> <output file name>", pwd);
        return;
    }
    let path: &String = &args[1];
    let out_name: &String = &args[2];






    let file_content: String =
    {
        let r: Result<String, std::io::Error> = fs::read_to_string(path);
        
        match (r) 
        {
            Ok(val) => val,
            Err(err) => {
                eprintln!("ERROR: failed to load file {} with {:?}", path, err.kind());
                return;
            },
        }
    };
    

    let mut token_list: Vec<Token> = Vec::new();
    // parse and add tokens to token_list
    {
        let mut rows: u64 = 1;
        let mut cols: u64 = 1;
        for index in 0..file_content.len()
        {
            cols += 1;
            let data: &[u8] = file_content.as_bytes();
            
            match (data[index] as char)
            {
                '>' => token_list.push(Token {token_type: TokenType::Greater,      row: rows, col: cols, value: ">".to_string()}),
                '<' => token_list.push(Token {token_type: TokenType::Lesser,       row: rows, col: cols, value: "<".to_string()}),
                '+' => token_list.push(Token {token_type: TokenType::Plus,         row: rows, col: cols, value: "+".to_string()}),
                '-' => token_list.push(Token {token_type: TokenType::Minus,        row: rows, col: cols, value: "-".to_string()}),
                '.' => token_list.push(Token {token_type: TokenType::Dot,          row: rows, col: cols, value: ".".to_string()}),
                ',' => token_list.push(Token {token_type: TokenType::Comma,        row: rows, col: cols, value: ",".to_string()}),
                '[' => token_list.push(Token {token_type: TokenType::Rightbracket, row: rows, col: cols, value: "[".to_string()}),
                ']' => token_list.push(Token {token_type: TokenType::Leftbracket,  row: rows, col: cols, value: "]".to_string()}),
                
                '\n' => {rows += 1; cols = 1},

                _ => {},
            }
        }
    }


    let mut file: File = File::create("out.asm").unwrap();
    setup_asm(&mut file);
    {
        for index in 0..token_list.len()
        {
            let token: &Token = &token_list[index];
            match (token.token_type)
            {
                TokenType::Unknown => {
                    panic!("Unreachable");
                },
                TokenType::Greater => {
                    let asm = 
                    "
                    inc r13
                    ";
                    file.write_all(asm.as_bytes()).unwrap();
                },
                TokenType::Lesser => {
                    let asm = 
                    "
                    dec r13\n
                    ";
                    file.write_all(asm.as_bytes()).unwrap();
                },
                TokenType::Plus => {
                    let asm =
                    "
                    mov al, [r13]
                    inc al
                    mov [r13], al
                    ";
                    file.write_all(asm.as_bytes()).unwrap();

                },
                TokenType::Minus => {
                    let asm =
                    "
                    mov al, [r13]
                    dec al
                    mov [r13], al
                    ";
                    file.write_all(asm.as_bytes()).unwrap();

                },
                TokenType::Dot => {
                    let asm = 
                    "
                    movzx rcx, byte [r13]\n
                    call putchar\n
                    ";
                    file.write_all(asm.as_bytes()).unwrap();
                },
                TokenType::Comma => {
                    let asm =
                    "
                    call getchar
                    mov [r13], rax
                    ";
                    file.write_all(asm.as_bytes()).unwrap();
                },
                TokenType::Rightbracket => {

                    let mut left_bracket: &Token = &Token { 
                        token_type: TokenType::Unknown, 
                        row: (0), 
                        col: (0), 
                        value: ("".to_string()) 
                    };
                    {
                        let mut skip_nested_block_count: u64 = 0;
                        for index2 in index + 1..token_list.len()
                        {
                            if (token_list[index2].token_type == TokenType::Rightbracket)
                            {
                                skip_nested_block_count += 1;
                                continue;
                            }


                            if (token_list[index2].token_type == TokenType::Leftbracket)
                            {
                                if (skip_nested_block_count == 0)
                                {
                                    left_bracket = &token_list[index2]; 
                                    break;
                                }
                                else
                                {
                                    skip_nested_block_count -= 1;    
                                }
                            }

                        }
                        if (left_bracket.token_type != TokenType::Leftbracket)
                        {
                            eprintln!("ERROR: {}:{} block has to end with a ]", token.row, token.col);
                            return;
                        }
                    }

                    let asm =
                    "
                    movzx r12, byte [r13]\n
                    cmp r12, 0\n
                    je while_loop_end".to_string() + &left_bracket.row.to_string() + "_" + &left_bracket.col.to_string() + "\n" +
                    "while_loop_" +                   &token.row.to_string() + "_" + &token.col.to_string() + ":\n";
                    
                    file.write_all(asm.as_bytes()).unwrap(); 
                },
                TokenType::Leftbracket  => {

                    let mut right_bracket: &Token = &Token { 
                        token_type: TokenType::Unknown, 
                        row: (0), 
                        col: (0), 
                        value: ("".to_string()) 
                    };
                    
                    {
                        let mut skip_nested_block_count: u64 = 0;
                        for index2 in (0..index - 1).rev()
                        {
                            if (token_list[index2].token_type == TokenType::Leftbracket) 
                            {
                                skip_nested_block_count += 1;    
                            }

                            if (token_list[index2].token_type == TokenType::Rightbracket)
                            {
                                if (skip_nested_block_count == 0)
                                {
                                    right_bracket = &token_list[index2]; 
                                    break;
                                }
                                else
                                {
                                    skip_nested_block_count -= 1;
                                }
                            }
                        }
                        if (right_bracket.token_type != TokenType::Rightbracket)
                        {
                            eprintln!("ERROR: {}:{} block has to start with a [ ", token.row, token.col);
                            return;
                        }
                    }

                    let asm = 
                    "
                    movzx r12, byte [r13]\n
                    cmp r12, 0\n
                    jne while_loop_".to_owned() + &right_bracket.row.to_string() + "_" + &right_bracket.col.to_string() + "\n" + 
                    "while_loop_end" + &token.row.to_string() + "_" + &token.col.to_string() + ":\n";
                    file.write_all(asm.as_bytes()).unwrap();
                },
            }
        }
    }
    setup_end_asm(&mut file);


    {
        let mut assemble: Command;
        let mut link: Command;
        let mut clean_up: Command;
    
        if cfg!(target_os = "windows")
        {
            assemble = Command::new("yasm"); 
            link = Command::new("ld");
            clean_up = Command::new("rm");
        }
        else
        {
            assemble = Command::new("yasm");
            assemble.args(["-f", "elf64", "out.asm", "-o", "out.obj"]);
    
    
            link = Command::new("ld");
            link.args(["out.obj", "-entry=main", "-o", out_name]);
    
    
            clean_up = Command::new("rm");
            clean_up.args(["out.asm", "out.obj"]);
        }
    
        assemble.output().expect("failed to assemble");
        link.output().expect("failed to link");
        clean_up.output().expect("failed to clean up");
    }




    return;
}


#[cfg(target_os = "windows")]
fn setup_asm(file: &mut File) // TODO(Johan): use syscalls for linux and libc for windows
{
    let setup = 
    "
    bits 64
    default rel

    segment .data
    buffer: times 30000 db 0;
    
    segment .text
    global main
    extern putchar
    extern getchar
    extern ExitProcess
    
    main:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 32
    lea r13, [buffer]
    ";
    file.write_all(setup.as_bytes()).unwrap();
}


#[cfg(target_os = "windows")]
fn setup_end_asm(file: &mut File)
{
    let setup =
    "
    pop rbp
    xor rcx, rcx
    call ExitProcess
    ";
    file.write(setup.as_bytes()).unwrap();
}


#[cfg(target_os = "linux")]
fn setup_asm(file: &mut File)
{
    let setup = 
    "
    bits 64
    default rel

    segment .data
    buffer: times 30000 db 0;
    temp_buffer: times 1 db 0;

    segment .text
    global main
    
    putchar:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 32

        lea rsi, [temp_buffer]
        mov rax, 1
        mov rdi, 1
        mov [rsi], rcx
        mov rdx, 1
        syscall
        
        mov rsp, rbp
        pop rbp
        ret

    getchar:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 32

        mov rax, 0
        mov rdi, 0
        lea rsi, [temp_buffer]
        mov rdx, 1

        syscall
        mov rax, [rsi]

        mov rsp, rbp
        pop rbp
        ret

    main:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 32
        lea r13, [buffer]
    ";
    file.write_all(setup.as_bytes()).unwrap();
}


#[cfg(target_os = "linux")]
fn setup_end_asm(file: &mut File)
{
    let setup =
    "
    mov rsp, rbp
    pop rbp
    mov rax, 60
    mov rdi, 0
    syscall 
    ";
    file.write(setup.as_bytes()).unwrap();
}