/// スート文字
pub const SUIT_STR_HART: &str = "h";
pub const SUIT_STR_DIAMOND: &str = "d";
pub const SUIT_STR_CLOVER: &str = "c";
pub const SUIT_STR_SPADE: &str = "s";
// pub const SUIT_STR_JOKER: &str = "j";

/// スート記号
pub const SUIT_ICON_HART: &str = "♥";
pub const SUIT_ICON_DIAMOND: &str = "♦";
pub const SUIT_ICON_CLOVER: &str = "♣";
pub const SUIT_ICON_SPADE: &str = "♠";
// pub const SUIT_ICON_JOKER: &str = "J";

/// エースのランク（ゲーム内では 11 としても扱う）
pub const ACE_FROM_RANK: usize = 1;
pub const ACE_TO_RANK: usize = 11;
pub const ACE_STR_RANK: &str = "A";

/// 絵札のランク（ゲーム内では 10 として扱う）
pub const JACK_FROM_RANK: usize = 11;
pub const JACK_TO_RANK: usize = 10;
pub const JACK_STR_RANK: &str = "J";

pub const QUEEN_FROM_RANK: usize = 12;
pub const QUEEN_TO_RANK: usize = 10;
pub const QUEEN_STR_RANK: &str = "Q";

pub const KING_FROM_RANK: usize = 13;
pub const KING_TO_RANK: usize = 10;
pub const KING_STR_RANK: &str = "K";

/// 使用デッキ数
pub const NUM_DECKS: usize = 3;
/// ラウンド数
pub const DEFAULT_ROUND_COUNT: usize = 3;
/// CPU人数
pub const CPU_COUNT: usize = 3;
/// ブラックジャック
pub const TWENTY_ONE_NUM: usize = 21;
/// 初期所持チップ
pub const START_CHIP: isize = 10;
pub const MIN_CHIP: isize = 1;
pub const DEFAULT_CHIP: isize = 1;

/// Tab補完候補
pub const CALL_HIT: &str = "hit";
pub const CALL_STAND: &str = "stand";

pub const CALL_WORDS: &[&str] = &[CALL_HIT, CALL_STAND];

/// 外部ファイル読み書き
pub const CSV_PATH: &str = "data.dat";
