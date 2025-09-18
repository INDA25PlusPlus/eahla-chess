use std::cmp;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


// tiles representation helpers
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKind {
  King,
  Queen,
  Bishop,
  Knight,
  Rook,
  Pawn,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
  White,
  Black,
}

impl Color {
  fn swap(self) -> Color {
    match self {
      Color::White => Color::Black,
      Color::Black => Color::White,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
  pub piece_kind: PieceKind,
  pub color: Color,
}

// impl possible moves? for each piece?

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
  None,
  Occupied(Piece),
}

impl Tile {
  fn make_tile(kind: PieceKind, color: Color) -> Tile {
    Tile::Occupied(Piece { piece_kind: kind, color: color })
  }
}


// history helpers
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
  row: u8,
  column: u8,
}

impl Position {
  pub fn new(row: u8, column: u8) -> Option<Position> {
    // check that the position is a valid position on a chess board (index 0 to 7)
    match row {
      0..8 => {}
      _ => return None
    }
    match column {
      0..8 => {}
      _ => return None
    }
    Some(Position {
      row,
      column,
    })
  }

  fn manhattan_distance(&self, position: Position) -> u8 {
    ((self.row as i8 - position.row as i8).abs() + (self.column as i8 - position.column as i8).abs()) as u8
  }

  // is same row or column
  fn straight(&self, position: Position) -> bool {
    if self.row == position.row {return true;}
    if self.column == position.column {return true;}
    false
  }

  // is on same diagonal
  fn diagonal(&self, position: Position) -> bool {
    (self.row as i8 - position.row as i8).abs() == (self.column as i8 - position.column as i8).abs()
  }

  fn is_valid_king_movement(&self, position: Position) -> bool {
    (self.straight(position) && (self.manhattan_distance(position) == 1)) || (self.diagonal(position) && (self.manhattan_distance(position) == 2))
  }

  fn is_unblocked_straight_movement(&self, position: Position, board: &Board) -> bool {
    if self == &position {
      return false;
    }

    if !self.straight(position) {
      return false;
    }

    let sweep_row = (self.column == position.column);
    let mut sweep_at: u8;
    let mut sweep_stop: u8;
    if sweep_row {
      sweep_at = cmp::min(self.row, position.row) + 1;
      sweep_stop = cmp::max(self.row, position.row);
    } else {
      sweep_at = cmp::min(self.column, position.column) + 1;
      sweep_stop = cmp::max(self.column, position.column);
    }

    while sweep_at < sweep_stop {
      if sweep_row {
        if board.at_position(Position { row: sweep_at, column: self.column }).is_some() {
          return false;
        };
      } else {
        if board.at_position(Position { row: self.row, column: sweep_at }).is_some() {
          return false;
        };
      }

      sweep_at = sweep_at + 1;
    }

    true
  }

  fn is_unblocked_diagonal_movement(&self, position: Position, board: &Board) -> bool {
    if self == &position {
      return false;
    }

    if !self.diagonal(position) {
      return false;
    }

    let start_from_self = self.column < position.column;
    let subtract_row = ((self.column < position.column) && (self.row > position.row)) || ((self.column > position.column) && (position.row > self.row));
    let mut row_at: u8 = if start_from_self && subtract_row {self.row - 1} else if start_from_self {self.row + 1} else if subtract_row {position.row - 1} else {position.row + 1};
    let mut column_at: u8 = if start_from_self {self.column + 1} else {position.column + 1};

    while column_at < cmp::max(self.column, position.column) {
      if board.at_position(Position { row: row_at, column: column_at }).is_some() {
        return false;
      };

      row_at = if subtract_row {row_at - 1} else {row_at + 1};
      column_at = column_at + 1;
    }

    true
  }

  fn is_knight_pattern(self, position: Position) -> bool {
    (((self.row as i8 - position.row as i8).abs() == 1) || ((self.row as i8 - position.row as i8).abs() == 2)) && (self.manhattan_distance(position) == 3)
  }

  // fn is_valid_pawn_move(self, position: Position, turn: Color) -> bool {
  //   // or special eller promo
  //   if turn == Color::White {
  //     if (self.row == position.row + 1) && (self.column == position.column) {
  //       return true;
  //     }

  //     // allow 2 steps forward from beginning row
  //     if ((self.row == 6) && (position.row == 4) && (self.column == position.column)) {
  //       return true;
  //     }

  //     // allow diagonal take
  //     if (self.row == position.row + 1 && self.manhattan_distance(position) == 2 && )
  //   } else {
  //     if (self.row + 1 == position.row) && (self.column == position.column) {
  //       return true;
  //     }

  //     // allow 2 steps forward from beginning row
  //     if ((self.row == 1) && (position.row == 3) && (self.column == position.column)) {
  //       return true;
  //     }
  //   }

  //   false
  // }
}

#[derive(Clone)]
struct HalfMove {
  from: Position,
  to: Position,
  // took something?
  // promotion?
}

