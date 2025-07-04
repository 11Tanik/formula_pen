use macroquad::prelude::*;
use crate::vector::*;
use crate::car::*;
use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, PartialEq, Eq)]
pub enum TrackPoint {
	TRACK,
	OUT,
}

pub struct Track {
	grid: Vec<Vec<TrackPoint>>,
	width: usize,
	height: usize,
	starts: Vec<Vector<2,usize>>,
	goals: Vec<Vector<2,usize>>,
	used_starts: usize,
}

impl Track {
	pub fn new(grid: Vec<Vec<TrackPoint>>, starts: Vec<Vector<2,usize>>, goals: Vec<Vector<2,usize>>) -> Self {
		Track{width: grid[0].len(), height: grid.len(), starts, goals, used_starts: 0, grid}
	}

	pub fn sandbox(width: usize, height: usize, starts: Vec<Vector<2,usize>>, goals: Vec<Vector<2,usize>>) -> Self {
		let grid = vec![vec![TrackPoint::TRACK;width];height];
		Track{grid, width, height, starts, goals, used_starts: 0}
	}

	pub fn bcoor(&self, v: &Vector<2,usize>) -> Vector<2,f32> {
		assert!(v.get(0) < self.width);
		assert!(v.get(1) < self.height);

		let point_dist = self.unit_dist();

		let x_left = (screen_width() - point_dist*self.width as f32) / 2.0;
		let y_up = (screen_height() - point_dist*self.height as f32) / 2.0;

		let x_pos = x_left + point_dist/2.0 + point_dist*v.get(0) as f32;
		let y_pos = y_up + point_dist/2.0 + point_dist*v.get(1) as f32;
		return Vector::new([x_pos, y_pos]);
	}

	pub fn bdist(&self, v: &Vector<2,usize>) -> Vector<2,f32> {
		assert!(v.get(0) < self.width);
		assert!(v.get(1) < self.height);

		let point_dist = self.unit_dist();

		let x_dist = point_dist*v.get(0) as f32;
		let y_dist = point_dist*v.get(1) as f32;
		return Vector::new([x_dist, y_dist]);
	}

	pub fn unit_dist(&self) -> f32 {
		let x_dist = screen_width() / self.width as f32;
		let y_dist: f32 = screen_height() / self.height as f32;
		return x_dist.min(y_dist);
	}

	pub fn get_free_start(&mut self) -> Vector<2,usize> {
		let r = self.starts[self.used_starts];
		self.used_starts += 1;
		return r;
	}

	pub fn valid_points(&self, car: &CarHistory) -> Vec<Vector<2,usize>> {
		let mut valids = Vec::new();

		let mut center = car.hist.last().expect("History should always contain at least one element as determined by constructor.").clone();

		if car.hist.len() > 1 {
			let ic: Vector<2,isize> = center.into();
			let il: Vector<2,isize> = car.hist[car.hist.len()-2].into();
			let diff = ic - il;
			let r: Vector<2, isize> = ic + diff;
			center = r.into();
		}

		let icenter: Vector<2,isize> = center.into();
		for p in CarHistory::possiblities() {
			let pos = p.add(icenter);
			if pos.get(0) >= 0
				&& pos.get(0) < self.width as isize
				&& pos.get(1) >= 0
				&& pos.get(1) < self.height as isize
				&& self.grid[pos.get(1) as usize][pos.get(0) as usize] == TrackPoint::TRACK {
				valids.push(pos.into());
			}
		}
		return valids;
	}

	pub fn draw_valid_points(&self, car: &CarHistory) {
		for v in self.valid_points(car) {
			let pos = self.bcoor(&v);
			draw_circle(pos.get(0), pos.get(1), self.unit_dist() / 7.0, GREEN);
		}
	}

	pub fn draw_full(&self) {
		clear_background(DARKGREEN);
		
		let rect_begin = self.bcoor(&Vector::new([0,0]));
		let rect_width = self.bdist(&Vector::new([self.width-1,self.height-1]));
		draw_rectangle(rect_begin.get(0), rect_begin.get(1), rect_width.get(0), rect_width.get(1), WHITE);

		for y in 0..self.height {
			for x in 0..self.width {
				if self.grid[y][x] == TrackPoint::TRACK {
					let circle_pos = self.bcoor(&Vector::new([x,y]));
					draw_circle(circle_pos.get(0), circle_pos.get(1), self.unit_dist() / 8.0, BLACK);
				}
			}
		}

		for g in &self.goals {
			let goal_pos = self.bcoor(&g);
				draw_circle(goal_pos.get(0), goal_pos.get(1), self.unit_dist() / 4.0, YELLOW);
		}

		let highlight = self.pos_close_to_mouse();
		if let Some(v) = highlight {
			let pos = self.bcoor(&v);
			draw_circle(pos.get(0), pos.get(1), self.unit_dist() / 5.0, BLUE);
		}

        let text = format!("Ratio: {}", 5);
        let font_size = 30.;
        let text_size = measure_text(&text, None, font_size as _, 1.0);

        draw_text(
            &text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );
	}

	pub fn pos_close_to_mouse(&self) -> Option<Vector<2,usize>> {
		let (x_m, y_m) = mouse_position();
		let mv = Vector::new([x_m,y_m]);
		for y in 0..self.height {
			for x in 0..self.width {
				if mv.euclid_dist(self.bcoor(&Vector::new([x,y]))) < self.unit_dist() / 4.0 {
					return Some(Vector::new([x,y]));
				}
			}
		}
		return None;
	}
}