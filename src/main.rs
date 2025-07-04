mod board;
use crate::board::*;
use crate::board::TrackPoint::*;

mod car;
use crate::car::*;

mod vector;
use crate::vector::*;

use macroquad::prelude::*;

fn track() -> Track {
	let grid: Vec<Vec<TrackPoint>> = vec![
		vec![TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, OUT, OUT,OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, OUT, OUT, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, OUT, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK],
		vec![TRACK, TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT],
		vec![TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT, TRACK, TRACK, TRACK, TRACK, OUT, OUT],
		vec![OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT],
		vec![OUT, OUT, TRACK, TRACK, TRACK, TRACK, TRACK, TRACK, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT, OUT],
	];


	return Track::new(grid, vec![Vector::new([1,1])], vec![Vector::new([2,1])]);
}

#[macroquad::main("Formula Pen")]
async fn main() {

	let mut track = track();
	let mut car = CarHistory::new(track.get_free_start());

       loop {

		if is_mouse_button_released(MouseButton::Left) {
			let valids = track.valid_points(&car);
			if valids.is_empty() {
				println!("Crashed!");
				car.push(car.hist.last().unwrap().clone());
			} else {
				if let Some(p) = track.pos_close_to_mouse() {
					
					for q in valids {
						if p == q {
							car.push(p);
						}
					}
				}
			}
		}

		if is_key_released(KeyCode::Left) {
			car.pop();
		}
        
		track.draw_full();

		track.draw_valid_points(&car);

		car.draw_full(&track);
        
        next_frame().await;
	}
}
