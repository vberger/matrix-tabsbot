use std::collections::HashMap;

use super::format_amount;

pub enum SearchError {
    Ambiguous,
    NotFound,
}

/// Stores the balance of the users of a room, in centimes
#[derive(Serialize, Deserialize)]
pub struct RoomTab {
    pub users: HashMap<String, i32>,
}

impl RoomTab {
    pub fn new() -> RoomTab {
        RoomTab {
            users: HashMap::new(),
        }
    }

    /// Shift the mean to ensure total balance in the room
    /// is near 0
    ///
    /// (there can be rounding issues depending on the number of
    /// people in the room)
    pub fn rebalance(&mut self) {
        let sum: i32 = self.users.values().sum();
        let mean = sum / self.users.len() as i32;
        for v in self.users.values_mut() {
            *v -= mean;
        }
    }

    pub fn format_balance(&self) -> String {
        let mut s: String = "Current room balance:".into();
        for (user, balance) in self.users.iter() {
            s.push_str(&format!("\n - {}: {}", user, format_amount(*balance)));
        }
        s
    }

    pub fn find_user(&self, txt: &str) -> Result<String, SearchError> {
        let mut result = Err(SearchError::NotFound);
        for name in self.users.keys() {
            if name.contains(txt) {
                if let Err(SearchError::NotFound) = result {
                    result = Ok(name.clone());
                } else {
                    return Err(SearchError::Ambiguous);
                }
            }
        }
        result
    }
}

pub struct TabStore {
    pub rooms: HashMap<String, RoomTab>,
}

impl TabStore {
    pub fn new() -> TabStore {
        TabStore {
            rooms: HashMap::new(),
        }
    }

    pub fn restore(&mut self, room: String, tab: RoomTab) {
        self.rooms.insert(room, tab);
    }

    pub fn get(&self, room: &str) -> Option<&RoomTab> {
        self.rooms.get(room)
    }

    pub fn pay(&mut self, amount: i32, room: String, user: String) {
        let stored = self.rooms
            .entry(room)
            .or_insert_with(|| RoomTab::new())
            .users
            .entry(user)
            .or_insert(0);
        *stored += amount;
    }

    pub fn payto(
        &mut self,
        amount: i32,
        room: String,
        user: String,
        search: &str,
    ) -> Result<String, SearchError> {
        let room = self.rooms.entry(room).or_insert_with(|| RoomTab::new());
        let other = room.find_user(search)?;
        *(room.users.entry(user).or_insert(0)) += amount;
        *(room.users.entry(other.clone()).or_insert(0)) -= amount;
        Ok(other)
    }

    pub fn balance(&self, room: &str) -> String {
        self.rooms
            .get(room)
            .map(|r| r.format_balance())
            .unwrap_or_else(|| "The tab of this room is currently empty.".into())
    }

    pub fn rebalance(&mut self, room: &str) {
        if let Some(tab) = self.rooms.get_mut(room) {
            tab.rebalance();
        }
    }
}
