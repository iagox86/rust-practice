// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/html_renderer.html

use std::collections::HashMap;

enum ElementBodyType {
  StringType(String),
  TagType(Box<dyn IsElement>),
}

impl From<&str> for ElementBodyType {
  fn from(item: &str) -> Self {
    ElementBodyType::StringType(String::from(item))
  }
}

impl From<Box<dyn IsElement>> for ElementBodyType {
  fn from(item: Box<dyn IsElement>) -> Self {
    ElementBodyType::TagType(item)
  }
}

trait IsElement {
  fn tag(&self) -> String;
  fn content(&self) -> &Vec<ElementBodyType>;
  fn content_mut(&mut self) -> &mut Vec<ElementBodyType>;
  fn attributes(&self) -> &HashMap<String, String>;

  fn is_one_line(&self) -> bool {
    return false;
  }

  fn render_opening_tag(&self, indent: usize, buffer: &mut String) {
    buffer.push_str(&format!("{:indent$}<{}", "", self.tag(), indent=indent));

    let attr = self.attributes();
    if attr.len() > 0 {
      let tmp: Vec<String> = attr.into_iter().map(|(k, v)| {
        format!("{}=\"{}\"", k, v)
      }).collect();
      buffer.push_str(" ");
      buffer.push_str(&tmp.join(" "));
    }
    buffer.push('>');
    if !self.is_one_line() {
      buffer.push('\n');
    }
  }

  fn render_body(&self, indent: usize, buffer: &mut String) {
    self.content().into_iter().for_each(|e| {
      match e {
        ElementBodyType::StringType(s) => {
          if self.is_one_line() {
            buffer.push_str(&format!("{}", &s));
          } else {
            buffer.push_str(&format!("{:indent$}{}\n", "", &s, indent=indent+2));
          }
        }

        ElementBodyType::TagType(e) => {
          buffer.push_str(&e.render_internal(indent + 2));
        }
      }
    });
  }

  fn render_closing_tag(&self, indent: usize, buffer: &mut String) {
    if !self.is_one_line() {
      buffer.push_str(&format!("{:indent$}", "", indent=indent));
    }

    buffer.push_str(&format!("</{}>\n", self.tag()));
  }

  fn render_internal(&self, indent: usize) -> String {
    let mut out = String::new();

    self.render_opening_tag(indent, &mut out);
    self.render_body(indent, &mut out);
    self.render_closing_tag(indent, &mut out);

    out
  }

  fn render(&self) -> String {
    self.render_internal(0)
  }

  fn append(&mut self, data: ElementBodyType) {
    let content = self.content_mut();
    content.push(data);
  }
}

struct HtmlElement {
  content: Vec<ElementBodyType>,
  attributes: HashMap<String, String>,
}
impl HtmlElement {
  fn new(attributes: Option<HashMap<String, String>>) -> Self {
    return HtmlElement { content: Vec::new(), attributes: attributes.unwrap_or(HashMap::new()) };
  }
}
impl IsElement for HtmlElement {
  fn tag(&self) -> String {
    return String::from("html");
  }

  fn content(&self) -> &Vec<ElementBodyType> {
    return &self.content;
  }

  fn content_mut(&mut self) -> &mut Vec<ElementBodyType> {
    return &mut self.content;
  }

  fn attributes(&self) -> &HashMap<String, String> {
    return &self.attributes;
  }
}

struct HeadElement {
  content: Vec<ElementBodyType>,
  attributes: HashMap<String, String>,
}
impl HeadElement {
  fn new(attributes: Option<HashMap<String, String>>) -> Self {
    return HeadElement { content: Vec::new(), attributes: attributes.unwrap_or(HashMap::new()) };
  }
}
impl IsElement for HeadElement {
  fn tag(&self) -> String {
    return String::from("head");
  }

  fn content(&self) -> &Vec<ElementBodyType> {
    return &self.content;
  }

  fn content_mut(&mut self) -> &mut Vec<ElementBodyType> {
    return &mut self.content;
  }

