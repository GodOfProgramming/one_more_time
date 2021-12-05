use std::{collections::BTreeMap, io::BufReader};
use xml::{reader::XmlEvent, EventReader};

#[derive(Default, Debug)]
pub struct XmlNode {
  pub name: String,
  pub text: Option<String>,
  pub attribs: BTreeMap<String, String>,
  pub children: Vec<XmlNode>,
}

impl XmlNode {
  pub fn parse(data: &str) -> Result<Vec<Self>, String> {
    let reader = BufReader::new(data.as_bytes());
    let parser = EventReader::new(reader);

    let mut root_nodes = Vec::new();
    let mut nodes: Vec<XmlNode> = Vec::new();

    for event in parser {
      let event = event.map_err(|e| e.to_string())?;
      match event {
        XmlEvent::Characters(text) => {
          let node = XmlNode {
            name: String::default(),
            text: Some(text),
            attribs: BTreeMap::default(),
            children: Vec::default(),
          };

          if !nodes.is_empty() {
            nodes.last_mut().unwrap().children.push(node);
          } else {
            root_nodes.push(node);
          }
        }
        XmlEvent::StartElement {
          name, attributes, ..
        } => {
          let name = name.borrow().local_name.to_string();

          let mut attribs = BTreeMap::default();

          for attrib in attributes {
            let attrib = attrib.borrow();
            let name = attrib.name.local_name.to_string();
            let value = attrib.value.to_string();
            attribs.insert(name, value);
          }

          let node = XmlNode {
            name,
            text: None,
            attribs,
            children: Vec::new(),
          };

          nodes.push(node);
        }
        XmlEvent::EndElement { .. } => {
          if !nodes.is_empty() {
            if nodes.len() == 1 {
              root_nodes.push(nodes.pop().unwrap());
            } else {
              let last_node = nodes.pop().unwrap();
              nodes.last_mut().unwrap().children.push(last_node);
            }
          }
        }
        _ => (),
      }
    }

    Ok(root_nodes)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_valid_xml() {
    const XML: &str = "<text>This <bold>is</bold> text</text>";

    let node = XmlNode::parse(XML).unwrap();
    assert_eq!(node.len(), 1);

    let text_node = &node[0];

    assert_eq!(text_node.name, String::from("text"));

    assert_eq!(text_node.children.len(), 3);

    let plain_text1 = &text_node.children[0];

    let bold = &text_node.children[1];

    let plain_text2 = &text_node.children[2];

    assert!(plain_text1.text.is_some());
    assert_eq!(plain_text1.text.as_ref().unwrap(), &String::from("This "));

    assert_eq!(bold.children.len(), 1);
    let bold_text = &bold.children[0];
    assert!(bold_text.text.is_some());
    assert_eq!(bold_text.text.as_ref().unwrap(), &String::from("is"));

    assert!(plain_text2.text.is_some());
    assert_eq!(plain_text2.text.as_ref().unwrap(), &String::from(" text"));
  }
}
