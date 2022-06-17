use std::borrow::Cow;
use std::{num::NonZeroU32, ops::Range};

use std::fmt::Debug;

mod iters;
mod token;

use crate::error::{StreamError, SvgError, SvgErrorType, SvgResult};
use crate::span::{ErrorPos, QName, ResolvedQName, Span};
use crate::stream::SvgStream;
use token::ElementControl;
use token::{Attribute, Token, Tokeniser};

pub const SVG_URI: &'static str = "http://www.w3.org/2000/svg";
pub const NS_XML_URI: &str = "http://www.w3.org/XML/1998/namespace";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeId(NonZeroU32);
impl NodeId {
	pub fn new(i: u32) -> NodeId {
		NodeId(NonZeroU32::new(i + 1).unwrap())
	}
	pub fn u32(&self) -> u32 {
		self.0.get() - 1
	}
	pub fn usize(&self) -> usize {
		self.u32() as usize
	}
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Node<'a> {
	pub id: NodeId,
	pub document: &'a Document<'a>,
	pub node_data: &'a NodeData<'a>,
}
#[derive(PartialEq)]
pub struct Namespace<'a> {
	pub prefix: Option<&'a str>,
	pub uri: &'a str,
}
#[derive(PartialEq)]
pub struct ResolvedAttribute<'a> {
	pub qname: ResolvedQName<'a>,
	pub value: Span<'a>,
}
impl<'a> Debug for ResolvedAttribute<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} = {}", self.qname, self.value)
	}
}
#[derive(PartialEq)]
pub struct Document<'a> {
	text: &'a str,
	nodes: Vec<NodeData<'a>>,
	attributes: Vec<ResolvedAttribute<'a>>,
	pub namespaces: Vec<Namespace<'a>>,
}
impl<'a> Document<'a> {
	pub fn root(&'a self) -> Node<'a> {
		Node {
			id: NodeId::new(0),
			document: self,
			node_data: &self.nodes[0],
		}
	}
	fn append_node(&mut self, parent: NodeId, kind: NodeKind<'a>, parse_data: &mut ParseData) -> NodeId {
		// Generate new ID
		let new_id = NodeId::new(self.nodes.len() as u32);

		// Retrieve the previous last child of the parent, this is the previous sibling
		let old_last_child = self.nodes[parent.usize()].last_child;
		// Update the last child of the parent
		self.nodes[parent.usize()].last_child = Some(new_id);

		for nodeid in &parse_data.awaiting_next_tree {
			self.nodes[nodeid.usize()].next_tree = Some(new_id);
		}
		parse_data.awaiting_next_tree.clear();

		match kind {
			NodeKind::Element { .. } => {}
			// The next sibling of non-elements is always the next node in the tree
			_ => parse_data.awaiting_next_tree.push(new_id),
		}
		self.nodes.push(NodeData {
			parent: Some(parent),
			previous_sibling: old_last_child,
			next_tree: None,
			last_child: None,
			kind,
		});

		new_id
	}
	pub fn get_node(&'a self, id: NodeId) -> Option<Node<'a>> {
		self.nodes.get(id.usize()).map(|node_data| Node { id, document: self, node_data })
	}
	pub fn create_err(&self, byte: usize) -> ErrorPos {
		ErrorPos::new(self.text, byte)
	}
}
impl<'a> Debug for Document<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		macro_rules! writeln_indented {
            ($depth:expr, $f:expr, $fmt:expr) => {
                for _ in 0..$depth { write!($f, "    ")?; }
                writeln!($f, $fmt)?;
            };
            ($depth:expr, $f:expr, $fmt:expr, $($arg:tt)*) => {
                for _ in 0..$depth { write!($f, "    ")?; }
                writeln!($f, $fmt, $($arg)*)?;
            };
        }

