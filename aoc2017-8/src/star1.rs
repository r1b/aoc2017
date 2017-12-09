use std::collections::HashMap;
use std::collections::hash_map::Drain;
use std::io::{BufRead, Cursor};
use regex::Regex;

struct Condition {
    pub register: String,
    operator: String,
    operand: i32
}

impl Condition {
    pub fn is_satisfied(&self, register_value: i32) -> bool {
        match self.operator.as_str() {
            ">" => register_value > self.operand,
            "<" => register_value < self.operand,
            ">=" => register_value >= self.operand,
            "<=" => register_value <= self.operand,
            "==" => register_value == self.operand,
            "!=" => register_value != self.operand,
            _ => panic!("well")
        }
    }

    pub fn get_register(&self) -> String {
        self.register.clone()
    }
}

struct Registers {
    registers: HashMap<String, i32>
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: HashMap::new()
        }
    }

    pub fn drain(&mut self) -> Drain<String, i32> {
        self.registers.drain()
    }

    pub fn init_reg(&mut self, register: String) -> () {
        self.registers.entry(register).or_insert(0);
    }

    pub fn reg_value(&self, register: String) -> &i32 {
        self.registers.get(&register).unwrap()
    }

    pub fn update_reg(&mut self, register: String, value: i32) -> () {
        self.registers.insert(register, value);
    }
}

struct Routine {
    pub register: String,
    instruction: String,
    operand: i32,
    condition: Condition
}

impl Routine {
    pub fn new(buf: String) -> Self {
        let re = Regex::new(
            r"([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) (>|<|>=|<=|==|!=) (-?\d+)"
        ).unwrap();
        let caps = re.captures(&buf).unwrap();

        Routine {
            register: String::from(caps.get(1).unwrap().as_str()),
            instruction: String::from(caps.get(2).unwrap().as_str()),
            operand: i32::from_str_radix(caps.get(3).unwrap().as_str(), 10).unwrap(),
            condition: Condition {
                register: String::from(caps.get(4).unwrap().as_str()),
                operator: String::from(caps.get(5).unwrap().as_str()),
                operand: i32::from_str_radix(caps.get(6).unwrap().as_str(), 10).unwrap()
            }
        }
    }

    pub fn execute(&self, registers: &Registers) -> i32 {
        // FIXME : Why do I have to clone
        let reg_value = registers.reg_value(self.register.clone());
        let condition_reg_value = registers.reg_value(self.condition.register.clone());

        if self.condition.is_satisfied(*condition_reg_value) {
            return match self.instruction.as_str() {
                "inc" => *reg_value + self.operand,
                "dec" => *reg_value - self.operand,
                _ => panic!("at the disco AHAHAHAHA")
            }
        }

        *reg_value
    }
}

pub fn max_reg_value(input: &'static str) -> i32 {
    let mut maximum = None;
    let mut registers = Registers::new();
    let routines = parse_input(input);

    // execute routines
    for routine in routines {
        registers.init_reg(routine.register.clone());
        registers.init_reg(routine.condition.register.clone());
        let new_value = routine.execute(&registers);
        registers.update_reg(routine.register.clone(), new_value);
    }

    for (_, value) in registers.drain() {
        maximum = Some(match maximum {
            Some(max) => if value > max { value } else { max },
            None =>  value
        });
    }

    maximum.unwrap()
}

fn parse_input(input: &'static str) -> Vec<Routine> {
    let mut cursor = Cursor::new(input);
    let mut routines = Vec::new();

    loop {
        let mut buf = String::new();
        let num_bytes = cursor.read_line(&mut buf).unwrap();

        if num_bytes == 0 {
            break;
        }
        else {
            routines.push(Routine::new(buf));
        }
    }

    routines
}
