# tlns-google-oauth2

A server-side Google OAuth2 authentication with built in scopes for extra type safety.

## Generating scopes enums

Go to [Google's OAuth2 Scopes Listing](https://developers.google.com/identity/protocols/oauth2/scopes) and start copying from the first header to the final row of the table at the bottom of the document.
![A picture showing the first blue header](./lmao.png)  
![A picture showing the last row of the table](./help.png)  
Then pasting all of that to the `info.txt` and execute `parsery.py` and `to_rust.py` in order.  
And you should get `sin.rs`! The script doesn't guarantee that the `sin.rs` will be valid Rust code. Please fix it yourself. (they are just simply syntax errors but not that much) Or you can improve the `parsery.py` script to make it parse stuff better!

## Usage

```rust


```