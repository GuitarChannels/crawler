use super::consts::DEVELOPMENT;

pub fn get_db_name(environment: &str) -> String {
    if environment.eq(DEVELOPMENT) {
        "guitar-channels-dev".to_string()
    } else {
        "guitar-channels".to_string()
    }
}
