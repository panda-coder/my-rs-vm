use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive, Debug)]
pub enum Bytecode {
    ICONST = 9,
    PRINT = 14,
    HALT = 16
}


#[derive(Debug)]
pub struct VirtualMachine {
    code: Vec<i32>,
    stack: Vec<i32>,
    sp: i32,
    fp: i32,
    ip: i32
}

impl VirtualMachine {
    pub fn cpu(&mut self) {
        
        while(self.ip as usize) < self.code.len() {
            println!("OPCODE {}, line {}", self.code[self.ip as usize], self.ip);
            match FromPrimitive::from_i32(self.code[self.ip as usize]){
                Some(Bytecode::ICONST) => {
                    self.ip += 1;
                    self.sp += 1;
                    self.stack[self.sp as usize] = self.code[self.ip as usize]
                },
                Some(Bytecode::PRINT) => {
                    println!("{}", self.stack[self.sp as usize]);
                    self.sp -= 1;
                },
                Some(Bytecode::HALT) => {
                    return;
                },
                None => println!("Not covered! {}", self.code[self.ip as usize])
            }
            self.ip += 1;
        }
    }
}

pub fn new_vm(code: Vec<i32>, main: i32, datasize: i32) -> VirtualMachine {
    let stack = vec![0, datasize];
    VirtualMachine {
        code: code,
        stack: stack,
        ip: main,
        sp: -1,
        fp: 0
    }
}