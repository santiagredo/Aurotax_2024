use crate::utils::{has_invalid_chars, is_empty, is_lt_zero, is_too_short};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UserData {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

const ID: &'static str = "id";
const NAME: &'static str = "name";
const EMAIL: &'static str = "email";
const PASSWORD: &'static str = "password";

impl UserData {
    pub fn validate(incoming_data: UserData) -> Result<Self, String> {
        let mut new_user_data = UserData::default();

        for (key, value) in incoming_data.into_iter() {
            match key {
                ID => match value {
                    None => (),
                    Some(val) => {
                        let user_id = match val.parse::<i32>() {
                            Err(_) => return Err(format!("{val} is not a valid i32")),
                            Ok(val) => val,
                        };

                        UserData::check_id(user_id)?;

                        new_user_data.id = Some(user_id)
                    }
                },
                NAME => match value {
                    None => (),
                    Some(val) => {
                        UserData::check_name(val.to_string())?;

                        new_user_data.name = Some(val.to_lowercase().trim().to_string())
                    }
                },
                EMAIL => match value {
                    None => (),
                    Some(val) => {
                        UserData::check_email(val.to_string())?;

                        new_user_data.email = Some(val.to_lowercase().trim().to_string())
                    }
                },
                PASSWORD => match value {
                    None => (),
                    Some(val) => {
                        UserData::check_password(val.to_string())?;

                        new_user_data.password = Some(val.trim().to_string())
                    }
                },
                _ => (),
            }
        }

        Ok(new_user_data)
    }

    pub fn unwrap_or_error<T>(option: Option<T>) -> Result<T, String> {
        match option {
            None => Err(format!("Given Option is None")),
            Some(val) => Ok(val),
        }
    }

    pub fn check_none_values(&self) -> Result<(), String> {
        for (key, value) in self.clone().into_iter() {
            if value.is_none() && key != ID {
                return Err(format!("{key} is None"));
            }
        }

        Ok(())
    }

    pub fn check_id(s: i32) -> Result<i32, String> {
        is_lt_zero(s)?;

        Ok(s)
    }

    pub fn check_name(s: String) -> Result<String, String> {
        is_empty(s.to_string())?;
        has_invalid_chars(s.to_string())?;

        Ok(s)
    }

    pub fn check_email(s: String) -> Result<String, String> {
        is_empty(s.to_string())?;
        has_invalid_chars(s.to_string())?;

        Ok(s)
    }

    pub fn check_password(s: String) -> Result<String, String> {
        is_empty(s.to_string())?;
        is_too_short(s.to_string())?;

        match has_invalid_chars(s.to_string()) {
            Err(_) => return Err(format!("'Password' has invalid characters")),
            Ok(_) => (),
        };

        Ok(s)
    }
}

impl IntoIterator for UserData {
    type Item = (&'static str, Option<String>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            (ID, self.id.map(|v| v.to_string())),
            (NAME, self.name),
            (EMAIL, self.email),
            (PASSWORD, self.password),
        ]
        .into_iter()
    }
}
