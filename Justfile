


# This command is used to capture any output from the program since the terminal is being captured by Cursive.
# This means we need to pipe the output to a file so we can see it. I run the program with the RUST_BACKTRACE=full
# by default should an error occur.

run:
    RUST_BACKTRACE=full cargo run -p hackrschat-client > err.out 2>&1;

server:
    cargo run -p hackrschat-server;

