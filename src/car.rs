use macroquad::prelude::*;
use crate::vector::*;
use crate::board::*;

pub struct CarHistory {
	pub hist: Vec<Vector<2,usize>>,
}

impl CarHistory {
	pub fn possiblities() -> Vec<Vector<2,isize>> {
		vec![
			Vector::new([-1,-1]),
			Vector::new([-1,0]),
			Vector::new([-1,1]),
			Vector::new([0,-1]),
			Vector::new([0,0]),
			Vector::new([0,1]),
			Vector::new([1,-1]),
			Vector::new([1,0]),
			Vector::new([1,1]),
		]
	}

	pub fn new(start: Vector<2,usize>) -> Self {
		CarHistory{hist: vec![start;1]}
	}

	pub fn push(&mut self, pos: Vector<2,usize>) {
		self.hist.push(pos);
	}

	pub fn pop(&mut self) {
		if self.hist.len() > 1 {
			self.hist.pop();
		}
	}

	pub fn draw_full(&self, track: &Track) {
		let mut last_pos = track.bcoor(self.hist.last().expect("History should always contain at least one element as determined by constructor."));
		draw_circle(last_pos.get(0), last_pos.get(1), track.unit_dist() / 6.0, RED);

		for i in (0..self.hist.len()-1).rev() {
			let current_pos = track.bcoor(&self.hist[i]);
			draw_line(current_pos.get(0), current_pos.get(1), last_pos.get(0), last_pos.get(1), track.unit_dist() / 16.0, GRAY);
			last_pos = current_pos;
			draw_circle(last_pos.get(0), last_pos.get(1), track.unit_dist() / 6.0, RED);
		}
	}
}