use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Circuit {
    wires: HashMap<String, u16>,
}

enum CircuitError {
    InvalidOperation(String),
    UnkownInput,
}

impl Circuit {
    pub fn new() -> Circuit {
        let wires = HashMap::new();
        Circuit { wires }
    }

    pub fn reset(&mut self) {
        self.wires.clear();
    }

    pub fn set_wire(&mut self, wire: String, signal: u16) {
        self.wires.insert(wire, signal);
    }

    pub fn get_wire(&self, r: String) -> u16 {
        self.wires[&r]
    }

    pub fn assemble(&mut self, input: Vec<&str>) {
        let mut q = VecDeque::from_iter(input);
        let mut start_elements = q.len();
        let mut counter = start_elements;

        while !q.is_empty() {
            let i = q.pop_front().unwrap();
            let op = self.decode(i.to_string());

            match op {
                Err(e) => match e {
                    CircuitError::InvalidOperation(e) => panic!("Invalid operation: {}", e),
                    CircuitError::UnkownInput => {
                        q.push_back(i);
                        continue;
                    }
                },
                Ok(op) => {
                    match op {
                        Operation::Assignment(reg, value) => {
                            if self.wires.get(&reg).is_some() {
                                continue;
                            }
                            self.wires.insert(reg, value)
                        }
                        Operation::Not(reg, value) => self.wires.insert(reg, !value),
                        Operation::And(reg, x, y) => self.wires.insert(reg, x & y),
                        Operation::Or(reg, x, y) => self.wires.insert(reg, x | y),
                        Operation::LShift(reg, x, y) => self.wires.insert(reg, x << y),
                        Operation::RShift(reg, x, y) => self.wires.insert(reg, x >> y),
                    };
                }
            }

            // Check we are actually removing entries
            counter -= 1;
            if counter == 0 {
                if start_elements == q.len() {
                    panic!("Elements are no longer being processed: {:?}", q);
                }
                start_elements = q.len();
                counter = start_elements;
            }
        }
    }

    fn parse_arg(&self, input: &str) -> Option<u16> {
        let signal = input.parse::<u16>();

        match signal {
            Ok(signal) => Some(signal),
            Err(_) => self.wires.get(input).copied(),
        }
    }

    fn decode(&self, op: String) -> Result<Operation, CircuitError> {
        let op: Vec<&str> = op.split(' ').collect();

        match &op[..] {
            [x, opcode, y, "->", target] => {
                let x = self.parse_arg(x);
                let y = self.parse_arg(y);
                let target = target.to_string();

                if x == None || y == None {
                    return Err(CircuitError::UnkownInput);
                }

                let x = x.unwrap();
                let y = y.unwrap();

                if opcode == &"AND" {
                    Ok(Operation::And(target, x, y))
                } else if opcode == &"OR" {
                    Ok(Operation::Or(target, x, y))
                } else if opcode == &"LSHIFT" {
                    Ok(Operation::LShift(target, x, y))
                } else if opcode == &"RSHIFT" {
                    Ok(Operation::RShift(target, x, y))
                } else {
                    Err(CircuitError::InvalidOperation(op.join(" ")))
                }
            }
            ["NOT", value, "->", target] => {
                let value = self.parse_arg(value);

                if let Some(value) = value {
                    Ok(Operation::Not(target.to_string(), value))
                } else {
                    Err(CircuitError::UnkownInput)
                }
            }
            [value, "->", target] => {
                let value = self.parse_arg(value);

                if let Some(value) = value {
                    Ok(Operation::Assignment(target.to_string(), value))
                } else {
                    Err(CircuitError::UnkownInput)
                }
            }
            _ => Err(CircuitError::InvalidOperation(op.join(" "))),
        }
    }
}

pub enum Operation {
    Assignment(String, u16),
    Not(String, u16),
    And(String, u16, u16),
    Or(String, u16, u16),
    LShift(String, u16, u16),
    RShift(String, u16, u16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers() {
        let mut circuit = Circuit::new();
        let input = [
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "123 -> x",
            "456 -> y",
            "NOT x -> h",
            "NOT y -> i",
        ];

        circuit.assemble(input.to_vec());

        assert_eq!(circuit.wires["d"], 72);
        assert_eq!(circuit.wires["e"], 507);
        assert_eq!(circuit.wires["f"], 492);
        assert_eq!(circuit.wires["g"], 114);
        assert_eq!(circuit.wires["h"], 65412);
        assert_eq!(circuit.wires["i"], 65079);
        assert_eq!(circuit.wires["x"], 123);
        assert_eq!(circuit.wires["y"], 456);
    }
}
