use super::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

impl GameEngine {
    pub fn new() -> GameEngine{
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };
        engine.initialize_pieces();
        engine
    }

    pub fn initialize_pieces(&mut self) {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2 ,2, 2, 2].iter())
            .map(|(a, b)|(*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::White));
            });

        [0, 2, 4, 6, 1, 3, 5, 7, 0, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(a, b)|(*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][y] = Some(GamePiece::new(PieceColor::Black));
            });
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row]{
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut vmoves = self.valid_moves_from(loc);
                        moves.append(&mut vmoves);
                    }
                }
            }
        }

        moves
    }

    fn valid_moves_from(&self, loc: Coordinate) -> Vec<Move> {
        // Vec::new() // remove after valid_move fn is written
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            let mut jumps = loc
                .jump_targets_from()
                .filter(|t| self.valid_jump(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();
            let mut moves = loc
                .move_targets_from()
                .filter(|t| self.valid_move(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();
            jumps.append(&mut moves);
            jumps
        } else {
            Vec::new()
        }
    }

    fn valid_move(&self, p: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool {
        if !to.on_board() || !from.on_board() {
            false
        } else {
            let Coordinate(tx, ty) = *to;
            if let Some(_piece) = self.board[tx][ty] {
                false
            } else {
                let Coordinate(_fx, fy) = *from;
                let mut valid = false;
                if ty > fy && p.color == PieceColor::White {
                    // white moves down
                    valid = true;
                }
                if ty < fy && p.color == PieceColor::Black {
                    // black moves up
                    valid = true;
                }
                if ty > fy && p.color == PieceColor::Black && p.crowned {
                    // crowned black mv down
                    valid = true;
                }
                if ty < fy && p.color == PieceColor::White && p.crowned {
                    // crowned white mv up
                    valid = true;
                }
                valid

            }
        }
    }

    fn valid_jump(&self, p: &GamePiece, from: &Coordinate, to: &Coordinate) -> bool{
        if !to.on_board() || !from.on_board() {
            false
        } else {
            let Coordinate(x, y) = *from;
            let Coordinate(tx, ty) = *to;

            let midpiece = self.midpiece(x, y, tx, ty);
            match midpiece {
                Some(mp) if mp.color != p.color => true,
                _ => false,
            }
        }
    }

    // not yet implemented
    fn advance_turn(&self) { }

    // not yet implemented
    fn crown_piece(&self, c: Coordinate) { }

    // not yet implemented
    fn should_crown(&self, p: GamePiece, c: Coordinate) -> bool {
        true
    }

    fn midpiece_coordinate(&self, x: usize, y: usize, tx: usize, ty: usize) -> Option<Coordinate> {
        if tx == x + 2 && ty == y + 2 {
            Some(Coordinate(x + 1, y + 1))
        } else if x >= 2 && y >= 2 && tx == x - 2 && ty == y - 2 {
            Some(Coordinate(x - 1, y - 1))
        } else if x >= 2 && tx == x - 2 && ty == y + 2 {
            Some(Coordinate(x - 1, y + 1))
        } else if y >= 2 && tx == x + 2 && ty == y - 2 {
            Some(Coordinate(x + 1, y - 1))
        } else {
            None
        }
    }

    fn midpiece(&self, x: usize, y: usize, tx: usize, ty: usize) -> Option<GamePiece> {
        match self.midpiece_coordinate(x, y, tx, ty) {
            Some(Coordinate(x, y)) => self.board[x][y],
            None => None,
        }
    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        let legal_moves = self.legal_moves();

        if !legal_moves.contains(mv) {
            return Err(());
        }

        let Coordinate(fx, fy) = mv.from;
        let Coordinate(tx, ty) = mv.to;
        let piece = self.board[fx][fy].unwrap();
        let midpiece_coordinate = self
            .midpiece_coordinate(fx, fy, tx, ty);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            self.board[x][y] = None; // remove the jumped piece
        }

        // Move piece from source to dest
        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;

        let crowned = if self.should_crown(piece, mv.to) {
            self.crown_piece(mv.to);
            true
        } else {
            false
        };
        self.advance_turn();

        Ok(MoveResult {
            mv: mv.clone(),
            crowned: crowned,
        })
    }
}