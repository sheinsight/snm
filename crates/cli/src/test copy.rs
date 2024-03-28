use colored::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use snm_core::println_error;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

// 定义节点结构体，用于存储路径段和子节点
#[derive(Debug, Default, Serialize, Clone)]
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

impl Node {}

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

fn filter_dependencies(
    root: &Node,
    dependency_name: &str,
    cache: &mut HashMap<String, Option<Node>>,
) -> Option<Node> {
    // 首先检查缓存中是否已经有搜索结果
    if let Some(cached_result) = cache.get(&root.name) {
        return cached_result.clone();
    }

    // 如果当前节点就是被查找的依赖，则直接克隆整个节点并返回
    if root.name == dependency_name {
        return Some(root.clone());
    }

    // if root.name.contains(dependency_name) {
    //     return Some(root.clone());
    // }

    // 否则，遍历子节点以寻找依赖
    let mut filtered_children: HashMap<String, Node> = HashMap::new();
    for (child_name, child_node) in root.children.iter() {
        if let Some(filtered_child) = filter_dependencies(child_node, dependency_name, cache) {
            filtered_children.insert(child_name.clone(), filtered_child);
        }
    }

    // 如果当前节点包含任何具有被查找依赖的子节点，则保留该节点及其相关子节点
    if !filtered_children.is_empty() {
        Some(Node {
            name: root.name.clone(),
            version: root.version.clone(),
            children: filtered_children,
        })
    } else {
        None
    }
}

fn print_tree(node: &Node, prefix: String, last: bool, match_name: &str) {
    let connector = if last { "└── " } else { "├── " };
    if node.name == match_name {
        println!(
            "{}{}{}",
            prefix,
            connector,
            format!("{}@{}", node.name, node.version).bright_red()
        );
    } else {
        println!("{}{}{}@{}", prefix, connector, node.name, node.version);
    };

    let mut iter = node.children.iter().peekable();
    while let Some((_, child)) = iter.next() {
        let new_prefix = if last { "    " } else { "│   " };
        print_tree(
            child,
            prefix.clone() + new_prefix,
            iter.peek().is_none(),
            match_name,
        );
    }
}

fn main() -> Result<()> {
    let start = Instant::now();

    // let file_path = "/Users/10015448/GitRepository/plm-front/package-lock.json";
    let file_path = "/Users/ityuany/GitRepository/pdc/package-lock.json";
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
            name: ".".to_string(),
            version: ".".to_string(),
            children: HashMap::new(),
        };

        nodeList.iter().for_each(|node| {
            let path = simplify_path_with_scope(&node.name.to_string());
            insert(&mut root, &path, &node.version);
        });

        let query = "query-string";

        let mut cache: HashMap<String, Option<Node>> = HashMap::new(); // 创建

        let x = filter_dependencies(&root, query, &mut cache);

        if let Some(v) = x {
            print_tree(&v, "".to_string(), true, query);
        }

        // println!("{}", serde_json::to_string_pretty(&root).unwrap());
    }
    // 计算并打印经过的时间
    let duration = start.elapsed();

    println!("Function took: {:?}", duration);

    Ok(())
}

fn print_tree_with_style(nodes: &[&Node], prefix: &str) {
    let total = nodes.len();
    for (index, node) in nodes.iter().enumerate() {
        let is_last = index + 1 == total;

        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", prefix, connector, node.name);

        let new_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let child_nodes: Vec<&Node> = node.children.values().collect();
        print_tree_with_style(&child_nodes, &new_prefix);
    }
}

fn insert(root: &mut Node, path: &[String], v: &str) {
    if path.len() == 0 {
        return;
    }

    let current = &path[0];

    match root.children.get_mut(current) {
        Some(x) => insert(x, &path[1..], v),
        None => {
            root.children.insert(
                current.clone(),
                Node {
                    name: current.clone(),
                    version: v.to_string(),
                    children: HashMap::new(),
                },
            );
            if path.len() > 1 {
                insert(root.children.get_mut(&path[0]).unwrap(), &path[1..], v);
            }
        }
    }
}
