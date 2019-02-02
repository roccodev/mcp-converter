use parser::srg::*;

pub fn parse_command(el_type: &char, from: &char, to: &char, value: &str) {

    let mut result = String::from("Error");

    match el_type {
        'c' => result = res_class(from, to, value),
        'f' => result = res_field(from, to, value),
        _ => {}
    }

    println!("{}", result);

}

fn res_class(from: &char, to: &char, value: &str) -> String {
    match from {
        'n' => {
            match to {
                'm' => {
                    let res = find_class_notchian(value).expect("Class not found by the notchian name provided.").mcp_name;
                    return res;
                }
                'f' => {}
                _ => {}
            }
        }
        'm' => {
            match to {
                'n' => {
                    let res = find_class(value).expect("Class not found by the MCP name provided.").notchian_name;
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

fn res_field(from: &char, to: &char, value: &str) -> String {
    let data: Vec<&str> = value.split("#").collect(); // Class#Field
    match from {
        'n' => {
            match to {
                'm' => {
                    let res = find_field_notchian(&data[1], find_class_notchian(&data[0]).expect("Class not found."))
                        .expect("Field not found by the notchian name provided.").mcp_name;
                    return res;
                }
                'f' => {}
                _ => {}
            }
        }
        'm' => {
            match to {
                'n' => {
                    let res = find_field(&data[1], find_class(&data[0]).expect("Class not found."))
                        .expect("Field not found by the MCP name provided.").notchian_name;
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