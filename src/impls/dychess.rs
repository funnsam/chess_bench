use dychess::prelude::*;

pub struct Perft;

impl Perft {
    fn do_perft(board: &Board, depth: usize) -> u64 {
        let movegen = board.pseudo_legal_moves(&[]);

        if depth == 1 {
            let mut count = 0;

            for m in movegen {
                let mut new = *board;
                new.make_move(m);
                count += !new.is_illegal() as u64;
            }

            return count;
        }

        let mut count = 0;

        for m in movegen {
            let mut new = *board;
            new.make_move(m);

            if !new.is_illegal() {
                count += Self::do_perft(&new, depth - 1);
            }
        }

        count
    }

    fn do_hperft(board: &Board, depth: usize) -> u64 {
        if depth == 0 {
            let white = board.white_pieces().0;
            let black = board.black_pieces().0;

            return white
                .wrapping_mul(crate::HPERFT_WHITE)
                .wrapping_add(black.wrapping_mul(crate::HPERFT_BLACK));
        }

        let mut count: u64 = 0;

        for m in board.pseudo_legal_moves(&[]) {
            let mut new = *board;
            new.make_move(m);

            if !new.is_illegal() {
                count = count.wrapping_add(Self::do_hperft(&new, depth - 1));
            }
        }

        count
    }
}

impl crate::Perft for Perft {
    fn name(&self) -> &'static str {
        "dychess"
    }

    fn perft(&self, fen: &str, depth: usize) -> u64 {
        let board = Board::from_epd(false, fen).expect("invalid fen");
        Self::do_perft(&board, depth)
    }

    fn hperft(&self, fen: &str, depth: usize) -> u64 {
        let board = Board::from_epd(false, fen).expect("invalid fen");
        Self::do_hperft(&board, depth)
    }
}
