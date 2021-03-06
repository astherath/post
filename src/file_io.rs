use super::errors;
use clipboard::{ClipboardContext, ClipboardProvider};
use errors::ClapIoError;

use std::path::PathBuf;
use std::{fmt, fs, io};

type IoResult = Result<(), errors::ClapIoError>;

pub fn add_entry(text: &str, comment: Option<String>) -> IoResult {
    let mut entries = get_entries_from_file()?;
    entries.add_entry_from_str(text, comment);
    println!("added note to position \"{}\"", entries.0.len() - 1);
    overwrite_entries_to_file(entries)?;
    Ok(())
}

fn print_entry_to_console(entry: &Entry, no_fmt: bool) {
    match no_fmt {
        false => println!("{}", entry),
        true => println!("{:#?}", entry),
    }
}

pub enum Range<'a> {
    Tail(&'a u16),
    Top(&'a u16),
}

fn print_no_notes_msg() {
    println!("(no notes to view! add one first with \"post\")");
}

pub fn view_entries_from_end(range: Range, no_fmt: bool) -> IoResult {
    let entries = get_entries_from_file()?.0;
    if entries.is_empty() {
        print_no_notes_msg();
        return Ok(());
    }

    let print_to_console = |x: Vec<Entry>| x.iter().for_each(|x| print_entry_to_console(x, no_fmt));
    match range {
        Range::Top(num) => print_to_console(entries.into_iter().take(*num as usize).collect()),
        Range::Tail(num) => {
            let tail_entries = entries
                .into_iter()
                .rev()
                .take(*num as usize)
                .rev()
                .collect();
            print_to_console(tail_entries);
        }
    }
    Ok(())
}

pub fn view_all_entries(no_fmt: bool) -> IoResult {
    let entries = get_entries_from_file()?.0;
    if entries.is_empty() {
        print_no_notes_msg();
        return Ok(());
    }
    entries
        .iter()
        .for_each(|x| print_entry_to_console(x, no_fmt));
    Ok(())
}

pub fn view_entry_by_index(index: &u16, no_fmt: bool) -> IoResult {
    let entry = find_entry_by_index(index)?;
    print_entry_to_console(&entry, no_fmt);
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
    // should be able to clear even if file is corrupted
    let entries_resp = get_entries_from_file();
    let mut msg = "reformatted data file and cleared all entries".to_string();
    if let Ok(entries) = entries_resp {
        if entries.0.is_empty() {
            print_no_notes_msg();
            return Ok(());
        } else {
            msg = format!("cleared {} entries from file", entries.0.len());
        }
    }
    let entries = Entries::empty();
    overwrite_entries_to_file(entries)?;
    println!("{}", msg);
    Ok(())
}

pub fn clear_from_end(range: Range) -> IoResult {
    let entries = get_entries_from_file()?.0;
    if entries.is_empty() {
        print_no_notes_msg();
        return Ok(());
    }
    let initial_len = entries.len();
    let clear_n_entries =
        |x: Vec<Entry>, num: &u16| -> Vec<Entry> { x.into_iter().skip(*num as usize).collect() };
    let new_entries = Entries::from_entries(match range {
        Range::Top(num) => clear_n_entries(entries, num),
        Range::Tail(num) => clear_n_entries(entries.into_iter().rev().collect(), num),
    });
    let amount_of_removed_entries = initial_len - new_entries.0.len();
    overwrite_entries_to_file(new_entries)?;
    println!("cleared {} entries from file", amount_of_removed_entries);
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

pub fn backup_data_file(dest_path: &mut PathBuf) -> IoResult {
    if !check_if_file_exists() {
        print_no_notes_msg();
        return Ok(());
    }
    if !dest_path.is_dir() {
        return Err(ClapIoError::from(format!(
            "Backup of data file failed: {}",
            "destination path does not exist"
        )));
    }
    dest_path.push(format!("backup-{}-{}", get_date(), get_filename()));
    let success_string = format!("backup to {:#?} complete", &dest_path);
    fs::copy(get_file_path(), dest_path)?;
    println!("{}", success_string);
    Ok(())
}

fn overwrite_entries_to_file(entries: Entries) -> IoResult {
    let all_entries_to_str = entries.into_output_string();
    write_raw_text_to_file(&all_entries_to_str)?;
    Ok(())
}

fn get_entries_from_file() -> Result<Entries, errors::ClapIoError> {
    if !check_if_file_exists() {
        create_dir_and_empty_file()?;
    }
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

fn create_dir_and_empty_file() -> IoResult {
    // we can move this use into here because large import and rarely called
    use std::io::prelude::*;
    create_dir_if_none_exists()?;
    let mut file = fs::File::create(get_file_path())?;
    file.write_all("".as_bytes())?;
    Ok(())
}

fn write_raw_text_to_file(text: &str) -> IoResult {
    if !check_if_file_exists() {
        create_dir_and_empty_file()?;
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
    let filename = get_filename();
    path.push(PathBuf::from(filename));
    path
}

fn get_date() -> String {
    format!("{}", chrono::offset::Local::now())
}

fn get_filename() -> &'static str {
    "notes.txt"
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
    fn add_entry_from_str(&mut self, entry_text: &str, comment: Option<String>) {
        let index = self.0.len();
        let entry = Entry::new(index as u16, entry_text.to_string(), comment);
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
    pub comment: Option<String>,
}

impl Entry {
    fn new(index: u16, content: String, comment: Option<String>) -> Self {
        Self {
            index,
            content,
            comment,
        }
    }

    fn from_str(data: &str) -> Result<Self, String> {
        let char_index_resp = data.chars().position(|c| c == '|');
        if char_index_resp.is_none() {
            return Err("entry data is not in valid format; reformat file with \"clear --all\" and try again".to_string());
        }
        let (index_str, content_str) = data.split_at(char_index_resp.unwrap());
        let index_resp = index_str.parse::<u16>();
        if let Err(reason) = index_resp {
            return Err(format!("bad index number parse: {reason:?}"));
        };
        let main_string: String = content_str.chars().skip(1).collect();
        let content_string: String;
        let mut comment: Option<String> = None;

        if let Some(index) = main_string.chars().position(|c| c == '??') {
            let (content_str, comment_str) = main_string.split_at(index);
            comment = Some(comment_str.chars().skip(1).collect());
            content_string = content_str.to_string();
        } else {
            content_string = main_string;
        }

        Ok(Self {
            content: content_string,
            index: index_resp.unwrap(),
            comment,
        })
    }

    fn into_output_string(self) -> String {
        let comment_string = {
            if self.comment.is_some() {
                format!("??{}", &self.comment.unwrap())
            } else {
                "".to_string()
            }
        };
        format!("{}|{}{}", self.index, self.content, comment_string)
    }
}
impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let comment_string = match &self.comment {
            Some(comment_str) => format!("\t# {}", comment_str),
            None => "".to_string(),
        };
        write!(f, "{:<3}| {}{}", self.index, self.content, comment_string)
    }
}
impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
