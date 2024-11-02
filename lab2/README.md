# CSE 542S Lab 2

## General Information

1. Lab Number
    1. CSE 542S Fall 2024 Lab 2

2. Student Contact Information
    1. Sam Yoo, <yoosy950@gmail.com>
    2. Alex Kloppenburg, <kloppenburg.alex@gmail.com>
    3. Ben Kim, <bencobble@gmail.com>

3. Program Design Overview
    1. We designed this program in pieces to ensure that we could test accurately over the course of the project.  That meant going a little bit out of order so that we could have some rudimentary output despite not having fully implemented all the pieces of the project.  This approach worked really well, because we were able to capture and correct errors as we found them, rather than running into ten different issues all at once at the end.  I think we could be a little better next lab at working together on our pieces, rather than dividing and conquering, but separating the program into manageable chunks really helped.  We also placed a lot of emphasis on error reporting, which meant that any issuyes we ran into were easier to find and spot.
    2. In addition to that, we found it cleaner to keep our test files in a separate directory, so for this lab we added some code based on a piazza post one of our classmates made where he asked if he could take input from any directory.  This was fairly simple to implement, and made our testing a lot more modular, which was very helpful.

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


## Structs (Step 08)
1. Description of Structs
    1. We refactored the code to organize functionality into structs, primarily Play and Player. The Play struct manages the entire script, including loading configuration files and coordinating characters, while each Player struct handles the lines and actions of individual characters. This struct-based design improved modularity and made the code easier to manage. A key challenge was ensuring Play and Player interacted seamlessly to handle dialogue flow; we addressed this by using methods like prepare and speak within Player to keep each struct’s responsibilities clear and maintain data encapsulation.

## Return Wrapper (step 10)
1. Description of Return Wrapper
    1. The ReturnWrapper struct was implemented to handle exit codes and error messages in a consistent way across the program. This struct wraps a single u8 code and implements the Termination trait, allowing us to manage exit codes directly in the main function. If the code is non-zero, ReturnWrapper outputs an error message to stderr and returns the exit code to the shell. In main.rs, we updated the return type to ReturnWrapper and wrapped each Ok and Err with ReturnWrapper::new. This approach simplified error handling and ensured that each stage of the program could report errors consistently to the operating system.

## Scene Fragments (step 15)
1. Description of Scene Fragments
    1. To manage multiple consecutive scene fragments, we designed a SceneFragment struct to represent each part of a play, holding details like scene title, characters, and their lines. The Play struct manages these fragments sequentially, enabling smooth transitions between scenes. Key methods like enter, exit, and recite allow characters to be introduced or removed based on the current and adjacent scenes. Handling Rust’s borrowing rules presented challenges, especially when accessing multiple scene fragments simultaneously. We addressed this by using Rust’s split_at_mut function to create non-overlapping references, allowing flexible yet safe access to mutable data. Additionally, detailed error handling with debug messages was incorporated to manage missing or malformed data files, ensuring robustness. This design effectively organizes and presents each scene in sequence, creating a cohesive, dynamic play structure.

## Testing (step 17)
1. description of Testing
    1. To ensure our program handled multiple scene fragments effectively, we tested it using various script files, configuration files, and part files. The initial tests included provided files like partial_hamlet_act_ii_script.txt along with several configuration and part files (hamlet_ii_1a_config.txt, hamlet_ii_1b_config.txt, etc.), which allowed us to verify the program's ability to parse and handle consecutive scenes. We also created custom files and intentionally modified them with extra tokens, misplaced lines, and out-of-order parts to test error handling.

    2. During testing, we encountered issues such as warnings for unexpected tokens in lines and missing scene titles in fragments, especially in whinge mode, which provided valuable debugging feedback. We addressed these by refining error messages and ensuring that the program could handle varied input gracefully. These tests confirmed that the program could correctly parse, sequence, and recite scenes across a range of input cases.