// impl verify move?


#[derive(Clone)]
pub struct Board {
  pub history: Vec<HalfMove>,
  pub turn: Color,
  // initialState
  pub board: [[Tile; 8]; 8], // board är skönt att ha
  // turn och initialState behövs för custom-spel
}

impl Board {
  pub fn new() -> Board {
    Board { board: [
      [Tile::make_tile(PieceKind::Rook, Color::Black), Tile::make_tile(PieceKind::Knight, Color::Black), Tile::make_tile(PieceKind::Bishop, Color::Black), Tile::make_tile(PieceKind::Queen, Color::Black), Tile::make_tile(PieceKind::King, Color::Black), Tile::make_tile(PieceKind::Bishop, Color::Black), Tile::make_tile(PieceKind::Knight, Color::Black), Tile::make_tile(PieceKind::Rook, Color::Black)],
      [Tile::make_tile(PieceKind::Pawn, Color::Black); 8],
      [Tile::None; 8],
      [Tile::None; 8],
      [Tile::None; 8],
      [Tile::None; 8],
      [Tile::make_tile(PieceKind::Pawn, Color::White); 8],
      [Tile::make_tile(PieceKind::Rook, Color::White), Tile::make_tile(PieceKind::Knight, Color::White), Tile::make_tile(PieceKind::Bishop, Color::White), Tile::make_tile(PieceKind::Queen, Color::White), Tile::make_tile(PieceKind::King, Color::White), Tile::make_tile(PieceKind::Bishop, Color::White), Tile::make_tile(PieceKind::Knight, Color::White), Tile::make_tile(PieceKind::Rook, Color::White)],
      ], turn: Color::White, history: Vec::new() }
  }

  fn find_king(&self, color: Color) -> Option<Position> {
    let seek: Tile = Tile::Occupied(Piece { piece_kind: PieceKind::King, color: color });
    for row in 0..8 {
      for column in 0..8 {
        if self.board[row][column] == seek {
          return Position::new(row as u8, column as u8);
        }
      }
    }

    None
  }

  pub fn at_position(&self, position: Position) -> Option<Piece> {
    match self.board[position.row as usize][position.column as usize] {
      Tile::Occupied(piece) => Some(piece),
      Tile::None => None,
    }
  }

  fn is_valid_move_ignoring_check(&self, from: Position, to: Position) -> bool {
    // check that there is a piece at position from
    let Some(piece) = self.at_position(from) else {
      return false;
    };
    
    // and that it belongs to the player who's turn it is
    if piece.color != self.turn {return false};

    // check that the Tile at position to is empty or contains a piece of opposite color
    // (not king?) !
    let to_piece = self.at_position(to);
    match to_piece {
      Some(a_piece) => {
        // we cannot take our own piece
        if a_piece.color == self.turn {return false;}
      }
      _ => {}
    }

    match piece.piece_kind {
      PieceKind::King => {
        from.is_valid_king_movement(to)
        // or rockad
      }
      PieceKind::Queen => {
        from.is_unblocked_diagonal_movement(to, self) || from.is_unblocked_straight_movement(to, self)
      }
      PieceKind::Bishop => {
        from.is_unblocked_diagonal_movement(to, self)
      }
      PieceKind::Knight => {
        from.is_knight_pattern(to)
      }
      PieceKind::Rook => {
        from.is_unblocked_straight_movement(to, self)
      }
      PieceKind::Pawn => {
        self.is_valid_pawn_move(from, to)
      }
    }
  }

  fn is_valid_move(&self, from: Position, to: Position) -> bool {
    if !self.is_valid_move_ignoring_check(from, to) {
      return false;
    }

    let mut new_board = self.clone();

    new_board.perform_half_move(from, to);

    let is_check = new_board.is_in_check(self.turn);
    
    if is_check.is_some() {
      return false;
    }

    true
  }

  pub fn is_in_check(&self, color: Color) -> Option<Position> {
    let Some(king_position) = self.find_king(color) else {
      return None;
    };
    for row in 0..8 {
      for column in 0..8 {
        match self.board[row][column] {
          Tile::Occupied(piece) => {
            // for every piece of opposite color, check if they threathen the king of color color
            if piece.color != color {
              if self.is_valid_move_ignoring_check(Position { row: row as u8, column: column as u8 }, king_position) {
                return Some(Position { row: row as u8, column: column as u8 });
              }
            }
          }
          Tile::None => {}
        }
      }
    }

    None
  }

  fn has_no_legal_moves(&self, color: Color) -> bool {
    for row in 0..8 {
      for column in 0..8 {
        match self.board[row][column] {
          Tile::Occupied(piece) => {
            if piece.color == color {
              for row2 in 0..8 {
                for column2 in 0..8 {
                  if self.is_valid_move(Position { row: row as u8, column: column as u8 }, Position { row: row2, column: column2 }) {
                    return false;
                  }
                }
              }
            }
          }
          Tile::None => {}
        }
      }
    }

    true
  }

