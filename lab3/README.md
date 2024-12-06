# CSE 542 Fall 2024 Lab 3 #

## Team members: ##
Alex Kloppenburg, <kloppenburg.alex@gmail.com><br />
Ben Kim, <bencobble@gmail.com><br />
Sam Yoo, <yoosy950@gmail.com><br />

## Overview ##
This lab was used to highlight the thread and memory safety guarentees that come with writing code in Rust. First, we updated all of our println! and eprintln! macros to use the writeln! macro and lock the stdout/stderr before writing to them. This guarentees that only one thread is able to write at the same time. We also wrapped all objects/references that we wanted to pass to different threads in the `Arc` type. The Atomic Reference Counter allows us to pass around immutable objects and references safely between threads without having to worry about causing undefined behavior. Passing mutable objects and references safely between different threads required us to wrap the object or reference in both the `Arc` type and the `Mutex` type. The `Mutex` is what allows the object to be mutable as the coder must lock before updating the object so that only one thread can update it at a time. We coded multithreaded file IO as well as a multithreaded server that kicked off a new thread every time a new client connected. New threads were created by using the `spawn()` function and were joined together at certain points with the `join()` function. With these server and client code, we were able to code up a small server that would be able to communicate and send/receive data through TCP connections with its clients. Clients are now able to print out scripts in order that are found remotely in the server or locally on the client side.

## Insights, Observations, Questions ##
Some insights and observations that we had were that it was a lot easier than we thought it would be to make sure that objects and references would be thread safe. Once we figured out whether they were going to be mutable or immutable all we had to do was surround them with the respective `Arc` and `Mutex` or just `Arc`. Also, if we were to forget about or miss the use of using a certain object or reference in a different thread, the compiler would be able to flag that for us and we could fix the bug before even running the code. This helped debugging and understanding the multi-threaded code much easier than expected.

## Usage Instructions ##

1. How To Unpack
    1. All that needs to be done to unpack these files is to unzip the `lab3` zipped folder.  Everything needed for execution is right there.

2. How To Build
    1. As with any rust package, it can be built with `cargo build` or `cargo run` (which executes `cargo build` automatically).

3. How To Run
    1. The program can be run simply with `cargo run` and the program will tell you how to run each part (what types of arguments you need)

## Testing ##
We tested our code in many different ways. First, we tested our code to make sure the no compiler errors or warning occured at any point when compiling or running our code. We also ran our code with relevant scripts and made sure that the output was what we were expecting. To cover edge cases and make sure that we were printing out all the correct errors, we played around with the script files, configuration files, and part files by adding extra tokens, weird spacing, incorrect lines, and more to ensure that we had covered all possible incorrect inputs. This testing included making sure the both remote and local scripts, configuration, and part files worked. Finally, we stress tested the server code by running multiple clients on a single server to make sure that nothing crashed or faulted.