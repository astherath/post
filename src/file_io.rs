    use super::errors;
    use shellexpand;
    use std::fmt;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    type DefaultResult = io::Result<()>;
    pub fn add_entry(text: &str) -> DefaultResult {
        write_raw_text_to_file(text)?;
        Ok(())
    }

    fn print_entry_to_console(entry: &Entry) {
        println!("{entry}")
    }

    pub enum Range<'a> {
        Tail(&'a u16),
        Top(&'a u16),
    }

    pub fn view_entries_from_end(range: Range) -> DefaultResult {
        let entries = get_entries_from_file()?;
        let print_n_to_console = |x: Vec<Entry>, num: &u16| {
            x.iter()
                .take(*num as usize)
                .for_each(|y| print_entry_to_console(y))
        };
        match range {
            Range::Top(num) => print_n_to_console(entries, num),
            Range::Tail(num) => print_n_to_console(entries.into_iter().rev().collect(), num),
        }
        Ok(())
    }

    pub fn view_entry_by_index(index: &u16) -> DefaultResult {
        let entries = get_entries_from_file()?;
        errors::check_index_bounds(index, entries.len()).unwrap();
        let entry = entries.iter().find(|x| &x.index == index).unwrap();
        print_entry_to_console(&entry);
        Ok(())
    }

    pub fn delete_entry_from_file_by_index(index: &u16) -> DefaultResult {
        let mut entries = get_entries_from_file()?;
        entries.remove(*index as usize);
        overwrite_entries_to_file(entries)?;
        Ok(())
    }

    fn overwrite_entries_to_file(entries: Entries) -> DefaultResult {
        let all_entries_to_str = entries
            .into_iter()
            .map(|x| x.to_output_string())
            .collect::<Vec<String>>()
            .join("\n");
        write_raw_text_to_file(&all_entries_to_str)?;
        Ok(())
    }

    fn get_entries_from_file() -> io::Result<Entries> {
        Ok(get_lines_from_file()?
            .iter()
            .map(|x| Entry::from_str(x).unwrap())
            .collect())
    }

    fn get_lines_from_file() -> io::Result<Vec<String>> {
        let lines = fs::read_to_string(get_file_path())?
            .lines()
            .map(|x| x.to_string())
            .collect();

        Ok(lines)
    }

    fn write_raw_text_to_file(text: &str) -> DefaultResult {
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
        let filename = "";
        path.push(PathBuf::from(filename));
        path
    }

    fn get_dir_path() -> PathBuf {
        let dir_path_str = "~/.config/post/";
        PathBuf::from(shellexpand::tilde(dir_path_str).to_string())
    }

    type Entries = Vec<Entry>;
    struct Entry {
        index: u16,
        content: String,
    }
    impl Entry {
        fn from_str(data: &str) -> Result<Self, !> {
            let char_index_resp = data.chars().position(|c| c == '|');
            if let None = char_index_resp {
                errors::handle_entry_from_str_error("");
            }
            let (index_str, content_str) = data.split_at(char_index_resp.unwrap());
            let index = match index_str.parse::<u16>() {
                Ok(index) => index,
                Err(reason) => errors::handle_entry_from_str_error(format!(
                    "bad index number parse: {reason:?}"
                )),
            };
            Ok(Self {
                index,
                content: content_str.to_string(),
            })
        }

        fn to_output_string(self) -> String {
            format!("{}|{}", self.index, self.content)
        }
    }
    impl fmt::Display for Entry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} | {}", self.index, self.content)
        }
    }
