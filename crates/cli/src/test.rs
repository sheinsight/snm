use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use snm_core::println_error;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Root {
    name: String,
    version: String,
    packages: HashMap<String, Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: Option<String>,
    version: String,
    dependencies: Option<HashMap<String, String>>,
}

// 定义节点结构体，用于存储路径段和子节点
#[derive(Debug, Default, Serialize)]
struct Node {
    name: String,
    version: String,
    // resolved: String,
    // integrity: String,
    // dev: bool,
    // license: String,
    // children: HashMap<String, Node>,
    children: HashMap<String, Node>,
}

fn simplify_path_with_scope(path: &str) -> Vec<String> {
    let segments: Vec<&str> = path.split('/').collect();
    let mut results: Vec<String> = vec![];

    let mut i = 0;
    while i < segments.len() {
        if segments[i].starts_with("@") && i + 1 < segments.len() {
            // 当前部分是作用域，将其与下一个部分（包名）组合成一个完整的包标识符
            results.push(format!("{}/{}", segments[i], segments[i + 1]));
            i += 2; // 跳过紧随的包名部分
        } else if !segments[i].is_empty() {
            // 不是空字符串，也不是作用域的一部分
            results.push(segments[i].to_string());
            i += 1; // 正常移动到下一个部分
        } else {
            // 忽略空字符串（通常位于字符串开始或结束）
            i += 1;
        }
    }

    results
}

fn main() -> Result<()> {
    println!("xxx");
    let file_path = "/Users/ityuany/GitRepository/mrp-feh-front/package-lock.json";
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let mut v: Value = serde_json::from_str(&data).unwrap();

    if let Value::Number(root) = &v["lockfileVersion"] {
        if root.as_i64().unwrap() == 1 {
            println!("package-lock.json version is 1, please update to version 2");
        }
    }

    if let Value::Object(package) = &v["packages"] {
        let nodeList = package
            .iter()
            .filter(|(k, _)| !k.is_empty())
            .map(|(k, v)| Node {
                name: k
                    .clone()
                    .split("/")
                    .filter(|&seg| seg != "node_modules")
                    .collect::<Vec<&str>>()
                    .join("/"),
                version: v["version"].as_str().unwrap().to_string(),
                children: HashMap::new(),
                // resolved: v["resolved"].as_str().unwrap().to_string(),
                // integrity: v["integrity"].as_str().unwrap().to_string(),
                // dev: v["dev"].as_bool().unwrap(),
                // license: v["license"].as_str().unwrap().to_string(),
            })
            .collect::<Vec<Node>>();

        let mut root = Node {
            name: "root".to_string(),
            version: "".to_string(),
            children: HashMap::new(),
        };

        nodeList.iter().for_each(|node| {
            println!("{}", node.name);
            let path = simplify_path_with_scope(&node.name.to_string());
            insert(&mut root, path);
        });

        let serialized = serde_json::to_string(&root).unwrap();

        println!("Serialized Person as JSON: {}", serialized);
    }

    Ok(())
}

fn insert(root: &mut Node, path: Vec<String>) {
    if path.len() == 0 {
        return;
    }

    let current = &path[0];

    match root.children.get_mut(current) {
        Some(x) => insert(x, path[1..].to_vec()),
        None => {
            root.children.insert(
                current.clone(),
                Node {
                    name: current.clone(),
                    version: "".to_string(),
                    children: HashMap::new(),
                },
            );
            if path.len() > 1 {
                insert(root.children.get_mut(&path[0]).unwrap(), path[1..].to_vec());
            }
        }
    }
}
