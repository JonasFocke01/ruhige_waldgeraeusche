use std::path::Path;

pub fn testfunction() {
    let mut plain_content = match std::fs::read_to_string(Path::new("src/testmodule/snake.tpl")) {
        Ok(e) => e,
        Err(_) => panic!()
    };
    plain_content = plain_content.replace(" ", "");
    plain_content = plain_content.replace("-", " ");
    let vec_content = plain_content.split("\n");

    for (line_i, line) in vec_content.enumerate() {
        if line_i % 6 != 0 {
            print!("line_i: {}\n", line_i);
            print!("line: {}\n", line);
            let mut line_params = line.split(",");

            let x: u8 = line_params.next().unwrap().parse().unwrap();
            let y: u8 = line_params.next().unwrap().parse().unwrap();
            let dir_up: u8 = line_params.next().unwrap().parse().unwrap();
            let dir_down: u8 = line_params.next().unwrap().parse().unwrap();
            let dir_in: u8 = line_params.next().unwrap().parse().unwrap();
            let dir_out: u8 = line_params.next().unwrap().parse().unwrap();
            print!("{}\n", x);
        } else {
            print!("--------------------------------------------------\n");
        }
    }
}