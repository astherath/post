## 📝 📋 post 

A minimalist note taking tool to keep clipboard crumbs - a post-it note for your terminal

# Table of contents

- [Installation](#installation)
- [Usage](#usage)
    - [Common usage example](#common-usage-example)

# Installation

- Install from [crates.io](https://crates.io/crates/post-it)

    - `cargo install post-it`

- Build manually from source
    ```sh
    $ git clone https://github.com/astherath/post
    $ cd post
    $ cargo install --path=.
    ```

# Usage

`post` works best with single line notes and other small strings

```
post 1.0.0
a simple cli to keep and move notes in/out of the clipboard

USAGE:
    post <SUBCOMMAND>

OPTIONS:
    -h, --help
            Print help information

    -V, --version
            Print version information

SUBCOMMANDS:
    add
            Adds a note to the stack
    clear
            Deletes many notes at once
    delete
            Deletes a note
    help
            Print this message or the help of the given subcommand(s)
    pop
            Yanks the contents of a note and then deletes it
    view
            Views the notes in the stack (if no argument given, views the lates 10 notes)
    yank
            Copies the text from a note onto the clipboard

```

## Common usage example

```bash
# we can add notes with "add"
post add "this is my first note"
> added note to position "0"

# "view" defaults to showing the latest 10 notes
post view
> 0 | this is my first note

# we can add multiple notes and track their index
post add "another note"
> added note to position "1"

post view
> 0 | this is my first note
> 1 | another note

# top and tail flags change the starting point
post view --top=1
> 0 | this is my first note

post view --tail=1
> 1 | another note

# yank the note content at the given index to the clipboard
post yank 0
> yanked entry at index 0

# delete removes the note from the tracking table
post delete 0
> deleted entry at index 0

# pop is shorthand for yank + delete a note
post pop 0
> yanked entry at index 0
> deleted entry at index 0

# clear allows for bulk note removal
post clear --all
> cleared 1 entries from file
```
