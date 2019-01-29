use regex::Regex;
use std::fs;

static SRG_FILE: &'static str = include_str!("../../../resources/srg.srg");

const CLASS_REGEX: &'static str = r"CL: (.*) (.*)"; /* [Notchian name] [MCP name] */
const FIELD_REGEX: &'static str = r"FD: (.*) (.*)"; /* [Notchian class/Notchian name] [MCP class/MCP name] */

/* [Notchian class/Notchian name] [Notchian params] [Notchian return type] [MCP class/MCP name] [MCP params] [MCP return type] */
const METHOD_REGEX: &'static str = r"MD: (.*) \((.*)\)(.*) (.*) \((.*)\)(.*)"; 

pub struct ClassDef {
    pub notchian_name: String, /* Obfuscated vanilla jar name, e.g., 'lo' */
    pub mcp_name: String /* MCP name, e.g., 'net/minecraft/server/network/NetHandlerLoginServer' */
}

pub struct FieldDef {
    pub class_notchian: String, /* Notchian name of the class, e.g., 'anu' */
    pub class_mcp: String, /* MCP name of the class, e.g., 'a' */

    pub notchian_name: String, /* Notchian name of the field, e.g., 'net/minecraft/world/gen/ChunkProviderDebug' */
    pub mcp_name: String /* MCP name of the field, e.g., 'field_177464_a' */
}

/// Finds a class by its MCP name.
pub fn find_class(mcp_name: &str) -> Option<ClassDef> {
    let class_reg = Regex::new(CLASS_REGEX).unwrap();

    let contents = SRG_FILE.to_string();
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

    let contents = SRG_FILE.to_string();
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

/// Finds a field by its MCP name
pub fn find_field(mcp_name: &str, class: ClassDef) -> Option<FieldDef> {
    let field_reg = Regex::new(FIELD_REGEX).unwrap();

    let contents = SRG_FILE.to_string();
    let iter: Vec<&str> = contents.split("\n").collect();

    let name = format!("{}/{}", class.mcp_name, mcp_name);

    for line in iter {
        if(!line.starts_with("FD: ")) {
            continue;
        }
        let cap = field_reg.captures(line.trim()).unwrap();
        if &cap[2] == name {
            let notchian_split: Vec<&str> = cap[1].split("/").collect();
            return Some(FieldDef {
                class_notchian: class.notchian_name,
                class_mcp: class.mcp_name,

                notchian_name: String::from(notchian_split[notchian_split.len() - 1]),
                mcp_name: String::from(mcp_name)
            });
        }
    }
    return None;
}

/// Finds a field by its Notchian name
pub fn find_field_notchian(notchian_name: &str, class: ClassDef) -> Option<FieldDef> {
    let field_reg = Regex::new(FIELD_REGEX).unwrap();

    let contents = SRG_FILE.to_string();
    let iter: Vec<&str> = contents.split("\n").collect();

    let name = format!("{}/{}", class.notchian_name, notchian_name);

    for line in iter {
        if(!line.starts_with("FD: ")) {
            continue;
        }
        let cap = field_reg.captures(line.trim()).unwrap();
        if &cap[1] == name {
            let mcp_split: Vec<&str> = cap[2].split("/").collect();
            return Some(FieldDef {
                class_notchian: class.notchian_name,
                class_mcp: class.mcp_name,

                notchian_name: String::from(notchian_name),
                mcp_name: String::from(mcp_split[mcp_split.len() - 1])
            });
        }
    }
    return None;
}
