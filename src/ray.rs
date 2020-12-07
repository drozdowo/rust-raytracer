pub mod ray {
    use crate::vector3::vector3::Vec3;

    pub struct Ray {
        pub origin: Vec3,
        pub dir: Vec3
    }

    impl Ray {
        pub fn new() -> Ray {
            Ray {
                origin: Vec3 {x: 0.0, y:0.0, z: 0.0},
                dir: Vec3 {x: 0.0, y: 0.0, z: 0.0}
            }
        }

        pub fn fetch_origin(self) -> Vec3 {
            return self.origin;
        }

        pub fn dir(self) -> Vec3 {
            return self.dir;
        }

        pub fn at(self, t: f32) -> Vec3 {
            return self.origin + self.dir*t;
        }

    }

}