pub fn is_empty(s: String) -> Result<String, String> {
    match s.trim().is_empty() {
        true => Err(format!("'{s}' is empty")),
        false => Ok(s)
    }
}

pub fn has_invalid_chars(s: String) -> Result<String, String> {
    let invalid_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

    let contains_invalid_chars = s.trim().chars().any(|g| invalid_chars.contains(&g));

    match contains_invalid_chars {
        true => Err(format!("'{s}' has invalid characters")),
        false => Ok(s)
    }
}

pub fn is_lt_zero(id: i32) -> Result<i32, String> {
    match id < 0 {
        true => Err(format!("'{id}' is lesser than 0")),
        false => Ok(id)
    }
}

pub fn is_too_short(s: String) -> Result<String, String> {
    match s.len() < 7 {
        true => Err(format!("'{s}' is too short")),
        false => Ok(s)
    }
}