  pub fn is_check_mate(&self) -> bool {
    let is_in_check = self.is_in_check(self.turn);
    if is_in_check.is_none() {
      return false;
    }

    if self.has_no_legal_moves(self.turn) {
      return true;
    }

    false
  }

  fn perform_half_move(&mut self, from: Position, to: Position) {
    // update board
    self.board[to.row as usize][to.column as usize] = self.board[from.row as usize][from.column as usize];
    self.board[from.row as usize][from.column as usize] = Tile::None;

    // change turn
    self.turn = if self.turn == Color::White { Color::Black } else { Color::White };

    // update hitory
    self.history.push(HalfMove {from, to});
  }

  pub fn make_half_move(&self, from: Position, to: Position) -> Option<Board> {
    if !self.is_valid_move_ignoring_check(from, to) {
      return None;
    }

    let mut new_board = self.clone();

    new_board.perform_half_move(from, to);

    let is_check = new_board.is_in_check(self.turn);
    
    if is_check.is_some() {
      return None;
    }

    return Some(new_board);
  }

  fn is_valid_pawn_move(&self, from: Position, to: Position) -> bool {
    if self.turn == Color::White {
      // normal
      if (from.row == to.row + 1) && (from.column == to.column) && (self.at_position(to).is_none()) {
        return true;
      }

      // allow 2 steps forward from beginning row
      if (from.row == 6) && (to.row == 4) && (from.column == to.column) && (self.at_position(to).is_none()) && (self.at_position(Position { row: to.row + 1, column: to.column }).is_none()) {
        return true;
      }

      // allow diagonal take
      if ((from.row == to.row + 1) && (from.manhattan_distance(to) == 2)) {
        if let Some(piece) = self.at_position(to) {
          if piece.color != self.turn {
            return true;
          }
        }
        return false;
      }

      // en passant
      // todo!
    } else {
      // normal
      if (from.row + 1 == to.row) && (from.column == to.column) && (self.at_position(to).is_none()) {
        return true;
      }

      // allow 2 steps forward from beginning row
      if (from.row == 1) && (to.row == 3) && (from.column == to.column) && (self.at_position(to).is_none()) && (self.at_position(Position { row: to.row - 1, column: to.column }).is_none()) {
        return true;
      }

      // allow diagonal take
      if ((from.row + 1 == to.row) && (from.manhattan_distance(to) == 2)) {
        if let Some(piece) = self.at_position(to) {
          if piece.color != self.turn {
            return true;
          }
        }
        return false;
      }

      // en passant
      // todo!
    }

    false
  }

  // todo!("Add A1 -> 7 0 code");
}


#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn new_board() {
      let new_board = Board::new();
    }

    #[test]
    fn test_some_moves() {
      let board = Board::new();

      // test bad move (wrong player)
      let result = board.make_half_move(Position { row: 1, column: 3 }, Position { row: 3, column: 3 });
      assert!(result.is_none());

      // white move
      let Some(board) = board.make_half_move(Position { row: 6, column: 3 }, Position { row: 4, column: 3 }) else {
        panic!();
      };

      // black move
      let Some(board) = board.make_half_move(Position { row: 1, column: 3 }, Position { row: 3, column: 3 }) else {
        panic!();
      };

      // test bad move
      let result = board.make_half_move(Position { row: 4, column: 3 }, Position { row: 3, column: 3 });
      assert!(result.is_none());

      // test bad move
      let result = board.make_half_move(Position { row: 7, column: 0 }, Position { row: 3, column: 0 });
      assert!(result.is_none());

      // test bad move
      let result = board.make_half_move(Position { row: 7, column: 0 }, Position { row: 7, column: 5 });
      assert!(result.is_none());

      // white move
      let Some(board) = board.make_half_move(Position { row: 7, column: 1 }, Position { row: 5, column: 2 }) else {
        panic!();
      };
    }

    #[test]
    fn test_check_mate() {
      let board = Board::new();

      // white move
      let Some(board) = board.make_half_move(Position { row: 6, column: 5 }, Position { row: 5, column: 5 }) else {
        panic!();
      };

      // black move
      let Some(board) = board.make_half_move(Position { row: 1, column: 4 }, Position { row: 3, column: 4 }) else {
        panic!();
      };

      // white move
      let Some(board) = board.make_half_move(Position { row: 6, column: 6 }, Position { row: 4, column: 6 }) else {
        panic!();
      };

      // black move
      let Some(board) = board.make_half_move(Position { row: 0, column: 3 }, Position { row: 4, column: 7 }) else {
        panic!();
      };

      let check_mate = board.is_check_mate();
      assert!(check_mate);
    }
}


// todo:
// fix pawn moves:
//   2 steps
//   diagonal step
//
//   promotion
//   en passant
//
// rockad
// 
// is_unblocked_diagonal_movement
// 
// add comments
// add tests



// is_unblocked_diagonal_movement ???