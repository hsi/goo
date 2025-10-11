use std::ops::Add;

pub struct Vector<T>(T, T, T);

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, vector: Vector<T>) -> Vector<T> {
        Vector::<T>(
            self.0 + vector.0,
            self.1 + vector.1,
            self.2 + vector.2,
        )
    }
}
