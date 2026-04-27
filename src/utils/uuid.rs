use std::time::SystemTime;

pub fn generate_uuid() -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", now)
}
