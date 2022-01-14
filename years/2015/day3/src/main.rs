#[derive(Debug, Default, PartialEq)]
struct Position(i8, i8);

impl Position {
    #[inline(always)]
    fn up(&mut self) {
        self.1 += 1
    }

    #[inline(always)]
    fn right(&mut self) {
        self.0 += 1
    }

    #[inline(always)]
    fn down(&mut self) {
        self.1 -= 1
    }

    #[inline(always)]
    fn left(&mut self) {
        self.0 -= 1
    }
}

impl Clone for Position {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

fn main() {
    let input = utils::input_bytes!("1");
    let mut pos = Position::default();
    let mut visited =  Vec::<Position>::with_capacity(input.len() + 1);

    input.iter().for_each(|dir| {
        match dir {
            b'^' => pos.up(),
            b'>' => pos.right(),
            b'v' => pos.down(),
            b'<' => pos.left(),
            _ => return
        }

        if !visited.iter().any(|visited| *visited == pos) {
            visited.insert(0, pos.clone());
        }
    });

    println!("{}", visited.len())
}
