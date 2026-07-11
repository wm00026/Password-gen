A very basic password-generator program that can generate a password. 
You can chose what characters you would like in your password (lowercase, uppercase, numbers, and a few symbols), along with the length of the password.
Notes:
    - A password with a length less than 4 characters may not contain every option. 
    - The symbols selected (!?$#@) are done for better compatability with password requirements
    - A password will be represented like an array: ["A", "B", "C", "D"]. This *may* be an UX/UI issue but the password is no less secure.

You can use cargo to build and run the program:

```sh
cd password-generator

cargo build

# You can just run without building first, but it's good to verify before running anyway.
cargo run

```
