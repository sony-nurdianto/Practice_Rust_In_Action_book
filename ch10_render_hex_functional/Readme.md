input this command to see the result:

cargo run -- $(echo 'Rust in Action' | sha1sum | cut -f1 -d' ')
