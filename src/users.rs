use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: Uuid,
    username: String,
    nickname: String,
    is_active: bool,
    created_at: u32,
}

impl User {
    pub fn new(
        id: Uuid,
        username: impl Into<String>,
        nickname: impl Into<String>,
        is_active: bool,
        created_at: u32,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            nickname: nickname.into(),
            is_active,
            created_at,
        }
    }

    pub fn change_nickname(&mut self, new_nickname: impl Into<String>) -> &mut Self {
        self.nickname = new_nickname.into();
        self
    }

    pub fn toggle_is_active(&mut self) -> &mut Self {
        self.is_active = !self.is_active;
        self
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let User {
            id,
            username,
            nickname,
            is_active,
            created_at,
        } = self;
        write!(
            f,
            "User {id} {{ username: {username}, nickname: {nickname}, is_active: {is_active}, created_at: {created_at} }}"
        )
    }
}

