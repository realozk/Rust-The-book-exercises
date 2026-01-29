pub struct LoginAttempt {
    pub username: String,
    pub ip: String,
    pub success: bool,
}

impl LoginAttempt {
    pub fn new(user: &str, ip: &str, success: bool) -> LoginAttempt {
        LoginAttempt {
            username: String::from(user),
            ip: String::from(ip),
            success,
        }
    }
}


pub fn analyze_attempt(attempt: &LoginAttempt) -> bool {
    if !attempt.success && attempt.username == "admin" {
        return true; 
    }
    false
}