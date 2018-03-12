use std::collections::HashMap;
use std::cmp::max;
use std::ops::Index;

pub struct Heap<V> {
	map: HashMap<usize, V>
}

impl<V> Heap<V> {
	pub fn new() -> Heap<V> {
		Heap {
			map: HashMap::new()
		}
	}

	pub fn get(&self, index: usize) -> Option<&V> {
		return self.map.get(&index);
	}

	pub fn alloc(&mut self, val: V) -> usize {
		let key = self.map.keys()
			.fold(0, |x, y| max(x, y+1));
		self.map.insert(key, val);
		return key;
	}

	pub fn dealloc(&mut self, index: usize) -> Option<V> {
		self.map.remove(&index)
	}
}

impl<V> Index<usize> for Heap<V> {
	type Output = V;

	fn index(&self, index: usize) -> &V {
		self.get(index).unwrap()
	}
}
