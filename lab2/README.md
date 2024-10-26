# CSE 542S Lab 1

## General Information

1. Lab Number
    1. CSE 542S Fall 2024 Lab 1

2. Student Contact Information
    1. Sam Yoo, <yoosy950@gmail.com>
    2. Alex Kloppenburg, <kloppenburg.alex@gmail.com>
    3. Ben Kim, <bencobble@gmail.com>

3. Program Design Overview
    1. We designed this program in pieces to ensure that we could test accurately over the course of the project.  That meant going a little bit out of order so that we could have some rudimentary output despite not having fully implemented all the pieces of the project.  This approach worked really well, because we were able to capture and correct errors as we found them, rather than running into ten different issues all at once at the end.  I think we could be a little better next lab at working together on our pieces, rather than dividing and conquering, but separating the program into manageable chunks really helped.  We also placed a lot of emphasis on error reporting, which meant that any issuyes we ran into were easier to find and spot.

4. Insights/Observations/Questions
    1. We ran into a few issues with following the flow of data through the various functions.  For example, we had an issue where only the last line of the play was being output.  We first thought it was in `process_config`, then in `add_script_line`, but it turned out the actual issue was in `recite`.  Had we had a better grasp of the program's data flow, we could have solved that a bit easier.

## Usage Instructions

1. How To Unpack
    1. All that needs to be done to unpack these files is to unzip the `lab1` zipped folder.  Everything needed for execution is right there.

2. How To Build
    1. As with any rust package, it can be built with `cargo build` or `cargo run` (which executes `cargo build` automatically).

3. How To Run
    1. The program can be run simply with `cargo run`.

## Testing

1. Description of Testing
    1. For testing, we used a few different types of config and character files.  First, we had a couple of well-formatted ones that we used for initial testing and to ensure proper formatting.  We also had two incorrect config files that we used to confirm our error/warning reporting - one with 3 tokens on each of the character lines, and one that contained a character with no config file.  We also used a character file that was completely empty to check that those were being parsed correctly.  The incorrect files were really good for figuring out where we had holes in our error reporting, or in some cases just figuring out where we had error output that wasn't specific enough.
