use pulldown_cmark::Event::{End, Rule, Start, Text};
use pulldown_cmark::Tag::CodeBlock;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub music_type: Option<String>,
    pub time_signature: Option<String>,
    pub tempo: Option<String>,
    pub composer: Option<String>,
    pub arranger: Option<String>,
    pub instrument: Option<String>,
    pub source: Option<String>,
}

#[derive(PartialEq)]
enum ParsingState {
    Header,
    Score,
    Other,
}

pub fn extract(content: &str) -> (String, String) {
    let mut header = String::new();
    let mut score = String::new();
    use ParsingState::*;
    let mut state = Other;
    let parser = pulldown_cmark::Parser::new(content);
    for event in parser {
        match event {
            Rule => state = if state == Other { Header } else { Other },

            Start(CodeBlock(_)) => state = Score,
            End(CodeBlock(_)) => state = Other,

            Text(text) => match state {
                Header => header.push_str(format!("{}{}", text, '\n').as_str()),
                Score => score.push_str(text.as_ref()),
                Other => (),
            },
            _ => (),
        }
    }

    (header, score)
}

pub fn parse(content: &str) -> FrontMatter {
    toml::from_str(content).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_parse() {
        let (header, score) = extract(
            r#"
---
key = "val"
clef = "G"
---
```ly
e8 \grg a4
```
         "#,
        );
        assert_eq!(header, "key = \"val\"\nclef = \"G\"\n".to_owned());
        assert_eq!(score, "e8 \\grg a4\n".to_owned());
    }

    #[test]
    fn toml_parse() {
        let result = parse(r#" title = "Brolum" "#);
        assert_eq!(result.title.unwrap(), "Brolum");
    }

    #[test]
    fn together() {
        let file = r#"
---
title = "Gael"
---
```tex
e8```
         "#;
        let (header, _) = extract(file);
        let result = parse(&header);
        assert_eq!(result.title.unwrap(), "Gael")
    }
}
