# Cadom
Some error-processing helpers for Rust

## Name
This project has autogenerated name, thanks to [This Word Does Not Exist](https://thisworddoesnotexist.com) project.
The word definition can be checked [here](https://l.thisworddoesnotexist.com/7VDZ).

## Purpose
The whole purpose of this project is to make errors processing routine simplier. The idea is to provide enough information about an error happened including its whole backtrace, but avoiding unneccessary verbosity in the code as well as in the output. The "unneccessary verbosity" is a fully subjective concept, so one can disagree with the results.
One can be stucked in choice between using [thiserror](https://crates.io/crates/thiserror) for each kind of processed underlying error, [anyhow](https://crates.io/crates/anyhow) for processing them all no matter what and self-made implementations to gather information about error trace (at least as long as [std::backtrace::Backtrace](https://doc.rust-lang.org/std/backtrace/struct.Backtrace.html) stays experimental). [Cadom](https://crates.io/crates/cadom) should be the decision for such cases.

## Usage example
[Cadom](https://crates.io/crates/cadom) suggests somewhat in the middle of [thiserror](https://crates.io/crates/thiserror), [anyhow](https://crates.io/crates/anyhow) and manual Error implementation. Supposing one has already described some own type `Origin` which gathers all other types of errors processed in an application (presumably via [thiserror](https://crates.io/crates/thiserror)), but is still struggling to add some trace information. It can be easily done via [cadom](https://crates.io/crates/cadom):
```rust
#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
enum FailKind {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Custom(String),
}

impl From<String> for FailKind {
    fn from(src: String) -> Self {
        FailKind::Custom(src)
    }
}

type Fail = cadom::Decay<FailKind>;

fn parse_u8(text: &str) -> Result<u8, std::num::ParseIntError> {
    text.parse::<u8>()
}

fn do_something() -> Result<(), String> {
    Err("Unimplemented functionality".into())
}

fn do_something_else(opt_text: Option<&str>) -> Result<(), Fail> {
    match opt_text {
        Some(text @ "Dramatically unsuitable text") => Err(cadom::decay!("Passed text ('{}') is soooo unsuitable it generated a special error", text)),
        Some(text) => parse_u8(text).map_err(cadom::rot!("Passed text can`t be parsed as u8")).map(|_| ()),
        None => do_something().map_err(cadom::rot!())
    }
}

fn main() {
    let err_1: Fail = do_something_else(Some("Dramatically unsuitable text")).map_err(cadom::rot!()).unwrap_err();
    println!("{}", err_1);
    let err_2: Fail = do_something_else(Some("Not so unsuitable text, but still not a number")).map_err(cadom::rot!()).unwrap_err();
    println!("{}", err_2);
    let err_3: Fail = do_something_else(None).map_err(cadom::rot!("No data were passed into function")).unwrap_err();
    println!("{}", err_3);
}
```
Type FailKind here is being used to convert all needed "external" error types into one with appropriate _type-related_ comments, whereas type Fail presents the multi-level version of application error, with _place-related_ comments added in every place they are needed. Those comments are being automatically enriched with place information in file:line:column format when macros `deacay` or `rot` are used. The example above gives the next output:
```console
{place: [src/main.rs:34:87, src/main.rs:27:60], note: Passed text ('Dramatically unsuitable text') is soooo unsuitable it generated a special error}
{place: [src/main.rs:36:104, src/main.rs:28:46], note: Passed text can`t be parsed as u8, error: invalid digit found in string}
{place: [src/main.rs:38:55], note: No data were passed into function, place: [src/main.rs:29:40], error: Unimplemented functionality}
```
As can be seen, information about every place in code an error passed while program execution (from th moment that error had become of Fail type) is added into that error, due to usage of `decay` and `rot` macros. Unfortunately, an error type annotation should be given usually, but thats considered not a problems since the suggested approach uses a final type, which should be specified in the application itself (like Fail is in the example above).
To fulfill the example, here is the prettified version of the example output:
```console
{
    place: [src/main.rs:34:87, src/main.rs:27:60],
    note: Passed text ('Dramatically unsuitable text') is soooo unsuitable it generated a special error,
}
{
    place: [src/main.rs:36:104, src/main.rs:28:46],
    note: Passed text can`t be parsed as u8,
    error: invalid digit found in string,
}
{
    place: [src/main.rs:38:55],
    note: No data were passed into function,
    place: [src/main.rs:29:40],
    error: Unimplemented functionality,
}
```