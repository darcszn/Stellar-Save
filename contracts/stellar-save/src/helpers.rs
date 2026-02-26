//! Helper utilities for formatting and display

use soroban_sdk::{String, Env, Bytes};
use crate::Group;

/// Formats a group ID for display with a "GROUP-" prefix.
/// 
/// # Arguments
/// * `env` - Soroban environment for string allocation
/// * `group_id` - The numeric group ID to format
/// 
/// # Returns
/// A formatted string in the format "GROUP-{id}"
/// 
/// # Example
/// ```
/// let formatted = format_group_id(&env, 42);
/// // Returns: "GROUP-42"
/// ```
pub fn format_group_id(env: &Env, group_id: u64) -> String {
    // Convert u64 to bytes manually
    let mut num = group_id;
    let mut digits = Bytes::new(env);
    
    if num == 0 {
        digits.push_back(b'0');
    } else {
        // Build digits in reverse, then reverse them
        let mut temp = Bytes::new(env);
        while num > 0 {
            temp.push_back(b'0' + (num % 10) as u8);
            num /= 10;
        }
        // Reverse the digits
        for i in (0..temp.len()).rev() {
            digits.push_back(temp.get(i).unwrap());
        }
    }
    
    // Build the final string: "GROUP-" + digits
    let mut result = Bytes::new(env);
    result.push_back(b'G');
    result.push_back(b'R');
    result.push_back(b'O');
    result.push_back(b'U');
    result.push_back(b'P');
    result.push_back(b'-');
    
    // Append digits
    for i in 0..digits.len() {
        result.push_back(digits.get(i).unwrap());
    }
    
    String::from_bytes(env, &result)
}

/// Checks if the current cycle deadline has passed.
/// 
/// # Arguments
/// * `group` - The group to check
/// * `current_time` - Current timestamp in seconds
/// 
/// # Returns
/// `true` if the deadline has passed, `false` otherwise
/// 
/// # Example
/// ```
/// let deadline_passed = is_cycle_deadline_passed(&group, env.ledger().timestamp());
/// ```
pub fn is_cycle_deadline_passed(group: &Group, current_time: u64) -> bool {
    if !group.started {
        return false;
    }
    
    let cycle_deadline = group.started_at + (group.cycle_duration * (group.current_cycle as u64 + 1));
    current_time > cycle_deadline
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, Address};
    use crate::group::GroupStatus;

    #[test]
    fn test_format_group_id_single_digit() {
        let env = Env::default();
        let result = format_group_id(&env, 1);
        assert_eq!(result, String::from_str(&env, "GROUP-1"));
    }

    #[test]
    fn test_format_group_id_multi_digit() {
        let env = Env::default();
        let result = format_group_id(&env, 12345);
        assert_eq!(result, String::from_str(&env, "GROUP-12345"));
    }

    #[test]
    fn test_format_group_id_zero() {
        let env = Env::default();
        let result = format_group_id(&env, 0);
        assert_eq!(result, String::from_str(&env, "GROUP-0"));
    }

    #[test]
    fn test_format_group_id_max_value() {
        let env = Env::default();
        let result = format_group_id(&env, u64::MAX);
        let expected = String::from_str(&env, "GROUP-18446744073709551615");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_cycle_deadline_passed_not_started() {
        let env = Env::default();
        let creator = Address::generate(&env);
        let group = Group::new(1, creator, 1000000, 604800, 5, 2, 1000);
        
        assert!(!is_cycle_deadline_passed(&group, 2000));
    }

    #[test]
    fn test_is_cycle_deadline_passed_before_deadline() {
        let env = Env::default();
        let creator = Address::generate(&env);
        let mut group = Group::new(1, creator, 1000000, 604800, 5, 2, 1000);
        group.activate(1000);
        
        // Current time before deadline (started_at + cycle_duration)
        assert!(!is_cycle_deadline_passed(&group, 1000 + 604800));
    }

    #[test]
    fn test_is_cycle_deadline_passed_after_deadline() {
        let env = Env::default();
        let creator = Address::generate(&env);
        let mut group = Group::new(1, creator, 1000000, 604800, 5, 2, 1000);
        group.activate(1000);
        
        // Current time after deadline
        assert!(is_cycle_deadline_passed(&group, 1000 + 604800 + 1));
    }

    #[test]
    fn test_is_cycle_deadline_passed_second_cycle() {
        let env = Env::default();
        let creator = Address::generate(&env);
        let mut group = Group::new(1, creator, 1000000, 604800, 5, 2, 1000);
        group.activate(1000);
        group.advance_cycle(&env);
        
        // Deadline for cycle 1 is started_at + (cycle_duration * 2)
        assert!(!is_cycle_deadline_passed(&group, 1000 + 604800 * 2));
        assert!(is_cycle_deadline_passed(&group, 1000 + 604800 * 2 + 1));
    }
}
