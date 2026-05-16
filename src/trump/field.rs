use crate::{Card};

/// ルームカード
struct RoomCard(Vec<Card>);
impl RoomCard {
    fn add(&mut self, card: Card) {
        self.0.push(card);
    }

    fn get(&mut self) -> &mut Vec<Card> {
        &mut self.0
    }

    fn remove(&mut self, index: usize) -> Card {
        self.0.remove(index)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

/// ゲームフィールド
pub struct Field {
    room: RoomCard,
    discard: Vec<Card>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            room: RoomCard(vec![]),
            discard: vec![],
        }
    }

    pub fn add_room_card(&mut self, card: Card) {
        self.room.add(card);
    }

    pub fn get_room(&mut self) -> &mut Vec<Card> {
        self.room.get()
    }

    pub fn get_room_len(&self) -> usize {
        self.room.len()
    }

    pub fn get_room_index(&mut self, index: usize) -> Option<Card> {
        let card = self.room.remove(index);
        Some(card)
    }

    pub fn room_clear(&mut self) {
        self.room.clear();
    }

    /// 捨て札をまとめて参照
    pub fn get_discards(&self) -> Vec<String> {
        let mut discards: Vec<String> = Vec::new();
        for card in &self.discard {
            discards.push(format!("{}", card));
        }
        discards
    }
}
