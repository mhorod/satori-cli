use std::collections::HashMap;

pub struct SimpleHTMLParser<'a> {
    content: &'a str,
}

impl<'a> SimpleHTMLParser<'a> {
    pub fn next(self, tag: &'static str) -> Self {
        let (_, content) = self
            .content
            .split_once(format!("</{}>", tag).as_str())
            .unwrap();
        Self::new(content)
    }

    pub fn in_next<F>(self, tag: &'static str, mut func: F) -> Option<Self>
    where
        F: FnMut(Self) -> (),
    {
        let (_, content) = match self.content.split_once(format!("<{}", tag).as_str()) {
            Some(val) => val,
            None => return None,
        };
        let (arg_str, content) = content.split_once(">").unwrap();

        //ignoring recursive tags
        let (content, rest) = content.split_once(format!("</{}>", tag).as_str()).unwrap();

        func(Self::new(content));

        Some(Self::new(rest))
    }

    pub fn on_next<F>(self, tag: &'static str, mut func: F) -> Option<Self>
    where
        F: FnMut(HashMap<String, String>, &str) -> (),
    {
        let (_, content) = match self.content.split_once(format!("<{}", tag).as_str()) {
            Some(val) => val,
            None => return None,
        };
        let (arg_str, content) = content.split_once(">").unwrap();

        //ignoring recursive tags
        let (content, rest) = content.split_once(format!("</{}>", tag).as_str()).unwrap();

        let mut args: HashMap<String, String> = HashMap::new();

        arg_str.trim().split("\" ").for_each(|arg| {
            if arg.len() == 0 {
                return;
            }
            let (key, value) = arg.split_once("=").unwrap();
            let value = if value.ends_with("\"") {
                &value[1..value.len() - 1]
            } else {
                &value[1..value.len()]
            };
            args.insert(key.to_owned(), value.to_owned());
        });

        func(args, content);

        Some(Self::new(rest))
    }

    pub fn on_all<F>(self, tag: &'static str, mut func: F)
    where
        F: FnMut(HashMap<String, String>, &str) -> (),
    {
        let mut val = self;
        while let Some(x) = val.on_next(tag, |args, content| func(args, content)) {
            val = x;
        }
    }

    pub fn new(content: &'a str) -> Self {
        SimpleHTMLParser { content }
    }
}
