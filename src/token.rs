use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum Token<'a> {
    Tag(&'a str),
    Text(&'a str),
}

impl<'a> Token<'a> {
    pub fn lex(source: &'a str) -> Vec<Self> {
        let mut tokens = Vec::new();
        let mut start = 0;
        let mut end = 0;
        let mut in_tag = false;

        source.graphemes(true).for_each(|char| match char {
            "<" => {
                in_tag = true;
                if end != start {
                    tokens.push(Self::Text(&source[start..end]));
                }
                end += 1;
                start = end;
            }
            ">" => {
                in_tag = false;
                tokens.push(Self::Tag(&source[start..end]));
                end += 1;
                start = end;
            }
            _ => end += char.len(),
        });

        if !in_tag && end != start {
            tokens.push(Self::Text(&source[start..end]));
        }

        tokens
    }
}
