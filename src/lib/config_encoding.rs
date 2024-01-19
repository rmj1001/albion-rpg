use crate::user::profile::UserProfile;

pub fn deserialize_user(data: String) -> Result<UserProfile, String> {
    let user_result: Result<UserProfile, toml::de::Error> = toml::from_str(&data);

    match user_result {
        Ok(profile) => Ok(profile),
        Err(message) => Err(format!(
            "This profile is corrupted and will be deleted: {}",
            message
        )),
    }
}

pub fn serialize_user(user: &UserProfile) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(&user)
}
