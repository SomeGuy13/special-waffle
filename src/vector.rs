// A simple vector class

#[derive(Debug)]
#[derive(PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    // Creates a new vector
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {x, y, z}
    }

    // Creates a new vector at zero
    fn zero() -> Self {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }

    // The following 3 functions return their respective
    // values within the vector
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    } 

    fn z(&self) -> f64 {
        self.z
    }

    // Makes the vector negative
    fn negateive(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    // Returns the value at an index of the vector
    fn index(&self, i: usize) -> f64 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Index out of bounds"),
        }
    }

    // Returns a mutable value at an index of the vector
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }

    // Adds two vectors together
    fn add_to(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    // Subtracts two vectors
    fn subtract_from(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    // Multiplies two vectors
    fn multiply_by(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    // Divides two vectors
    fn divide_by(&self, other: &Self) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    // Returns the dot product of two vectors
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Returns the cross product of two vectors
    fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Normalizes the vector
    fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Vec3{
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    // Returns the magnitude of the vector
    fn magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

// Impl block for operation symbols

// Addition

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        self.add_to(&other)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

// Subtraction

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        self.subtract_from(&other)
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

// Multiplication

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        self.multiply_by(&other)
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

// Multiplication

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Self::Output {
        self.divide_by(&other)
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let vec = Vec3::new(1.23, 4.56, 7.89);

        assert_eq!(1.23, vec.x);
        assert_eq!(4.56, vec.y);
        assert_eq!(7.89, vec.z);
    }

    #[test]
    fn test_vector_zero_creation() {
        let vec = Vec3::zero();

        assert_eq!(0.0, vec.x);
        assert_eq!(0.0, vec.y);
        assert_eq!(0.0, vec.z);
    }

    #[test]
    fn test_vector_xyz() {
        let vec = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(1.0, vec.x());
        assert_eq!(2.0, vec.y());
        assert_eq!(3.0, vec.z());
    }

    #[test]
    fn test_vector_negate() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec.negateive();

        assert_eq!(-1.0, vec.x());
        assert_eq!(-2.0, vec.y());
        assert_eq!(-3.0, vec.z());
    }

    #[test]
    fn test_vector_index() {
        let vec = Vec3::new(53.32, 432.32, 23.12);

        assert_eq!(53.32, vec.index(0));
        assert_eq!(432.32, vec.index(1));
        assert_eq!(23.12, vec.index(2))
    }

    #[test]
    fn test_vector_index_mutable() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        *vec.index_mut(0) = 4.0;
        *vec.index_mut(1) = 5.0;
        *vec.index_mut(2) = 6.0;

        assert_eq!(4.0, vec.x());
        assert_eq!(5.0, vec.y());
        assert_eq!(6.0, vec.z());
    }

    #[test]
    fn test_vector_addition() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let vec3 = vec1.add_to(&vec2);

        assert_eq!(Vec3::new(2.0, 2.0, 2.0), vec3);
    }

    #[test]
    fn test_vector_subtraction() {
        let vec1 = Vec3::new(4.0, 4.0, 4.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let vec3 = vec1.subtract_from(&vec2);

        assert_eq!(Vec3::new(3.0, 3.0, 3.0), vec3);
    }

    #[test]
    fn test_vector_multiplication() {
        let vec1 = Vec3::new(4.0, 4.0, 4.0);
        let vec2 = Vec3::new(2.0, 2.0, 2.0);
        let vec3 = vec1.multiply_by(&vec2);

        assert_eq!(Vec3::new(8.0, 8.0, 8.0), vec3);
    }

    #[test]
    fn test_vector_division() {
        let vec1 = Vec3::new(4.0, 4.0, 4.0);
        let vec2 = Vec3::new(2.0, 2.0, 2.0);
        let vec3 = vec1.divide_by(&vec2);

        assert_eq!(Vec3::new(2.0, 2.0, 2.0), vec3);
    }

    #[test]
    fn test_vector_dot() {
        let vec1 = Vec3::new(2.4, 3.5, 3.5);
        let vec2 = Vec3::new(5.3, 3.6, 3.6);
        let dot = vec1.dot(&vec2);

        assert_eq!(37.92, dot);
    }

    #[test]
    fn test_vector_cross() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let cross = vec1.cross(&vec2);

        assert_eq!(Vec3::new(-3.0, 6.0, -3.0), cross);
    }

    #[test]
    fn test_vector_addition_operation() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let vec3 = vec1 + vec2;

        assert_eq!(Vec3::new(2.0, 2.0, 2.0), vec3);
    }

    #[test]
    fn test_vector_addition_operation_assignment() {
        let mut vec1 = Vec3::new(1.0, 1.0, 1.0);
        vec1 += Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(Vec3::new(3.0, 3.0, 3.0), vec1);
    }

    #[test]
    fn test_vector_subtraction_operation() {
        let vec1 = Vec3::new(2.0, 2.0, 2.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let vec3 = vec1 - vec2;

        assert_eq!(Vec3::new(1.0, 1.0, 1.0), vec3);
    }

    #[test]
    fn test_vector_subtraction_operation_assignment() {
        let mut vec1 = Vec3::new(5.0, 5.0, 5.0);
        vec1 -= Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(Vec3::new(3.0, 3.0, 3.0), vec1);
    }

    #[test]
    fn test_vector_multiplication_operation() {
        let vec1 = Vec3::new(2.0, 2.0, 2.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let vec3 = vec1 * vec2;

        assert_eq!(Vec3::new(2.0, 2.0, 2.0), vec3);
    }

    #[test]
    fn test_vector_multiplication_operation_assignment() {
        let mut vec1 = Vec3::new(5.0, 5.0, 5.0);
        vec1 *= Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(Vec3::new(10.0, 10.0, 10.0), vec1);
    }

    #[test]
    fn test_vector_division_operation() {
        let vec1 = Vec3::new(2.0, 2.0, 2.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        let vec3 = vec1 / vec2;

        assert_eq!(Vec3::new(2.0, 2.0, 2.0), vec3);
    }

    #[test]
    fn test_vector_division_operation_assignment() {
        let mut vec1 = Vec3::new(5.0, 5.0, 5.0);
        vec1 /= Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(Vec3::new(2.5, 2.5, 2.5), vec1);
    }
}
