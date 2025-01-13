# Console-Based Text Editor

## Overview

As an introduction to the Rust language, I decided to make a console-based text editor, similar to nano or Vim. It currently offers support for viewing and editing documents that contain all UTF-8 characters, including emojis and alternate languages as well! It also has the added feature of syntax highlighting for keywords, characters, strings, and comments.
## Implementation

There is *so much* going on under the hood of console-based text editors, but it can be broken down into a few key components:

1. Entering raw mode in the terminal
2. Listening for key presses and handling them accordingly
3. Using OOP to abstract the idea of a file
4. Rendering/editing the file by interacting with the terminal

### *How does it work?*

### Entering Raw Mode
When you use the console during the execution of a program, you may notice that the console waits for the `Enter` key to be pressed before processing the characters. Additionally, all the characters you type show up on the line that you are typing on and pressing `Ctrl+C` completely halts the execution of the program. When editing a text file, *none* of these features are desirable. That's where terminal raw mode comes in. It process key presses immediately, does not print characters that are pressed, and allows the programmer to handle control key processing.

### Processing Key Presses
To process key presses, the program listens for keyboard input and performs a command accordingly. For example, a regular character would prompt the editor to insert the key in the current position and move to the right, a control character, for example `Ctrl+Q` exits the text editor, etc.

### Abstracting a File
A file is simply a series of lines, and each line is a series of characters. When a file is opened in the text editor, it is handled exactly as such. To modify a specific line, we can alter or append a character to a line and then write each individual line back to the file.

### Rendering/Editing with the Terminal
In raw mode, we can manually move the cursor and print individual strings to the terminal. As a result, we can simply print out the given file, line by line, then listen for presses of the arrow keys to manually move the cursor around the terminal. The user can then enter characters at any specific location in the document.

## How to Use The Program

    cargo run --release sample_file

You can replace `sample_file` with your desired file to view/edit it!