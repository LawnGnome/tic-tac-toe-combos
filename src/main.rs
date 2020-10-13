use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Empty,
    Cross,
    Naught,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, Default)]
struct Board([Cell; 9]);

impl Board {
    fn is_complete(&self) -> bool {
        // There aren't many winning (or drawing) combinations, so we'll just brute force this.
        if match self.0 {
            [a, b, c, _, _, _, _, _, _] if a == b && a == c && a != Cell::Empty => true,
            [_, _, _, a, b, c, _, _, _] if a == b && a == c && a != Cell::Empty => true,
            [_, _, _, _, _, _, a, b, c] if a == b && a == c && a != Cell::Empty => true,
            [a, _, _, b, _, _, c, _, _] if a == b && a == c && a != Cell::Empty => true,
            [_, a, _, _, b, _, _, c, _] if a == b && a == c && a != Cell::Empty => true,
            [_, _, a, _, _, b, _, _, c] if a == b && a == c && a != Cell::Empty => true,
            [a, _, _, _, b, _, _, _, c] if a == b && a == c && a != Cell::Empty => true,
            [_, _, a, _, b, _, c, _, _] if a == b && a == c && a != Cell::Empty => true,
            [a, b, c, d, e, f, g, h, i]
                if a != Cell::Empty
                    && b != Cell::Empty
                    && c != Cell::Empty
                    && d != Cell::Empty
                    && e != Cell::Empty
                    && f != Cell::Empty
                    && g != Cell::Empty
                    && h != Cell::Empty
                    && i != Cell::Empty =>
            {
                true
            }
            _ => false,
        } {
            return true;
        }

        false
    }

    fn set(&mut self, i: usize, cell: Cell) {
        self.0[i] = cell;
    }
}

fn main() {
    let mut count = 0;
    let mut moves = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut seen = HashSet::new();

    permutohedron::heap_recursive(&mut moves, |p| {
        let mut board = Board::default();

        for (i, &mut pos) in p.into_iter().enumerate() {
            let cell = if i % 2 == 0 {
                Cell::Cross
            } else {
                Cell::Naught
            };
            board.set(pos, cell);

            if board.is_complete() {
                count += 1;
                seen.insert(p[0..i + 1].to_vec());
                break;
            }
        }

        ()
    });

    println!("{}", seen.len());
}
