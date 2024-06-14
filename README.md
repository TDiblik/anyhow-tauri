## Introduction

A crate that makes it easy to use the [anyhow crate](https://github.com/dtolnay/anyhow) with the [tauri framework](https://tauri.app/) as a command result.

## How to use

Define a function that uses `anyhow`:

```rust
fn function_that_throws() -> anyhow::Result<()> {
    anyhow::bail!("Simulating a possible throw")
}

fn function_that_succeeds() -> anyhow::Result<String> {
    Ok("this function succeeds".to_string())
}
```

Then proceed to use the `anyhow_tauri::TAResult<T>` as a return type of your command. And that's it! Now, you can handle your errors as you would outside tauri!

```rust
#[tauri::command]
fn test() -> anyhow_tauri::TAResult<String> {
    Ok("No error thrown.".into())
}

#[tauri::command]
fn test_anyhow_success() -> anyhow_tauri::TAResult<String> {
    function_that_succeeds().into_ta_result()

    // could also be written as
    // Ok(function_that_succeeds()?)
}

#[tauri::command]
fn test_throw() -> anyhow_tauri::TAResult<String> {
    function_that_throws()?;

    Ok("this should never trigger".to_owned())
}

#[tauri::command]
fn test_pure_err_conversion() -> anyhow_tauri::TAResult<String> {
    let _some_empty_err = anyhow::anyhow!("some err").into_ta_empty_result();
    anyhow::anyhow!("Showcase of the .into_ta_result()").into_ta_result()
}

#[tauri::command]
fn test_bail() -> anyhow_tauri::TAResult<String> {
    anyhow_tauri::bail!("Showcase of the .bail!()")
}

#[tauri::command]
fn test_ensure() -> anyhow_tauri::TAResult<String> {
    anyhow_tauri::ensure!(1 == 2); // this should throw

    Ok("this should never trigger".to_owned())
}
```

## Notes

- Notice that you can casually use the `?` operator.
- I've had to create a wrapper for the `bail!()` and `ensure!()` macros, since I was unable to implement proper traits for them (somebody can submit a PR tho).
- The `TA` before each type means `TauriAnyhow` because I don't want to use type names that could collide with other type names in your codebase.
- The whole crate is around +-100 lines of code. If you don't want to depend on _another_ package, you should be able to copy-paste it to your codebase without any problems (I don't expect this crate to change _that_ much).

## Caveats

- By default, **sending errors to the client is disabled in production builds**. To enable them, turn on the feature `show_errs_in_release`.
- You have to use the `anyhow_tauri::TAResult<T>` as a return type of your commands. Could be a problem for some people.
- Might lack support for some `anyhow` features, I didn't find any while dogfooding tho!

## Credit

Initially, when I asked about using `anyhow` with tauri in a Tauri Working Group Office Hours call, a Discord user [@kesomannen](https://github.com/Kesomannen), suggested I use the following code:

```rust
#[derive(Debug)]
pub struct CommandError(pub anyhow::Error);

impl std::error::Error for CommandError {}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:#}", self.0))
    }
}

impl From<anyhow::Error> for CommandError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

pub type Result<T> = std::result::Result<T, CommandError>;
```

I've taken this code, further extended it, and added some additional features so that it's "smoother" for `anyhow` to work with `tauri`.
The code is originally from his [gale project](https://github.com/Kesomannen/gale/blob/7d05fa16c3497dce002e3b77b29e2ec922fa1ad7/src-tauri/src/util/cmd.rs), which is under GPL-3.0, however, he said he's cool with this package being MIT. Sooo, go support him! :D