		fn print_children(node: Node, depth: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			for child in node.children() {
				if let NodeKind::Element { tag, attributes } = &child.node_data.kind {
					writeln_indented!(depth, f, "Element {{");
					writeln_indented!(depth + 1, f, "Tag = {}", tag);
					if attributes.start != attributes.end {
						writeln_indented!(depth + 1, f, "Attributes = [");
						for index in (attributes.start..attributes.end).rev() {
							writeln_indented!(depth + 2, f, "{:?}", child.document.attributes[index as usize]);
						}
						writeln_indented!(depth + 1, f, "]");
					}
					if child.has_children() {
						writeln_indented!(depth + 1, f, "Children = [");
						print_children(child, depth + 2, f)?;
						writeln_indented!(depth + 1, f, "]");
					}
					writeln_indented!(depth, f, "}}");
				} else {
					writeln_indented!(depth, f, "{:?}", child.node_data.kind);
				}
			}
			Ok(())
		}

		writeln!(f, "Document [")?;
		print_children(self.root(), 1, f)?;
		writeln!(f, "]")?;

		Ok(())
	}
}
#[derive(PartialEq, Debug)]
pub struct NodeData<'a> {
	/// The node above this node in the tree
	pub parent: Option<NodeId>,
	/// The sibling before this node in the tree
	pub previous_sibling: Option<NodeId>,
	/// Either the next sibling or if there isn't one, then next sibling of a parent element
	pub next_tree: Option<NodeId>,
	/// The last node that is a child of this node in the tree
	pub last_child: Option<NodeId>,
	pub kind: NodeKind<'a>,
}
#[derive(Debug, PartialEq)]
pub enum NodeKind<'a> {
	Root,
	Element { tag: ResolvedQName<'a>, attributes: Range<u32> },
	Text(Cow<'a, str>),
}

struct ParseData<'a> {
	temporary_attributes: Vec<Attribute<'a>>,
	first_attribute_index: usize,
	parent_id: NodeId,
	tag_name: QName<'a>,
	awaiting_next_tree: Vec<NodeId>,
}
pub fn parse_svg<'a>(svg: &'a str) -> SvgResult<Document<'a>> {
	let mut parse_data = ParseData {
		temporary_attributes: Vec::with_capacity(16),
		first_attribute_index: 0,
		parent_id: NodeId::new(0),
		tag_name: QName::default(),
		awaiting_next_tree: Vec::new(),
	};

	let mut document = Document {
		nodes: Vec::with_capacity(svg.chars().filter(|&c| c == '<').count()),
		attributes: Vec::with_capacity(svg.chars().filter(|&c| c == '=').count()),
		namespaces: Vec::new(),
		text: svg,
	};
	document.nodes.push(NodeData {
		parent: None,
		previous_sibling: None,
		next_tree: None,
		last_child: None,
		kind: NodeKind::Root,
	});
	document.namespaces.push(Namespace { prefix: Some("xml"), uri: NS_XML_URI });

	let mut tokeniser = Tokeniser::from(SvgStream::from(svg));
	parse_tokens(&mut tokeniser, &mut parse_data, &mut document)?;

	document.attributes.shrink_to_fit();
	document.nodes.shrink_to_fit();

	println!("{document:?}");

	Ok(document)
}

fn parse_tokens<'a>(tokeniser: &mut Tokeniser<'a>, parse_data: &mut ParseData<'a>, document: &mut Document<'a>) -> SvgResult<()> {
	for token in tokeniser {
		let token = token?;
		match token {
			// Processing instructions are discarded
			Token::ProcessingInstruction { .. } => {}
			Token::ElementStart { qname, .. } => {
				parse_data.tag_name = qname;
			}
			Token::Attribute(attr) => process_attribute(parse_data, attr, document)?,
			Token::ElementControl { control, .. } => {
				process_element_control(control, document, parse_data)?;
			}
			Token::Text { span } => {
				document.append_node(parse_data.parent_id, NodeKind::Text(process_text(span.text)), parse_data);
			}
		}
	}
	Ok(())
}

