use crate::xml_parse::{Node, NodeId};

impl<'a> Node<'a> {
	pub fn parent(&self) -> Option<Self> {
		self.node_data.parent.and_then(|id| self.document.get_node(id))
	}
	pub fn previous_sibling(&self) -> Option<Self> {
		self.node_data.previous_sibling.and_then(|id| self.document.get_node(id))
	}
	pub fn next_sibling(&self) -> Option<Self> {
		self.node_data
			.next_tree
			.and_then(|id| self.document.get_node(id))
			.filter(|node| node.node_data.previous_sibling.unwrap() == self.id)
	}
	pub fn first_child(&self) -> Option<Self> {
		self.node_data.last_child.and_then(|_| self.document.get_node(NodeId::new(self.id.u32() + 1)))
	}
	pub fn last_child(&self) -> Option<Self> {
		self.node_data.last_child.and_then(|id| self.document.get_node(id))
	}
	pub fn ancestors(&self) -> NodeIter {
		NodeIter {
			current: Some(*self),
			next: Node::parent,
		}
	}
	pub fn pervious_siblings(&self) -> NodeIter {
		NodeIter {
			current: Some(*self),
			next: Node::previous_sibling,
		}
	}
	pub fn next_siblings(&self) -> NodeIter {
		NodeIter {
			current: Some(*self),
			next: Node::next_sibling,
		}
	}
	pub fn children(&self) -> Children {
		Children {
			front: self.first_child(),
			back: self.last_child(),
		}
	}
	pub fn has_children(&self) -> bool {
		self.node_data.last_child.is_some()
	}
}
pub struct NodeIter<'a> {
	current: Option<Node<'a>>,
	next: fn(&Node<'a>) -> Option<Node<'a>>,
}
impl<'a> Iterator for NodeIter<'a> {
	type Item = Node<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let node = self.current.take();
		self.current = node.as_ref().and_then(self.next);
		node
	}
}
pub struct Children<'a> {
	front: Option<Node<'a>>,
	back: Option<Node<'a>>,
}
impl<'a> Iterator for Children<'a> {
	type Item = Node<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.front == self.back {
			self.back = None;
			self.front.take()
		} else {
			let node = self.front.take();
			self.front = node.as_ref().and_then(Node::next_sibling);
			node
		}
	}
}
impl<'a> DoubleEndedIterator for Children<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.front == self.back {
			self.front = None;
			self.back.take()
		} else {
			let node = self.back.take();
			self.back = node.as_ref().and_then(Node::previous_sibling);
			node
		}
	}
}
