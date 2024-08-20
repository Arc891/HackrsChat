#!/bin/bash
# This command is used to capture any output from the program since the terminal is being captured by Cursive.
# This means we need to pipe the output to a file so we can see it. I run the program with the RUST_BACKTRACE=full
# by default should an error occur. 
RUST_BACKTRACE=full cargo run --bin hackrschat > err.out 2>&1;