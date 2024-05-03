use std::process::Command;
use std::option::Option;
use serde_json::Value;

// this function recurse through a node in an hyprland tree and:
// a) return the node if it is the focused one
// b) calls this function on all the sub-nodes (normal or floating)
fn recurse_nodes(tree: &Value) -> Option<&Value> {
    if tree["focused"] == true {
        Some(tree)
    }
    else {
        if let Value::Array(node_vector) = &tree["nodes"] {
            for node in node_vector {
                if let Some(focused_node) = recurse_nodes(&node) {
                    return Some(focused_node);
                }
            }
        }
        if let Value::Array(node_vector) = &tree["floating_nodes"] {
            for node in node_vector {
                if let Some(focused_node) = recurse_nodes(&node) {
                    return Some(focused_node);
                }
            }
        }
        None
    }
}

pub fn sway_focus() -> String {
    let tree_json = Command::new("swaymsg")
                            .arg("-t")
                            .arg("get_tree")
                            .output()
                            .expect("Error with swaymsg -t get_tree");

    let tree_json = String::from_utf8_lossy(&tree_json.stdout);
    let tree: Value = serde_json::from_str(&tree_json.to_string()).expect("Error parsing json by sway");

    match recurse_nodes(&tree) {
        Some(focused_node) => {
            if let Some(app) = focused_node["app_id"].as_str(){
                return String::from(app);
            }
            else {
                if let Some(class) = focused_node["window_properties"]["class"].as_str(){
                    return String::from(class);
                }
                else {
                    return String::from("default");
                }
            }
        }
        None => return String::from("default"),
    }
}

pub fn hyprland_focus() -> String {
    let tree_json = Command::new("hyprctl")
                            .arg("activewindow")
                            .arg("-j")
                            .output()
                            .expect("Error with hyprctl");

    let tree_json = String::from_utf8_lossy(&tree_json.stdout);
    let tree: Value = serde_json::from_str(&tree_json.to_string()).expect("Error parsing the json by Hyprland");
    let class = &tree["class"].to_string().replace("\"","");
    if class.to_string().is_empty() {
        return tree["initialTitle"].to_string().replace("\"","")
    }
    return class.clone()
}

// this function returns the app_id (or class if it runs on xwayland) of the
// focused window
// TODO: check if the WM is Hyprland or Sway and use the corresponding function
pub fn getfocusedwindow() -> String {
    return hyprland_focus();
}
