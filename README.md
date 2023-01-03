# Mira-CLI
Mira-CLI is a console app to change mode Onyx Boox Mira [ispirated by mira-js](https://github.com/ipodnerd3019/mira-js), usefull with AutoHotkey for create shurtcut with keyboard.

``` ps1
# Build Application
cargo build --release
```
#PS: Need [Rust](https://www.rust-lang.org/tools/install) installed

``` ps1
# Change Screen Mode
".\mira-cli" --mode speed
".\mira-cli" --mode text
".\mira-cli" --mode image
".\mira-cli" --mode video
".\mira-cli" --mode read
```

``` ps1
# Refresh Screen
".\mira-cli" --refresh 1
```