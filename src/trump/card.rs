use crate::constants::{
    SUIT_STR_HART,
    SUIT_STR_DIAMOND,
    SUIT_STR_CLOVER,
    SUIT_STR_SPADE,

    SUIT_ICON_HART,
    SUIT_ICON_DIAMOND,
    SUIT_ICON_CLOVER,
    SUIT_ICON_SPADE,

    ACE_TO_RANK,
    ACE_FROM_RANK,
    ACE_STR_RANK,

    JACK_FROM_RANK,
    JACK_TO_RANK,
    JACK_STR_RANK,

    QUEEN_FROM_RANK,
    QUEEN_TO_RANK,
    QUEEN_STR_RANK,

    KING_FROM_RANK,
    KING_TO_RANK,
    KING_STR_RANK,

    TWENTY_ONE_NUM,
};

/// カードの情報
#[derive(Debug, Clone)]
pub struct Card {
    suit: String,
    rank: usize,
}

impl Card {
    pub fn new(suit: &str, rank: usize) -> Self {
        Self { suit: suit.to_string(), rank }
    }

    pub fn get_suit(&self) -> &String {
        &self.suit
    }

    pub fn get_rank(&self) -> usize {
        self.rank
    }

    pub fn get_disp_rank(&self) -> String {
        match self.rank {
            ACE_FROM_RANK => ACE_STR_RANK.to_string(),
            JACK_FROM_RANK => JACK_STR_RANK.to_string(),
            QUEEN_FROM_RANK => QUEEN_STR_RANK.to_string(),
            KING_FROM_RANK => KING_STR_RANK.to_string(),
            _ => self.rank.to_string(),
        }
    }

    pub fn get_calc_rank(&self) -> usize {
        match self.rank {
            JACK_FROM_RANK => JACK_TO_RANK,
            QUEEN_FROM_RANK => QUEEN_TO_RANK,
            KING_FROM_RANK => KING_TO_RANK,
            _ => self.rank,
        }
    }

    pub fn ace_rank(&self, total_rank: usize)-> usize {
        if self.rank == ACE_FROM_RANK {
            if self.rank + total_rank <= TWENTY_ONE_NUM {
                return ACE_FROM_RANK;
            } else {
                return ACE_TO_RANK;
            }
        }
        self.rank
    }

    pub fn get_name(&self) -> String {
        let suit = match self.suit.as_str() {
            SUIT_STR_HART => SUIT_ICON_HART,
            SUIT_STR_DIAMOND => SUIT_ICON_DIAMOND,
            SUIT_STR_CLOVER => SUIT_ICON_CLOVER,
            SUIT_STR_SPADE => SUIT_ICON_SPADE,
            _ => "J",
        };

        format!("{}{}", suit, self.get_disp_rank())
    }
}

impl std::fmt::Display for Card {
    /// スート・ランク表示
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
