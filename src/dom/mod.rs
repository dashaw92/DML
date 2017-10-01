#![allow(dead_code)]
use std::error::Error;
use std::fmt;
use std::collections::HashMap;

///Takes stdin and converts it to html
pub fn parse(input: String) -> Result<String, ParseError> {
    let mut dom = Dom::new();
    //Support multi line by just removing backslashes followed by newlines :)
    let input = input
                 .replace("\\\n",   "")  //UNIX like
                 .replace("\\\r\n", "")  //Windows
                 .replace("\\\r",   ""); //Max OS X
    for line in input.lines() {
        if line.starts_with("[") {
            let (key, val) = scan_fmt!(line, r#"[{} = "{[0-9a-zA-Z ]}"]"#, String, String);
            let (key, val) = match (key, val) {
                (Some(k), Some(v)) => (k, v),
                _ => {
                    return Err(ParseError {
                        description: format!("Invalid line:\n{}", line),
                    })
                }
            };
            match key.clone().as_ref() {
                "style" => dom.set_style(val),
                "script" => dom.set_script(val),
                "title" => dom.set_title(val),
                _ => dom.push_metadata(key, val),
            }
        }
    }
    Ok(dom.serialize())
}

#[derive(Debug)]
pub struct ParseError {
    pub description: String,
}

impl Error for ParseError {
    fn description(&self) -> &str {
        self.description.as_ref()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing Error")
    }
}

struct Dom<'a> {
    metadata: HashMap<String, String>,
    title: String,
    styles: Option<String>,
    script: Option<String>,
    nodes: Vec<Node<'a>>,
}

struct Node<'a> {
    domtype: &'a str,
    class: Option<&'a str>,
    id: Option<&'a str>,
    content: Option<&'a str>,
    children: Vec<Node<'a>>,
}

impl<'a> Dom<'a> {
    fn new() -> Dom<'a> {
        Dom {
            metadata: HashMap::new(),
            styles: None,
            script: None,
            title: "Generated with sitegen!".to_owned(),
            nodes: vec![],
        }
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn set_style(&mut self, css: String) {
        self.styles = Some(css);
    }

    fn set_script(&mut self, js: String) {
        self.script = Some(js);
    }

    fn push_metadata(&mut self, key: String, val: String) {
        self.metadata.insert(key, val);
    }

    fn push_node(&mut self, node: Node<'a>) {
        self.nodes.push(node);
    }

    fn serialize(&self) -> String {
        let mut buffer: Vec<String> = vec!["<html>".to_owned(), "<head>".to_owned()];
        buffer.push(format!("<title>{}</title>", self.title).to_owned());
        if let Some(ref styles) = self.styles {
            buffer.push(
                format!(
                    r#"<link rel="styleshet" type="text/css" href="{}" />"#,
                    styles
                ).to_owned(),
            );
        }
        if let Some(ref script) = self.script {
            buffer.push(
                format!(
                    r#"<script src="{}" type="text/javascript"><script>"#,
                    script
                ).to_owned(),
            );
        }
        let zipped = self.metadata.keys().zip(self.metadata.values());
        for (key, val) in zipped {
            buffer.push(
                format!(r#"<meta name="{}" content="{}">"#, key, val).to_owned(),
            );
        }
        buffer.push("</head>".to_owned());
        buffer.push("<body>".to_owned()); //I would keep this in the line above, but I'd rather keep dom elems seperate. TODO
        for node in &self.nodes {
            buffer.push(node.serialize());
        }
        buffer.push("</body>".to_owned());
        buffer.push("</html>".to_owned());
        buffer.join("\n").to_owned()
    }
}

impl<'a> Node<'a> {
    fn serialize(&self) -> String {
        let mut buffer: Vec<String> = vec![];
        let class = match self.class {
            Some(class) => format!(r#"class="{}""#, class).to_owned(),
            None => "".to_owned(),
        };
        let id = match self.id {
            Some(id) => format!(r#"id="{}""#, id).to_owned(),
            None => "".to_owned(),
        };
        let content = match self.content {
            Some(content) => content,
            None => "",
        };
        let formatted = format!("<{} {} {}>{}", self.domtype, class, id, content).to_owned();
        buffer.push(formatted);
        for child in &self.children {
            buffer.push(child.serialize());
        }
        buffer.push(format!("</{}>", self.domtype).to_owned());
        buffer.join("\n").to_owned()
    }
}
