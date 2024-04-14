#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub ical_url: Option<String>,
}

#[derive(Default, Clone)]
pub struct DB {
    user: Vec<User>,
}

impl DB {
    pub fn insert_user(&mut self, user: User) -> User {
        self.user.push(user.clone());
        user
    }
    pub fn get_user(&self, id: String) -> Option<User> {
        let user = self.user.iter().rfind(|u| u.id == id);
        if let Some(u) = user {
            return Some(u.clone());
        }
        None
    }
    pub fn update_user(&mut self, id: String, user: User) -> Result<User, ()> {
        let mut users = self
            .user
            .iter()
            .filter(|u| u.id == id)
            .map(|u| u.clone())
            .collect::<Vec<_>>();
        users.push(user.clone());
        self.user = users;
        Ok(user)
    }
}
