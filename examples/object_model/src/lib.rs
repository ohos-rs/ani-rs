//! Object model example.
//!
//! Covers strongly typed derive-based ArkTS objects flowing through:
//! - direct parameters / returns
//! - `Either<T, String>`
//! - `Result<T>`

use ani::conversions::Either;
use ani::prelude::*;
use ani_derive::{ani, AniClass};

#[derive(AniClass)]
#[ani(class = "UserProfile")]
pub struct UserProfile {
    pub id: i32,
    #[ani(property)]
    pub name: String,
    #[ani(property)]
    pub active: bool,
}

#[ani]
pub fn make_user_profile(id: i32, name: String, active: bool) -> UserProfile {
    UserProfile { id, name, active }
}

#[ani]
pub fn describe_user_profile(user: UserProfile) -> String {
    let state = if user.active { "active" } else { "inactive" };
    format!("{}:{}:{}", user.id, user.name, state)
}

#[ani]
pub fn rename_user_profile(mut user: UserProfile, name: String) -> UserProfile {
    user.name = name;
    user
}

#[ani]
pub fn describe_optional_user_profile(user: Option<UserProfile>) -> String {
    match user {
        Some(user) => describe_user_profile(user),
        None => "none".to_string(),
    }
}

#[ani]
pub fn maybe_user_profile(flag: bool) -> Option<UserProfile> {
    if flag {
        Some(UserProfile {
            id: 11,
            name: "maybe".to_string(),
            active: true,
        })
    } else {
        None
    }
}

#[ani]
pub fn maybe_user_profile_result(flag: bool) -> Result<Option<UserProfile>> {
    if flag {
        Ok(Some(UserProfile {
            id: 12,
            name: "result-option".to_string(),
            active: false,
        }))
    } else {
        Ok(None)
    }
}

#[ani]
pub fn choose_user_profile(flag: bool) -> Either<UserProfile, String> {
    if flag {
        Either::A(UserProfile {
            id: 7,
            name: "ani".to_string(),
            active: true,
        })
    } else {
        Either::B("no-user".to_string())
    }
}

#[ani]
pub fn user_profile_result(flag: bool) -> Result<UserProfile> {
    if flag {
        Ok(UserProfile {
            id: 9,
            name: "result".to_string(),
            active: false,
        })
    } else {
        Err(Error::new(Status::InvalidArgs, "user profile disabled"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_model_functions_compile() {
        let user = make_user_profile(1, "ani".to_string(), true);
        assert_eq!(user.id, 1);
        assert_eq!(describe_user_profile(user), "1:ani:active");

        let renamed = rename_user_profile(
            UserProfile {
                id: 2,
                name: "old".to_string(),
                active: false,
            },
            "new".to_string(),
        );
        assert_eq!(renamed.name, "new");

        assert!(matches!(choose_user_profile(true), Either::A(_)));
        assert_eq!(describe_optional_user_profile(None), "none");
        assert!(maybe_user_profile(true).is_some());
        assert!(maybe_user_profile(false).is_none());
        assert!(maybe_user_profile_result(true).unwrap().is_some());
        assert!(maybe_user_profile_result(false).unwrap().is_none());
        assert!(user_profile_result(true).is_ok());
        assert!(user_profile_result(false).is_err());
    }
}
