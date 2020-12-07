pub mod vector3 {

    pub use std::array;
    pub use std::ops;
    use std::ops::Index;

    #[derive(Copy, Clone)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    pub type Color = Vec3;

    impl ops::Neg for Vec3 {
        type Output = Vec3;
        fn neg(self) -> Vec3 {
            Vec3{
                x: -self.x,
                y: -self.y,
                z: -self.z
            }
        }
    }

    //Division Operator Overrides
    impl ops::DivAssign<Vec3> for Vec3 {
        fn div_assign(&mut self, other: Vec3) -> () {
            self.x *= 1.0/other.x;
            self.y *= 1.0/other.y;
            self.z *= 1.0/other.z;
        }
    }

    impl ops::DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, other: f64) -> () {
                *self *= 1.0/other;
            }
    }

    //Multiplication Operator Overrides
    impl ops::MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, other: f64) -> () {
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

    impl ops::AddAssign<f64> for Vec3 {
        fn add_assign(&mut self, other: f64) -> () {
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

    impl ops::SubAssign<f64> for Vec3 {
        fn sub_assign(&mut self, other: f64) -> () {
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
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z
            }
        }
    }

    impl ops::Sub<Vec3> for Vec3 {
        type Output = Vec3;
        fn sub(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z
            }
        }
    }

    impl ops::Mul<Vec3> for Vec3 {
        type Output = Vec3;
        fn mul(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z
            }
        }
    }

    impl ops::Div<Vec3> for Vec3 {
        type Output = Vec3;
        fn div(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z
            }
        }
    }

    //For f64
    impl ops::Add<f64> for Vec3 {
        type Output = Vec3;
        fn add(self, other: f64) -> Vec3 {
            Vec3 {
                x: self.x + other,
                y: self.y + other,
                z: self.z + other
            }
        }
    }

    impl ops::Sub<f64> for Vec3 {
        type Output = Vec3;
        fn sub(self, other: f64) -> Vec3 {
            Vec3 {
                x: self.x - other,
                y: self.y - other,
                z: self.z - other
            }
        }
    }

    impl ops::Mul<f64> for Vec3 {
        type Output = Vec3;
        fn mul(self, other: f64) -> Vec3 {
            Vec3 {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other
            }
        }
    }

    impl ops::Div<f64> for Vec3 {
        type Output = Vec3;
        fn div(self, other: f64) -> Vec3 {
            Vec3 {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other
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

        pub fn to_string(&self) -> String {
            return format!("x: {:.10} y: {:.10} z: {:.10}", self.x, self.y, self.z);
        }


        pub fn length(&self) -> f64 {
            return self.length_squared().sqrt();
        }

        pub fn length_squared(&self) -> f64 {
            return self.x*self.x + self.y*self.y + self.z*self.z;
        }

        pub fn unit_vector(self) -> Vec3 {
            return self/self.length();
        }

        pub fn dot_product(&self, other: Vec3) -> f64 {
            return self.x*other.x + self.y*other.y + self.z*other.z;
        }

        pub fn cross_product(&self, other: Vec3) -> Vec3{
            return Vec3 {
                x: self.y*other.z-self.z*other.y,
                y: self.z*other.x-self.x*other.z,
                z: self.x*other.y-self.y*other.x
            }
        }

        pub fn print_color(&self) -> () {print!("{}\t{}\t{} ", (self.x * 255.999) as i32, (self.y * 255.999) as i32, (self.z * 255.999) as i32);}

    }
}
