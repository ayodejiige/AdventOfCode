use crate::common::quick_sort;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn main(file_path: String) {
    // Open file.
    let content = fs::read_to_string(file_path).unwrap();

    // Split by double new lines into rules list and updates.
    let content_split = content
        .split_terminator("\n\n")
        .collect::<Vec<_>>();
    let update_rules = content_split[0];
    let updates = content_split[1];

    // Create map of update rules. Key is predecessor and value is list of updates
    // that must come after the key.
    let mut update_rules_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for update_rule in update_rules.lines() {
        let parts: Vec<&str> = update_rule
            .split("|")
            .collect();
        let key = parts[0];
        let value = parts[1];
        if !update_rules_map.contains_key(key) {
            let values = HashSet::new();
            update_rules_map.insert(key, values);
        }
        update_rules_map
            .get_mut(key)
            .unwrap()
            .insert(value);
    }

    // Check if updates are valid and sum up middle parts of the update.
    let mut middle_page_sum_valid: u64 = 0;
    let mut invalid_updates: Vec<Vec<&str>> = Vec::new();
    for update in updates.lines() {
        let mut is_valid = true;
        // Set to store observed updates
        let mut observed_pages: HashSet<&str> = HashSet::new();

        let pages: Vec<&str> = update
            .split(",")
            .collect();
        for page in &pages {
            let dependent_pages = update_rules_map.get(page);

            if let Some(descendant_updates) = dependent_pages {
                // If descendant is already in observed updates, the rules have been broken.
                if !observed_pages.is_disjoint(descendant_updates) {
                    is_valid = false;

                    break;
                }
            }

            observed_pages.insert(page);
        }

        if is_valid {
            let middle_page = pages[pages.len() / 2];
            let middle_page = u64::from_str_radix(middle_page, 10).unwrap();
            middle_page_sum_valid += middle_page;
        } else {
            invalid_updates.push(pages);
        }
    }

    // Fix incorrect pages and calculate middle sum
    let mut middle_page_sum_invalid: u64 = 0;
    for mut pages in invalid_updates {
        // Sort invalid update pages based on update rules.
        quick_sort(&mut pages, |a, b| {
            // If a should preceed b in the update, a < b.
            if update_rules_map.contains_key(*a) {
                let rule = update_rules_map
                    .get(*a)
                    .unwrap();
                if rule.contains(*b) {
                    return std::cmp::Ordering::Less;
                }
            }

            // If b should precede a in the update, a > b.
            if update_rules_map.contains_key(*b) {
                let rule = update_rules_map
                    .get(*b)
                    .unwrap();
                if rule.contains(*a) {
                    return std::cmp::Ordering::Greater;
                }
            }

            // If no rules apply to a and b, a == b.
            std::cmp::Ordering::Equal
        });

        let middle_page = pages[pages.len() / 2];
        let middle_page = u64::from_str_radix(middle_page, 10).unwrap();
        middle_page_sum_invalid += middle_page;
    }

    println!("Middle Page Sum (Valid Updates): {middle_page_sum_valid}");
    println!("Middle Page Sum (Fixed Invalid Updates): {middle_page_sum_invalid}");
}