  fn attributes(&self) -> &HashMap<String, String> {
    return &self.attributes;
  }
}

struct TitleElement {
  content: Vec<ElementBodyType>,
  attributes: HashMap<String, String>,
}
impl TitleElement {
  fn new(attributes: Option<HashMap<String, String>>) -> Self {
    return TitleElement { content: Vec::new(), attributes: attributes.unwrap_or(HashMap::new()) };
  }
}
impl IsElement for TitleElement {
  fn tag(&self) -> String {
    return String::from("title");
  }

  fn content(&self) -> &Vec<ElementBodyType> {
    return &self.content;
  }

  fn content_mut(&mut self) -> &mut Vec<ElementBodyType> {
    return &mut self.content;
  }

  fn is_one_line(&self) -> bool {
    return true;
  }

  fn attributes(&self) -> &HashMap<String, String> {
    return &self.attributes;
  }
}

struct BodyElement {
  content: Vec<ElementBodyType>,
  attributes: HashMap<String, String>,
}
impl BodyElement {
  fn new(attributes: Option<HashMap<String, String>>) -> Self {
    return BodyElement { content: Vec::new(), attributes: attributes.unwrap_or(HashMap::new()) };
  }
}
impl IsElement for BodyElement {
  fn tag(&self) -> String {
    return String::from("body");
  }

  fn content(&self) -> &Vec<ElementBodyType> {
    return &self.content;
  }

  fn content_mut(&mut self) -> &mut Vec<ElementBodyType> {
    return &mut self.content;
  }

  fn attributes(&self) -> &HashMap<String, String> {
    return &self.attributes;
  }
}

struct PElement {
  content: Vec<ElementBodyType>,
  attributes: HashMap<String, String>,
}
impl PElement {
  fn new(attributes: Option<HashMap<String, String>>) -> Self {
    return PElement { content: Vec::new(), attributes: attributes.unwrap_or(HashMap::new()) };
  }
}
impl IsElement for PElement {
  fn tag(&self) -> String {
    return String::from("p");
  }

  fn content(&self) -> &Vec<ElementBodyType> {
    return &self.content;
  }

  fn content_mut(&mut self) -> &mut Vec<ElementBodyType> {
    return &mut self.content;
  }

  fn attributes(&self) -> &HashMap<String, String> {
    return &self.attributes;
  }
}

fn main() {
  let mut html = HtmlElement::new(None);

  let mut body = BodyElement::new(None);

  let mut head = HeadElement::new(None);

  let mut title_attributes: HashMap<String, String> = HashMap::new();
  title_attributes.insert(String::from("this_is"), String::from("an attribute"));
  title_attributes.insert(String::from("and_another"), String::from("one"));
  let mut title = TitleElement::new(Some(title_attributes));

  title.append(ElementBodyType::from("NotPythonClass - Session 6 example"));
  head.append(ElementBodyType::from(Box::new(title) as Box<dyn IsElement>));
  html.append(ElementBodyType::from(Box::new(head) as Box<dyn IsElement>));

  let mut p1 = PElement::new(None);
  p1.append(ElementBodyType::from("First paragraph"));

  let mut p2 = PElement::new(None);
  p2.append(ElementBodyType::from("Nested paragraph"));

  let mut p3 = PElement::new(None);
  p3.append(ElementBodyType::from("Separate paragraph"));

  p1.append(ElementBodyType::from(Box::new(p2) as Box<dyn IsElement>));
  body.append(ElementBodyType::from(Box::new(p1) as Box<dyn IsElement>));
  body.append(ElementBodyType::from(Box::new(p3) as Box<dyn IsElement>));

  html.append(ElementBodyType::from("Some string"));
  html.append(ElementBodyType::from("Some other string"));
  html.append(ElementBodyType::from(Box::new(body) as Box<dyn IsElement>));
  html.append(ElementBodyType::from("Yet another string"));

  println!("{}", html.render());
}
