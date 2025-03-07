use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use termgame::{
    run_game, CharChunkMap, Controller, Game, GameEvent, GameSettings, KeyCode, SimpleEvent,
};

/// This is a single "buffer".
struct Buffer {
    text: String,
}

impl Buffer {
    /// This creates a new Buffer, to use it you should run:
    /// ```rust
    /// Buffer::new()
    /// ```
    fn new() -> Buffer {
        Buffer {
            text: String::new(),
        }
    }

    /// A [`CharChunkMap`] is how termgame stores characters.
    /// This converts a buffer into something which can be shown on screen.
    /// You will likely not need to change this function.
    fn chunkmap_from_textarea(&mut self, map: &mut CharChunkMap) {
        let (mut line, mut col) = (0, 0);
        for c in self.text.chars() {
            map.insert(col, line, c.into());
            col += 1;
            if c == '\n' {
                line += 1;
                col = 0;
            }
        }
    }

    /// Adds a char to the end of the buffer.
    fn push_char(&mut self, c: char) {
        self.text.push(c);
    }

    /// Removes the last char in the buffer.
    fn pop_char(&mut self) {
        self.text.pop();
    }

    fn search(&self, search_text: &str) -> Vec<(usize, &str)> {
        self.text.lines()
            .enumerate()
            .filter(|(_, line)| line.contains(search_text))
            .map(|(i, line)| (i + 1, line)) // +1 for 1-based line numbering
            .collect()
    }

    // /// This is an example of a function that takes the Buffer as owned,
    // /// as well as another text area; and returns a new Buffer.
    // /// You would either need to return a `Buffer`, or be sure that
    // /// the user will not want the `Buffer` anymore.
    // fn example_owned(self, another_arg: Buffer) -> Buffer {
    //     todo!()
    // }

    // /// This is an example of a function that takes the Buffer by
    // /// mutable reference.
    // fn example_ref_mut(&mut self, another_arg: i32) {
    //     todo!()
    // }

    // /// This is an example of a function that takes the Buffer by
    // /// reference.
    // fn example_ref(&self) -> i32 {
    //     todo!()
    // }
}

/// This struct implements all the
/// logic for how the editor should work. It
/// implements "Controller", which defines how
/// something should interact with the terminal.
struct BufferEditor {
    buffers: HashMap<String, Buffer>, // use string as the key to the buffer
    active_buffer: String,            // track the currently active buffer
}

impl BufferEditor {
    /// Create a new BufferEditor with a default buffer
    fn new() -> Self {
        let mut buffers = HashMap::new();
        let default_name = "default".to_string();
        buffers.insert(default_name.clone(), Buffer::new());

        BufferEditor {
            buffers,
            active_buffer: default_name,
        }
    }

    /// Get the active buffer
    fn get_active_buffer(&mut self) -> &mut Buffer {
        self.buffers.get_mut(&self.active_buffer).unwrap()
    }

    /// Open a buffer with the given name, creating it if it doesn't exist
    fn open_buffer(&mut self, name: &str) {
        if !self.buffers.contains_key(name) {
            self.buffers.insert(name.to_string(), Buffer::new());
        }
        self.active_buffer = name.to_string();
    }

    // fn search_buffer(&mut self, search_str: &str) {
    //     for (buffer_name, buffer_content) in &self.buffers {
    //         if buffer_content.text.contains(search_str) {
    //             println!("buffer_name:{} {}", buffer_name, buffer_content.text);
    //         }
    //     }
    // }
    fn search_buffer(&self, text: &str) {
        for (buffer_name, buffer) in &self.buffers {
            let matches = buffer.search(text);
            for (line_num, line) in matches {
                println!("{}:{} {}", buffer_name, line_num, line);
            }
        }
    }
}

impl Controller for BufferEditor {
    /// This gets run once, you can probably ignore it.
    fn on_start(&mut self, game: &mut Game) {
        let mut chunkmap = CharChunkMap::new();
        self.get_active_buffer().chunkmap_from_textarea(&mut chunkmap);
        game.swap_chunkmap(&mut chunkmap);
    }

    /// Any time there's a keypress, you'll get this
    /// function called.
    fn on_event(&mut self, game: &mut Game, event: GameEvent) {
        match event.into() {
            SimpleEvent::Just(KeyCode::Char(c)) => self.get_active_buffer().push_char(c),
            SimpleEvent::Just(KeyCode::Enter) => self.get_active_buffer().push_char('\n'),
            SimpleEvent::Just(KeyCode::Backspace) => self.get_active_buffer().pop_char(),
            SimpleEvent::Just(KeyCode::Esc) => {
                game.end_game();
            }
            SimpleEvent::Just(KeyCode::Up) => {
                let mut viewport = game.get_viewport();
                if viewport.y > 0 {
                    viewport.y -= 1;
                }
                game.set_viewport(viewport)
            }
            SimpleEvent::Just(KeyCode::Down) => {
                let mut viewport = game.get_viewport();
                viewport.y += 1;
                game.set_viewport(viewport)
            }
            _ => {}
        }
        let mut chunkmap = CharChunkMap::new();
        self.get_active_buffer().chunkmap_from_textarea(&mut chunkmap);
        game.swap_chunkmap(&mut chunkmap);
    }

    /// This function gets called regularly, so you can use it
    /// for logic that's independent of key-presses like
    /// implementing a "mouse".
    fn on_tick(&mut self, _game: &mut Game) {}
}

fn run_command(editor: &mut BufferEditor, cmd: &str) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    match parts[0] {
        // implement open: edit struct BufferEditor and add impl BufferEditor
        "open" => {
            if parts.len() > 1 {
                editor.open_buffer(parts[1]);
            }
            run_game(
                editor,
                GameSettings::new().tick_duration(Duration::from_millis(25)),
            )?
        },
        "search" => {
            if parts.len() > 1 {
                let query = parts[1..].join(" ");
                editor.search_buffer(&query);
            } else {
                eprintln!("Error: No search term provided.");
            }
        }
        _ => println!("Command not recognised!"),
    }

    Ok(())
}

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to BuffeRS. ");
    // println!("Available commands: open [name], list, switch [name]");

    // you're only creating one buffer
    // let mut editor = BufferEditor {
    //     buffer: Buffer::new()
    // };

    // make a bunch of editors
    let mut editor = BufferEditor::new();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                run_command(&mut editor, &line)?;
                rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
