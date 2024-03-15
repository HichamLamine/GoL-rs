use crossterm::terminal;

fn main() {
    let (cols, rows) = terminal::size().unwrap();
    let game_buffer: Vec<Vec<char>> = vec![vec![' '; cols as usize]; rows as usize];

}
