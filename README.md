# LilyDown

File format to store short (bagpipe) pieces into markdown files for convenience

```
  ---
  title = "Scotland the Brave"
  music_type = "March"
  time_signature = "4/4"
  composer = "Trad."
  arranger = ""
  instrument = "bagpipe"
  source = "SC v2"
  ---

  ```tex
  \partial 8 e8
  \grg a4 \taor a8.[ b16] \dblc c8[ \gre a8] \dblc c8[ e8]
  \bar "|."
  ```
```

## Usage

```rust
use lilydown::{parse, extract};
```
