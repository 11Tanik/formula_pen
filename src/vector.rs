use std::ops::Add;
use std::ops::Sub;
use num_traits::PrimInt;
use num_traits::NumCast;
use num_traits::ToPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector<const D: usize, T> {
	v: [T;D],
}

impl<const D: usize, T> Vector<D, T> {
	pub fn new(v: [T;D]) -> Self {
		Vector::<D,T>{v}
	}
}

impl<const D: usize> From<Vector<D,usize>> for Vector<D, isize> {
	fn from(value: Vector<D, usize>) -> Self {
		let mut r = [0;D];
		for i in 0..D {
			r[i] = value.v[i] as isize;
		}
		return Vector::new(r);
	}
}

impl<const D: usize> From<Vector<D,isize>> for Vector<D, usize> {
	fn from(value: Vector<D, isize>) -> Self {
		let mut r = [0;D];
		for i in 0..D {
			r[i] = value.v[i] as usize;
		}
		return Vector::new(r);
	}
}

impl<const D: usize, T: Clone + Copy> Vector<D,T> {
	pub fn get(&self, i: usize) -> T {
		return self.v[i];
	}
}

impl<const D: usize> Vector<D, f32> {
	pub fn euclid_dist(self, other: Self) -> f32 {
		let mut sum = 0.0;
		for i in 0..D {
			sum += (self.v[i]-other.v[i])*(self.v[i]-other.v[i]);
		}
		return sum.sqrt();
	}
}

impl<const D: usize, S: Clone + Copy, T: Add<S> + Clone + Copy> Add<Vector<D,S>> for Vector<D, T> 
	where
		T::Output: Default + Copy
	{
	type Output = Vector<D,T::Output>;
	fn add(self, other: Vector<D,S>) -> Vector<D, T::Output> {
		let mut r = [T::Output::default();D];
		for i in 0..D {
			r[i] = self.v[i].add(other.v[i]);
		}
		return Vector::new(r);
	}
}

impl<const D: usize, S: Clone + Copy, T: Sub<S> + Clone + Copy> Sub<Vector<D,S>> for Vector<D, T> 
	where
		T::Output: Default + Copy
	{
	type Output = Vector<D,T::Output>;
	fn sub(self, other: Vector<D,S>) -> Vector<D, T::Output> {
		let mut r = [T::Output::default();D];
		for i in 0..D {
			r[i] = self.v[i].sub(other.v[i]);
		}
		return Vector::new(r);
	}
}