// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/html_renderer.html

use std::collections::HashMap;

enum Content {
  Text(String),
  Tag(Element),
}

enum Tag {
  Html,
  Head,
  Title,
  P,
  Body,
  Hr,
  Br,
  A,
  Ul,
  Li,
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
  Meta,
}

impl Tag {
  fn is_one_line(&self) -> bool {
    match *self {
      Tag::Title |
        Tag::A |
        Tag::Li |
        Tag::H1 |
        Tag::H2 |
        Tag::H3 |
        Tag::H4 |
        Tag::H5 |
        Tag::H6 => true,

      _ => false,
    }
  }

  fn is_self_closing(&self) -> bool {
    match *self {
      Tag::Hr | Tag::Br | Tag::Meta => true,
      _                 => false,
    }
  }

  fn tag(&self) -> &'static str {
    match *self {
      Tag::Html  => "html",
      Tag::Head  => "head",
      Tag::Title => "title",
      Tag::P     => "p",
      Tag::Body  => "body",
      Tag::Hr    => "hr",
      Tag::Br    => "br",
      Tag::A     => "a",
      Tag::Li    => "li",
      Tag::Ul    => "ul",
      Tag::H1    => "h1",
      Tag::H2    => "h2",
      Tag::H3    => "h3",
      Tag::H4    => "h4",
      Tag::H5    => "h5",
      Tag::H6    => "h6",
      Tag::Meta  => "meta",
    }
  }

  fn render_opening_tag(&self, indent: usize, attributes: &Option<HashMap<String, String>>, buffer: &mut String) {
    buffer.push_str(&format!("{:indent$}<{}", "", self.tag(), indent=indent));

    if let Some(attributes) = attributes {
      if attributes.len() > 0 {
        for (k, v) in attributes.into_iter() {
          buffer.push_str(&format!(" {}=\"{}\"", k, v));
        }
      }
    }

    if self.is_one_line() {
      buffer.push_str(">");
    } else {
      buffer.push_str(">\n");
    }
  }

  fn render_content(&self, indent: usize, content: &Content, buffer: &mut String) {
    match content {
      Content::Text(s) => {
        if self.is_one_line() {
          buffer.push_str(s);
        } else {
          buffer.push_str(&format!("{:indent$}{}\n", "", s, indent=(indent + 2)));
        }
      }

      Content::Tag(e) => {
        if self.is_one_line() {
          buffer.push_str(&format!("{:indent$}", "", indent=indent));
        }

        buffer.push_str(&e.render(indent + 2));

        if self.is_one_line() {
          buffer.push('\n');
        }
      }
    }
  }

  fn render_closing_tag(&self, indent: usize, buffer: &mut String) {
    if self.is_one_line() {
      buffer.push_str(&format!("</{}>\n", self.tag()));
    } else {
      buffer.push_str(&format!("{:indent$}</{}>\n", "", self.tag(), indent=indent));
    }
  }

  fn render_self_closing_tag(&self, indent: usize, attributes: &Option<HashMap<String, String>>, buffer: &mut String) {
    buffer.push_str(&format!("{:indent$}<{}", "", self.tag(), indent=indent));

    if let Some(attributes) = attributes {
      if attributes.len() > 0 {
        for (k, v) in attributes.into_iter() {
          buffer.push_str(&format!(" {}=\"{}\"", k, v));
        }
      }
    }

    buffer.push_str(" />\n");
  }

  fn render(&self, indent: usize, attributes: &Option<HashMap<String, String>>, content: &Vec<Content>) -> String {
    let mut out: String = String::new();

    if self.is_self_closing() {
      self.render_self_closing_tag(indent, attributes, &mut out);
    } else {
      self.render_opening_tag(indent, attributes, &mut out);
      for c in content.iter() {
        self.render_content(indent, c, &mut out);
      };
      self.render_closing_tag(indent, &mut out);
    }

    out
  }
}

struct Element {
  tag: Tag,
  attributes: Option<HashMap<String, String>>,
  content: Vec<Content>,
}

impl Element {
  fn new(tag: Tag, attributes: Option<HashMap<String, String>>) -> Self {
    return Element {
      tag: tag,
      attributes: attributes,
      content: Vec::new(),
    }
  }

  fn new_with_text(tag: Tag, attributes: Option<HashMap<String, String>>, text: &str) -> Self {
    let mut e = Self::new(tag, attributes);
    e.append(Content::Text(String::from(text)));

    e
  }

  fn create_p(attributes: Option<HashMap<String, String>>, text: &str) -> Self {
    Self::new_with_text(Tag::P, attributes, text)
  }

  fn create_link(href: String, text: String) -> Element {
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert(String::from("href"), href);

    let mut a = Element::new(Tag::A, Some(attributes));
    a.append(Content::Text(text));

    a
  }

  fn create_charset(charset: &str) -> Self {
    let mut attributes: HashMap<String, String> = HashMap::new();
    attributes.insert(String::from("charset"), String::from(charset));

    Element::new(Tag::Meta, Some(attributes))
  }

  fn create_title(title: &str) -> Self {
    Self::new_with_text(Tag::Title, None, title)
  }

  fn create_list(elements: Vec<&str>) -> Element {
    let mut ul = Element::new(Tag::Ul, None);

    for e in elements.into_iter() {
      let mut li = Element::new(Tag::Li, None);
      li.append(Content::Text(String::from(e)));
      ul.append(Content::Tag(li));
    }

    ul
  }

  fn create_header(text: &str, level: usize) -> Element {
    match level {
      1 => Self::new_with_text(Tag::H1, None, text),
      2 => Self::new_with_text(Tag::H2, None, text),
      3 => Self::new_with_text(Tag::H3, None, text),
      4 => Self::new_with_text(Tag::H4, None, text),
      5 => Self::new_with_text(Tag::H5, None, text),
      _ => Self::new_with_text(Tag::H6, None, text),
    }
  }

  fn append(&mut self, content: Content) {
    self.content.push(content);
  }

  fn render(&self, indent: usize) -> String {
    self.tag.render(indent, &self.attributes, &self.content)
  }
}

fn main() {
  let mut html = Element::new(Tag::Html, None);
  html.append(Content::Text(String::from("Before head")));

  let mut head = Element::new(Tag::Head, None);
  head.append(Content::Tag(Element::create_charset("UTF-8")));
  head.append(Content::Tag(Element::create_title("This is my title")));

  html.append(Content::Tag(head));

  html.append(Content::Text(String::from("After head")));

  let mut body = Element::new(Tag::Body, None);

  body.append(Content::Tag(Element::create_header("My Test Application", 1)));

  let mut p1 = Element::create_p(None, "Paragraph one");

  let p2 = Element::create_p(None, "Nested paragraph");
  p1.append(Content::Tag(p2));
  body.append(Content::Tag(p1));

  let hr = Element::new(Tag::Hr, None);
  body.append(Content::Tag(hr));

  let mut p3 = Element::new(Tag::P, None);
  p3.append(Content::Text(String::from("Separate paragraph")));
  p3.append(Content::Tag(Element::create_link(String::from("http://google.com"), String::from("google"))));
  body.append(Content::Tag(p3));

  let mut p4 = Element::new(Tag::P, None);
  p4.append(Content::Text(String::from("List:")));
  html.append(Content::Tag(p4));

  html.append(Content::Tag(Element::create_list(vec!["element1", "element2", "element3"])));

  html.append(Content::Tag(body));

  println!("<!DOCTYPE html>");
  println!("{}", html.render(0));
}
