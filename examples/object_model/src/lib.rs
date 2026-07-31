//! Object model example.
//!
//! Covers strongly typed derive-based ArkTS objects flowing through:
//! - direct parameters / returns
//! - `Either<T, String>`
//! - `Result<T>`
//! - `Map<string, T>` object values
//! - `Record<string, T>` object values
//! - `Set<T>` object elements

use std::collections::{BTreeMap, HashMap, HashSet};

use ani::conversions::Either;
use ani::prelude::*;
use ani_derive::{ani, AniClass};

#[derive(AniClass, PartialEq, Eq, Hash)]
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

#[ani]
pub fn make_user_profile_directory() -> BTreeMap<String, UserProfile> {
    let mut values = BTreeMap::new();
    values.insert(
        "primary".to_string(),
        UserProfile {
            id: 21,
            name: "directory-primary".to_string(),
            active: true,
        },
    );
    values.insert(
        "backup".to_string(),
        UserProfile {
            id: 22,
            name: "directory-backup".to_string(),
            active: false,
        },
    );
    values
}

#[ani]
pub fn summarize_user_profile_directory(values: BTreeMap<String, UserProfile>) -> String {
    let mut out = Vec::new();
    for (key, value) in values {
        let state = if value.active { "active" } else { "inactive" };
        out.push(format!("{}={}#{}#{}", key, value.id, value.name, state));
    }
    out.join("|")
}

#[ani]
pub fn make_user_profile_record() -> HashMap<String, UserProfile> {
    let mut values = HashMap::new();
    values.insert(
        "primary".to_string(),
        UserProfile {
            id: 41,
            name: "record-primary".to_string(),
            active: true,
        },
    );
    values.insert(
        "backup".to_string(),
        UserProfile {
            id: 42,
            name: "record-backup".to_string(),
            active: false,
        },
    );
    values
}

#[ani]
pub fn summarize_user_profile_record(values: HashMap<String, UserProfile>) -> String {
    let mut entries = values.into_iter().collect::<Vec<_>>();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
        .into_iter()
        .map(|(key, value)| {
            let state = if value.active { "active" } else { "inactive" };
            format!("{}={}#{}#{}", key, value.id, value.name, state)
        })
        .collect::<Vec<_>>()
        .join("|")
}

#[ani]
pub fn make_user_profile_group() -> HashSet<UserProfile> {
    let mut values = HashSet::new();
    values.insert(UserProfile {
        id: 61,
        name: "set-primary".to_string(),
        active: true,
    });
    values.insert(UserProfile {
        id: 62,
        name: "set-backup".to_string(),
        active: false,
    });
    values
}

#[ani]
pub fn summarize_user_profile_group(values: HashSet<UserProfile>) -> String {
    let mut entries = values.into_iter().collect::<Vec<_>>();
    entries.sort_by_key(|left| left.id);
    entries
        .into_iter()
        .map(|value| {
            let state = if value.active { "active" } else { "inactive" };
            format!("{}#{}#{}", value.id, value.name, state)
        })
        .collect::<Vec<_>>()
        .join("|")
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

        let directory = make_user_profile_directory();
        assert_eq!(directory.len(), 2);
        assert_eq!(
            summarize_user_profile_directory(directory),
            "backup=22#directory-backup#inactive|primary=21#directory-primary#active"
        );

        let record = make_user_profile_record();
        assert_eq!(record.len(), 2);
        assert_eq!(
            summarize_user_profile_record(record),
            "backup=42#record-backup#inactive|primary=41#record-primary#active"
        );

        let group = make_user_profile_group();
        assert_eq!(group.len(), 2);
        assert_eq!(
            summarize_user_profile_group(group),
            "61#set-primary#active|62#set-backup#inactive"
        );
    }
}
