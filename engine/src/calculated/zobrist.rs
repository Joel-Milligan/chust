use std::sync::LazyLock;

pub static ZOBRIST_SQUARES: LazyLock<[[[u64; 6]; 2]; 64]> =
    LazyLock::new(rand::random::<[[[u64; 6]; 2]; 64]>);
pub static ZOBRIST_BLACK: LazyLock<u64> = LazyLock::new(rand::random);
pub static ZOBRIST_WHITE_KING_CASTLING: LazyLock<u64> = LazyLock::new(rand::random);
pub static ZOBRIST_WHITE_QUEEN_CASTLING: LazyLock<u64> = LazyLock::new(rand::random);
pub static ZOBRIST_BLACK_KING_CASTLING: LazyLock<u64> = LazyLock::new(rand::random);
pub static ZOBRIST_BLACK_QUEEN_CASTLING: LazyLock<u64> = LazyLock::new(rand::random);
pub static ZOBRIST_EN_PASSANT: LazyLock<[u64; 8]> = LazyLock::new(rand::random::<[u64; 8]>);
