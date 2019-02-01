use parser::srg::*;

pub fn parse_command(el_type: &char, from: &char, to: &char, value: &str) {

    let mut result = String::from("Error");

    match el_type {
        'c' => result = res_class(from, to, value),
        _ => {}
    }

    println!("Result: {}", result);

}

fn res_class(from: &char, to: &char, value: &str) -> String {
    println!("From: {} To: {}", from, to);
    match from {
        'n' => {
            match to {
                'm' => {
                    let res = find_class_notchian(value).unwrap().mcp_name;
                    println!("{}", res);
                    return res;
                }
                'f' => {}
                _ => {}
            }
        }
        'm' => {
            match to {
                'n' => {
                    let res = find_class(value).unwrap().notchian_name;
                    return res;
                }
                'f' => {}
                _ => {}
            }
        }
        'f' => {}
        _ => {}
    }
    String::from("Error")
}