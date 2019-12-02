// machine_code[1] = noun
// machine_code[2] = verb
// response verb + 100 * noun

type MachineCode = [u32; 145];
type DataPoints = (usize, usize, usize);

fn generate_machine_code(noun: u32, verb: u32) -> MachineCode {
    let mut machine_code: MachineCode = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 19, 6, 23, 2, 23, 6, 27, 2,
        6, 27, 31, 2, 13, 31, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 6, 43, 47, 1, 13, 47, 51, 2, 6,
        51, 55, 2, 55, 6, 59, 1, 59, 5, 63, 2, 9, 63, 67, 1, 5, 67, 71, 2, 10, 71, 75, 1, 6, 75,
        79, 1, 79, 5, 83, 2, 83, 10, 87, 1, 9, 87, 91, 1, 5, 91, 95, 1, 95, 6, 99, 2, 10, 99, 103,
        1, 5, 103, 107, 1, 107, 6, 111, 1, 5, 111, 115, 2, 115, 6, 119, 1, 119, 6, 123, 1, 123, 10,
        127, 1, 127, 13, 131, 1, 131, 2, 135, 1, 135, 5, 0, 99, 2, 14, 0, 0,
    ];

    machine_code[1] = noun;
    machine_code[2] = verb;

    machine_code
}

fn naive_way() -> u32 {
    let mut result = 0;

    for noun in 0..100 {
        for verb in 0..100 {
            let partial = calculate(noun, verb);

            match partial {
                19690720 => {
                    result = noun * 100 + verb;
                    break;
                }
                _ => continue,
            };
        }
    }
    result
}

fn calculate(noun: u32, verb: u32) -> u32 {
    let mut machine_code = generate_machine_code(noun, verb);

    let mut reading_position = 0;

    fn update_mc(mc: &mut MachineCode, position: usize, value: u32) {
        mc[position] = value;
    }

    fn get_data(mc: &mut MachineCode, reading_position: usize) -> DataPoints {
        let rp1 = reading_position + 1;
        let rp2 = reading_position + 2;
        let rp3 = reading_position + 3;

        let d1 = mc[rp1] as usize;
        let d2 = mc[rp2] as usize;
        let d3 = mc[rp3] as usize;

        (d1, d2, d3)
    }

    loop {
        let operation = machine_code[reading_position];

        match operation {
            1 => {
                let (d1, d2, d3) = get_data(&mut machine_code, reading_position);
                let d1 = machine_code[d1];
                let d2 = machine_code[d2];
                let r = d1 + d2;
                update_mc(&mut machine_code, d3, r);
            }
            2 => {
                let (d1, d2, d3) = get_data(&mut machine_code, reading_position);
                let d1 = machine_code[d1];
                let d2 = machine_code[d2];
                let r = d1 * d2;
                update_mc(&mut machine_code, d3, r);
            }
            99 => {
                break;
            }
            _ => println!("Bad data!"),
        }

        reading_position = reading_position + 4;
    }

    machine_code[0]
}

fn main() {
    let part1 = calculate(12, 2);
    let part2 = naive_way();

    println!("{}", part1);
    println!("{}", part2);
}