fn process_attribute<'a>(parse_data: &mut ParseData<'a>, attribute: Attribute<'a>, document: &mut Document<'a>) -> SvgResult<()> {
	if attribute.qname.prefix.text == "xmlns" {
		// Namespace decleration
		document.namespaces.push(Namespace {
			prefix: Some(attribute.qname.local.text),
			uri: attribute.value.text,
		})
	} else if attribute.qname.local.text == "xmlns" {
		// Global namespace
		document.namespaces.push(Namespace {
			prefix: None,
			uri: attribute.value.text,
		})
	} else {
		parse_data.temporary_attributes.push(attribute)
	}

	Ok(())
}

/// Converts text possibly containg character references (e.g. `&lt;` for less than)
fn process_text<'a>(text: &'a str) -> Cow<'a, str> {
	let mut stream = SvgStream::from(text);
	let mut text: Option<Cow<'a, str>> = None;
	while !stream.at_end() {
		let segement = stream.consume_str_chunk();
		if let Some(text) = &mut text {
			text.to_mut().push_str(segement);
		} else {
			text = Some(Cow::Borrowed(segement));
		}
	}
	text.unwrap_or(Cow::Borrowed(""))
}

fn push_attributes<'a>(parse_data: &mut ParseData<'a>, document: &mut Document<'a>) -> SvgResult<Range<u32>> {
	while let Some(attr) = parse_data.temporary_attributes.pop() {
		let resolved_qname = ResolvedQName::from_qname(&attr.qname, document, true)?;

		// Check duplicated
		if document.attributes[parse_data.first_attribute_index..].iter().any(|x| x.qname == resolved_qname) {
			return Err(SvgError(SvgErrorType::DuplicateAttribute, StreamError::default(), document.create_err(resolved_qname.local.start)));
		}

		document.attributes.push(ResolvedAttribute {
			qname: resolved_qname,
			value: attr.value,
		})
	}
	let result = parse_data.first_attribute_index as u32..document.attributes.len() as u32;
	parse_data.first_attribute_index = document.attributes.len();

	Ok(result)
}

fn process_element_control<'a>(control: ElementControl, document: &mut Document<'a>, parse_data: &mut ParseData<'a>) -> SvgResult<()> {
	let attributes = push_attributes(parse_data, document)?;
	match control {
		ElementControl::EmptyTag => {
			let resolved_qname = ResolvedQName::from_qname(&parse_data.tag_name, document, false)?;

			let new_node = document.append_node(parse_data.parent_id, NodeKind::Element { tag: resolved_qname, attributes }, parse_data);
			parse_data.awaiting_next_tree.push(new_node);
		}
		ElementControl::Close(qname) => {
			if let NodeKind::Element { tag, .. } = &document.nodes[parse_data.parent_id.usize()].kind {
				if tag.prefix != qname.prefix || tag.local != qname.local {
					return Err(SvgError(SvgErrorType::InvalidCloseElement, StreamError::default(), document.create_err(qname.local.start)));
				}
				parse_data.awaiting_next_tree.push(parse_data.parent_id);
			}
			parse_data.parent_id = document.nodes[parse_data.parent_id.usize()].parent.unwrap();
		}
		ElementControl::Open => {
			let resolved_qname = ResolvedQName::from_qname(&parse_data.tag_name, document, false)?;
			let new_node = document.append_node(parse_data.parent_id, NodeKind::Element { tag: resolved_qname, attributes }, parse_data);
			parse_data.parent_id = new_node;
		}
	}
	Ok(())
}

#[test]
fn parse_simple() {
	let _ = parse_svg(
		r#"
<?xml version="1.0" encoding="UTF-8" standalone="no"?><?xml version="1.0" encoding="UTF-8" standalone="no"?> <v:bob xmlns:v="http://www.w3.org/2000/svg"><jeff/></v:bob>"#,
	)
	.unwrap();
}
