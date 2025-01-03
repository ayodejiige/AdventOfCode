
use std::fs;
use std::collections::{HashMap, HashSet};

pub fn main(file_path: String)
{
    // Open file.
    let content = fs::read_to_string(file_path).unwrap();

    // Split by double new lines into rules list and updates.
    let content_split = content.split_terminator("\n\n").collect::<Vec<_>>();
    let update_rules = content_split[0];
    let updates = content_split[1];

    // Create map of update rules. Key is predecessor and value is list of updates
    // that must come after the key.
    let mut update_rules_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for update_rule in update_rules.lines() {
        let parts: Vec<&str> = update_rule.split("|").collect();
        let key = parts[0];
        let value = parts[1];
        if !update_rules_map.contains_key(key) {
            let values = HashSet::new();
            update_rules_map.insert(key, values);
        }
        update_rules_map.get_mut(key).unwrap().insert(value);
    }

    println!("Rules: {update_rules_map:?}");

    // Check if updates are valid and sum up middle parts of the update.
    for update in updates.lines() {
        println!("Updates: {update}");
        let mut is_valid = true;
        // Set to store observed updates
        let mut observed_updates: HashSet<&str> = HashSet::new();

        let update_parts: Vec<&str> = updates.split(",").collect();
        for update_value in update_parts {
            println!("Update value: {update_value}");
            let descendant_updates = update_rules_map.get(update_value);
            
            if let Some(descendant_updates) = descendant_updates {
                // If descendant is already in observed updates, the rules have been broken.
                if !observed_updates.is_disjoint(descendant_updates) {
                    println!("Invalid: {update_value} {update} {descendant_updates:?} {observed_updates:?}");
                    is_valid = false;
                    break;
                }
            }

            observed_updates.insert(update_value);
        }

        if is_valid {
            println!("Valid: {update}");
        }
    }
}