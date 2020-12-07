pub mod vector3 {

    pub use std::array;
    pub use std::ops;
    use std::ops::Index;
    use crate::vector3::vector3::ops::IndexMut;

    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    pub type Color = Vec3;

    //Override index selection
    impl Index<usize> for Vec3 {
        type Output = f32;
        fn index(&self, index: usize) -> &f32 {
            if index == 0 {
                &self.x
            } else {
                if index == 1 {
                   &self.y
                } else {
                    &self.z
                }
            }
        }
    }

    impl IndexMut<usize> for Vec3 {
        fn index_mut(&mut self, index :usize) -> &mut f32 {
            if index == 0 {
                return &mut self.x
            } else {
                if index == 1 {
                    &mut self.y
                } else {
                    &mut self.z
                }
            }
        }
    }

    //Division Operator Overrides
    impl ops::DivAssign<Vec3> for Vec3 {
        fn div_assign(&mut self, other: Vec3) -> () {
            self.x /= other.x;
            self.y /= other.y;
            self.z /= other.z;
        }
    }

    impl ops::DivAssign<f32> for Vec3 {
        fn div_assign(&mut self, other: f32) -> () {
            self.x /= other;
            self.y /= other;
            self.z /= other;
        }
    }

    //Multiplication Operator Overrides
    impl ops::MulAssign<f32> for Vec3 {
        fn mul_assign(&mut self, other: f32) -> () {
            self.x *= other;
            self.y *= other;
            self.z *= other;
        }
    }

    impl ops::MulAssign<Vec3> for Vec3 {
        fn mul_assign(&mut self, other: Vec3) -> () {
            self.x *= other.x;
            self.y *= other.y;
            self.z *= other.z;
        }
    }

    //Addition Operation Overrides
    impl ops::AddAssign<Vec3> for Vec3 {
        fn add_assign(&mut self, other: Vec3) -> () {
            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
        }
    }

    impl ops::AddAssign<f32> for Vec3 {
        fn add_assign(&mut self, other: f32) -> () {
            self.x += other;
            self.y += other;
            self.z += other;
        }
    }

    //Subtraction Operation Overrides
    impl ops::SubAssign<Vec3> for Vec3 {
        fn sub_assign(&mut self, other: Vec3) -> () {
            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;
        }
    }

    impl ops::SubAssign<f32> for Vec3 {
        fn sub_assign(&mut self, other: f32) -> () {
            self.x -= other;
            self.y -= other;
            self.z -= other;
        }
    }


    //Regular Operators (+,*,/,-)
    //For Other Vec3
    impl ops::Add<Vec3> for Vec3 {
        type Output = Vec3;
        fn add(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self[0] + other[0],
                y: self[1] + other[1],
                z: self[0] + other[0]
            }
        }
    }

    impl ops::Sub<Vec3> for Vec3 {
        type Output = Vec3;
        fn sub(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self[0] - other[0],
                y: self[1] - other[1],
                z: self[0] - other[0]
            }
        }
    }

    impl ops::Mul<Vec3> for Vec3 {
        type Output = Vec3;
        fn mul(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self[0] * other[0],
                y: self[1] * other[1],
                z: self[0] * other[0]
            }
        }
    }

    impl ops::Div<Vec3> for Vec3 {
        type Output = Vec3;
        fn div(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self[0] / other[0],
                y: self[1] / other[1],
                z: self[0] / other[0]
            }
        }
    }

    //For f32
    impl ops::Add<f32> for Vec3 {
        type Output = Vec3;
        fn add(self, other: f32) -> Vec3 {
            Vec3 {
                x: self[0] + other,
                y: self[1] + other,
                z: self[0] + other
            }
        }
    }

    impl ops::Sub<f32> for Vec3 {
        type Output = Vec3;
        fn sub(self, other: f32) -> Vec3 {
            Vec3 {
                x: self[0] - other,
                y: self[1] - other,
                z: self[0] - other
            }
        }
    }

    impl ops::Mul<f32> for Vec3 {
        type Output = Vec3;
        fn mul(self, other: f32) -> Vec3 {
            Vec3 {
                x: self[0] * other,
                y: self[1] * other,
                z: self[0] * other
            }
        }
    }

    impl ops::Div<f32> for Vec3 {
        type Output = Vec3;
        fn div(self, other: f32) -> Vec3 {
            Vec3 {
                x: self[0] / other,
                y: self[1] / other,
                z: self[0] / other
            }
        }
    }

    impl Vec3 {
        pub fn new() -> Self {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
        pub fn x(&self) -> f32 {
            return self.x;
        }
        pub fn y(&self) -> f32 {
            return self.y;
        }
        pub fn z(&self) -> f32 {
            return self.z;
        }
        pub fn to_string(&self) -> String {
            return format!("x: {} y: {} z: {}", self.x, self.y, self.z);
        }

        pub fn get_index(&mut self, index :usize) -> &mut f32 {
            return &mut self[index];
        }

        pub fn length(&self) -> f32 {
            return self.length_squared().sqrt();
        }

        pub fn length_squared(&self) -> f32 {
            return self.x*self.x + self.y*self.y + self.z*self.z;
        }

        pub fn unit_vector(&self) -> Vec3 {
            return Vec3{..*self}/self.length();
        }

        pub fn print_color(&self) -> () {print!("{} {} {} ", self.x as i32, self.y as i32, self.z as i32);}

    }
}
