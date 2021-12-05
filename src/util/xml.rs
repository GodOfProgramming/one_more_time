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
    let mut nodes = Vec::new();

    for event in parser {
      match event {
        Ok(XmlEvent::StartElement {
          name, attributes, ..
        }) => {
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
        Ok(XmlEvent::EndElement { .. }) => {
          if !nodes.is_empty() {
            if nodes.len() == 1 {
              root_nodes.push(nodes.pop().unwrap());
            } else {
              let last_node = nodes.pop().unwrap();
              nodes.last_mut().unwrap().children.push(last_node);
            }
          }
        }
        Ok(XmlEvent::Characters(text)) => {
          nodes.last_mut().unwrap().text = Some(text);
        }
        Err(e) => return Err(e.to_string()),
        _ => (),
      }
    }

    Ok(root_nodes)
  }
}
