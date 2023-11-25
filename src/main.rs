#![warn(clippy::all, clippy::pedantic)]
// use cargo clippy -- -W clippy::pedantic to spot weaknesses/provide documentations in your code
mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    Editor::default().run();
}
