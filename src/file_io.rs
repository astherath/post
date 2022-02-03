use super::errors;
use clipboard::{ClipboardContext, ClipboardProvider};
use errors::ClapIoError;

use std::path::PathBuf;
use std::{fmt, fs, io};

type IoResult = Result<(), errors::ClapIoError>;

pub fn add_entry(text: &str) -> IoResult {
    let mut entries = get_entries_from_file()?;
    entries.add_entry_from_str(text);
    println!("added note to position \"{}\"", entries.0.len());
    overwrite_entries_to_file(entries)?;
    Ok(())
}

fn print_entry_to_console(entry: &Entry) {
    println!("{entry}")
}

pub enum Range<'a> {
    Tail(&'a u16),
    Top(&'a u16),
}

fn print_no_notes_msg() {
    println!("(no notes to view! add one first with \"post\")");
}

pub fn view_entries_from_end(range: Range) -> IoResult {
    let entries = get_entries_from_file()?.0;
    if entries.is_empty() {
        print_no_notes_msg();
        return Ok(());
    }

    let print_n_to_console = |x: Vec<Entry>, num: &u16| {
        x.iter()
            .take(*num as usize)
            .for_each(print_entry_to_console)
    };
    match range {
        Range::Top(num) => print_n_to_console(entries, num),
        Range::Tail(num) => print_n_to_console(entries.into_iter().rev().collect(), num),
    }
    Ok(())
}

pub fn view_entry_by_index(index: &u16) -> IoResult {
    let entry = find_entry_by_index(index)?;
    print_entry_to_console(&entry);
    Ok(())
}

fn find_entry_by_index(index: &u16) -> Result<Entry, ClapIoError> {
    let entries = get_entries_from_file()?.0;
    errors::check_index_bounds(index, entries.len())?;
    let entry = entries.into_iter().find(|x| &x.index == index).unwrap();
    Ok(entry)
}

pub fn handle_pop_entry(index: &u16) -> IoResult {
    yank_note(index)?;
    delete_entry_from_file_by_index(index)?;
    Ok(())
}

pub fn delete_entry_from_file_by_index(index: &u16) -> IoResult {
    let mut entries = get_entries_from_file()?;
    errors::check_index_bounds(index, entries.0.len())?;
    let entry = &entries.0[*index as usize];
    let entry_removed_msg = format!("deleted entry at index {}", entry.index);
    entries.remove_entry_at_index(index);
    overwrite_entries_to_file(entries)?;
    println!("{}", entry_removed_msg);
    Ok(())
}

pub fn clear_all_entries() -> IoResult {
    if get_entries_from_file()?.0.is_empty() {
        print_no_notes_msg();
        return Ok(());
    }
    let entries = Entries::empty();
    overwrite_entries_to_file(entries)?;
    Ok(())
}

pub fn clear_from_end(range: Range) -> IoResult {
    let entries = get_entries_from_file()?.0;
    if entries.is_empty() {
        print_no_notes_msg();
        return Ok(());
    }
    let clear_n_entries =
        |x: Vec<Entry>, num: &u16| -> Vec<Entry> { x.into_iter().skip(*num as usize).collect() };
    let new_entries = Entries::from_entries(match range {
        Range::Top(num) => clear_n_entries(entries, num),
        Range::Tail(num) => clear_n_entries(entries.into_iter().rev().collect(), num),
    });
    overwrite_entries_to_file(new_entries)?;
    Ok(())
}

pub fn yank_note(index: &u16) -> IoResult {
    let mut ctx: ClipboardContext = match ClipboardProvider::new() {
        Ok(ctx) => ctx,
        Err(error) => {
            return Err(ClapIoError::from(format!(
                "Copying note text to clipboard failed: {error}"
            )))
        }
    };
    let entry = find_entry_by_index(index)?;
    ctx.set_contents(entry.content)?;
    println!("yanked entry at index {}", entry.index);
    Ok(())
}

fn overwrite_entries_to_file(entries: Entries) -> IoResult {
    let all_entries_to_str = entries.into_output_string();
    write_raw_text_to_file(&all_entries_to_str)?;
    Ok(())
}

fn get_entries_from_file() -> Result<Entries, errors::ClapIoError> {
    let mut entries = vec![];
    for entry in get_lines_from_file()?.iter() {
        let parse_response = Entry::from_str(entry)?;
        entries.push(parse_response);
    }
    Ok(Entries::from_entries(entries))
}

fn get_lines_from_file() -> io::Result<Vec<String>> {
    let lines = fs::read_to_string(get_file_path())?
        .lines()
        .map(|x| x.to_string())
        .collect();

    Ok(lines)
}

fn write_raw_text_to_file(text: &str) -> IoResult {
    if !check_if_file_exists() {
        create_dir_if_none_exists()?;
    }
    Ok(fs::write(get_file_path(), text)?)
}

fn check_if_file_exists() -> bool {
    get_file_path().exists()
}

fn create_dir_if_none_exists() -> io::Result<()> {
    let dir_path = get_dir_path();
    fs::create_dir_all(dir_path)?;
    Ok(())
}

fn get_file_path() -> PathBuf {
    let mut path = get_dir_path();
    let filename = "notes.txt";
    path.push(PathBuf::from(filename));
    path
}

fn get_dir_path() -> PathBuf {
    let dir_path_str = "~/.config/post/";
    PathBuf::from(shellexpand::tilde(dir_path_str).to_string())
}

struct Entries(Vec<Entry>);
impl Entries {
    fn from_entries(entries: Vec<Entry>) -> Self {
        Self(entries)
    }
    fn add_entry_from_str(&mut self, entry_text: &str) {
        let index = self.0.len();
        let entry = Entry::new(index as u16, entry_text.to_string());
        self.0.push(entry);
    }
    fn into_output_string(self) -> String {
        self.0
            .into_iter()
            .map(|x| x.into_output_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn empty() -> Self {
        Self(vec![])
    }
    fn remove_entry_at_index(&mut self, index: &u16) {
        self.0.remove(*index as usize);
        // move all the indexes
        let mut counter = 0;
        self.0.iter_mut().for_each(|mut x| {
            x.index = counter;
            counter += 1;
        });
    }
}

struct Entry {
    pub index: u16,
    pub content: String,
}
impl Entry {
    fn new(index: u16, content: String) -> Self {
        Self { index, content }
    }
    fn from_str(data: &str) -> Result<Self, String> {
        let char_index_resp = data.chars().position(|c| c == '|');
        if char_index_resp.is_none() {
            return Err("line is not in valid format, ensure config file is correct".to_string());
        }
        let (index_str, content_str) = data.split_at(char_index_resp.unwrap());
        let index_resp = index_str.parse::<u16>();
        if let Err(reason) = index_resp {
            return Err(format!("bad index number parse: {reason:?}"));
        };
        Ok(Self {
            content: content_str.chars().skip(1).collect(),
            index: index_resp.unwrap(),
        })
    }

    fn into_output_string(self) -> String {
        format!("{}|{}", self.index, self.content)
    }
}
impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", self.index, self.content)
    }
}
