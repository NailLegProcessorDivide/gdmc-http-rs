use std::ops::{Add, Sub, BitAnd, BitOr, Shr, AddAssign};

use num_traits::{One, Zero};

pub trait VecT: Copy + Clone {}

impl VecT for i32{}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct V2d<T: VecT> {
    pub x: T,
    pub z: T
}

pub type V2di64 = V2d<i64>;
pub type V2di32 = V2d<i32>;
pub type V2di16 = V2d<i16>;
pub type V2di8 = V2d<i8>;

impl<T: VecT> V2d<T> {
    pub fn new(x: T, z: T) -> Self {
        Self { x, z }
    }
}

impl<T: VecT + Zero + One + PartialEq + AddAssign + Ord> V2d<T> {
    pub fn iter(&self) -> Iter2d<T> {
        assert!(self.ge(T::zero().into()));
        Iter2d {
            next: Self { x: T::zero(), z: T::zero() },
            target: self.clone()
        }
    }
}

impl<T: VecT> From<T> for V2d<T> {
    fn from(value: T) -> Self {
        Self { x: value.into(), z: value.into() }
    }
}

impl<R: Into<Self>, T: VecT + Add<Output = T>> Add<R> for V2d<T> {
    type Output = Self;

    fn add(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x + rhs.x, self.z + rhs.z)
    }
}

impl<R: Into<Self>, T: VecT + Sub<Output = T>> Sub<R> for V2d<T> {
    type Output = Self;

    fn sub(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x - rhs.x, self.z - rhs.z)
    }
}

impl<R: Into<Self>, T: VecT + BitAnd<Output = T>> BitAnd<R> for V2d<T>{
    type Output = Self;

    fn bitand(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x & rhs.x, self.z & rhs.z)
    }
}

impl<R: Into<Self>, T: VecT + BitOr<Output = T>> BitOr<R> for V2d<T> {
    type Output = Self;

    fn bitor(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x | rhs.x, self.z | rhs.z)
    }
}

impl<R: Into<Self>, T: VecT + Shr<Output = T>> Shr<R> for V2d<T> {
    type Output = Self;

    fn shr(self, rhs: R) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.x >> rhs.x, self.z >> rhs.z)
    }
}

impl<T: VecT + Ord> V2d<T> {

    pub fn gt(self, rhs: Self) -> bool {
        self.x > rhs.x && self.z > rhs.z
    }
    pub fn ge(self, rhs: Self) -> bool {
        self.x >= rhs.x && self.z >= rhs.z
    }
    pub fn lt(self, rhs: Self) -> bool {
        self.x < rhs.x && self.z < rhs.z
    }
    pub fn le(self, rhs: Self) -> bool {
        self.x <= rhs.x && self.z <= rhs.z
    }
}

impl Vec3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn iter(&self) -> Iter3d {
        assert!(self.x > 0 && self.y > 0 && self.z > 0);
        Iter3d {
            next: Vec3D { x: 0, y: 0, z: 0 },
            target: self.clone()
        }
    }
}

pub struct Iter2d<T: VecT + Zero + One + PartialEq + AddAssign> {
    next: V2d<T>,
    target: V2d<T>,
}

impl<T: VecT + Zero + One + PartialEq + AddAssign> Iterator for Iter2d<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.target.x == T::zero() || self.target.z == T::zero() || self.target.z == self.next.z {
            None
        }
        else {
            let res = (self.next.x, self.next.z);
            self.next.x += T::one();
            if self.next.x == self.target.x {
                self.next.x = T::zero();
                self.next.z += T::one();
            }
            Some(res)
        }
    }
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct Vec3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct Iter3d {
    next: Vec3D,
    target: Vec3D,
}

impl Iterator for Iter3d {
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.target.x == 0 || self.target.y == 0 || self.target.z == 0 || self.target.z == self.next.z {
            None
        }
        else {
            let res = (self.next.x, self.next.y, self.next.z);
            self.next.x += 1;
            if self.next.x == self.target.x {
                self.next.x = 0;
                self.next.y += 1;
                if self.next.y == self.target.y {
                    self.next.y = 0;
                    self.next.z += 1;
                }
            }
            Some(res)
        }
    }
}