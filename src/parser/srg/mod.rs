use regex::Regex;
use std::fs;

const CLASS_REGEX: &'static str = r"CL: (.*) (.*)"; /* [Notchian name] [MCP name] */
const FIELD_REGEX: &'static str = r"FD: (.*) (.*)"; /* [Notchian class/Notchian name] [MCP class/MCP name] */

/* [Notchian class/Notchian name] [Notchian params] [Notchian return type] [MCP class/MCP name] [MCP params] [MCP return type] */
const METHOD_REGEX: &'static str = r"MD: (.*) \((.*)\)(.*) (.*) \((.*)\)(.*)"; 

pub struct ClassDef {
    pub notchian_name: String, /* Obfuscated vanilla jar name, e.g., 'lo' */
    pub mcp_name: String /* MCP name, e.g., 'net/minecraft/server/network/NetHandlerLoginServer' */
}

/// Finds a class by its MCP name.
pub fn find_class(mcp_name: &str) -> Option<ClassDef> {
    let class_reg = Regex::new(CLASS_REGEX).unwrap();

    let contents = fs::read_to_string("resources/srg.srg").unwrap();
    let iter: Vec<&str> = contents.split("\n").collect();

    for line in iter {
        if(!line.starts_with("CL: ")) {
            continue;
        }
        let cap = class_reg.captures(line.trim()).unwrap();
        if &cap[2] == mcp_name {
            return Some(ClassDef {
                notchian_name: String::from(&cap[1]),
                mcp_name: String::from(mcp_name)
            });
        }
    }
    return None;

}

/// Finds a class by its Notchian name
pub fn find_class_notchian(notchian_name: &str) -> Option<ClassDef> {
    let class_reg = Regex::new(CLASS_REGEX).unwrap();

    let contents = fs::read_to_string("resources/srg.srg").unwrap();
    let iter: Vec<&str> = contents.split("\n").collect();

    for line in iter {
        if(!line.starts_with("CL: ")) {
            continue;
        }
        let cap = class_reg.captures(line.trim()).unwrap();
        if &cap[1] == notchian_name {
            return Some(ClassDef {
                notchian_name: String::from(notchian_name),
                mcp_name: String::from(&cap[2])
            });
        }
    }
    return None;

}