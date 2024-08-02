# Lock In

Version: 0.1.1

A simple Rust CLI tool that combines a focus timer with a list of focus items. Sometimes you just need to lock in and get work done. This helps you stay focused on the CLI.

## Features

- 45-minute focus timer
- Manage a list of 3 focus items
- ASCII art banner display
- Persistent storage of focus items

## Recent Updates (v0.1.1)

- Added ability to clear focus items list
- Implemented 'clear' command at startup and before timer begins
- Enhanced user interaction with command prompts

## Usage

1. Run the program
2. Choose to clear items or continue
3. View or update focus items
4. Start the timer or clear items
5. Focus on your tasks for 25 minutes

## Commands

- `clear`: Removes all focus items
- `start`: Begins the 25-minute timer

## Note

Focus items are stored in `focus_items.txt` in the same directory as the executable.
