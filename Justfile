# This is a Justfile, a file that contains recipes to build, run, test, and deploy your code.

# This command runs the client and captures the output in a file called err.out since cursive takes over the terminal and will not show any output.
run:
    RUST_BACKTRACE=full cargo run -p hackrschat-client > err.out 2>&1;

server:
    cargo run -p hackrschat-server;
