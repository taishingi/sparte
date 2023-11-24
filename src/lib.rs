pub mod spartans {

    #[derive(Default)]
    pub struct Maths {}

    impl Maths {
        pub fn new(description: &str) -> Maths {
            println!("\n{}\n", description);
            Self {}
        }

        pub fn describe(
            &mut self,
            description: &str,
            it: fn(&mut Maths) -> &mut Maths,
        ) -> &mut Maths {
            println!("\n\n{}\n", description);
            it(self)
        }

        pub fn theory(&mut self, description: &str, it: fn(&mut Maths) -> bool) -> &mut Maths {
            println!("\n\n{}\n", description);
            assert!(it(self));
            self
        }

        pub fn theorem(
            &mut self,
            description: &str,
            expected: bool,
            it: fn(&mut Maths) -> bool,
        ) -> &mut Maths {
            println!("\n\n{}\n", description);
            assert_eq!(expected, it(self));
            self
        }

        pub fn equals(&mut self, a: f32, b: f32) -> bool {
            a == b
        }

        pub fn unequals(&mut self, a: f32, b: f32) -> bool {
            a != b
        }

        pub fn expect(&mut self, f: fn(&mut Maths) -> f32, x: f32) -> bool {
            f(self) == x
        }

        pub fn add(&mut self, a: u32, b: u32) -> u32 {
            a + b
        }

        pub fn exp(&mut self, a: f32) -> f32 {
            a.exp()
        }

        pub fn exp2(&mut self, a: f32) -> f32 {
            a.exp2()
        }

        pub fn ln(&mut self, a: f32) -> f32 {
            a.ln()
        }

        pub fn log(&mut self, a: f32, b: f32) -> f32 {
            a.log(b)
        }

        pub fn log10(&mut self, a: f32) -> f32 {
            a.log10()
        }

        pub fn log2(&mut self, a: f32) -> f32 {
            a.log2()
        }

        pub fn acos(&mut self, b: f32) -> f32 {
            b.acos()
        }

        pub fn acosh(&mut self, b: f32) -> f32 {
            b.acosh()
        }

        pub fn asin(&mut self, b: f32) -> f32 {
            b.asin()
        }

        pub fn asinh(&mut self, b: f32) -> f32 {
            b.asinh()
        }

        pub fn atan(&mut self, b: f32) -> f32 {
            b.atan()
        }

        pub fn atanh(&mut self, b: f32) -> f32 {
            b.atanh()
        }

        pub fn max<T: PartialOrd>(&mut self, a: T, b: T) -> T {
            if a < b {
                b
            } else {
                a
            }
        }

        pub fn min<T: PartialOrd>(&mut self, a: T, b: T) -> T {
            if a > b {
                b
            } else {
                a
            }
        }

        pub fn between(&mut self, actual: f32, y: f32, z: f32) -> bool {
            actual > y && actual < z
        }

        pub fn div(&mut self, a: f32, b: f32) -> f32 {
            a / b
        }

        pub fn sqrt(&mut self, a: f32) -> f32 {
            a.sqrt()
        }

        pub fn floor(&mut self, a: f32) -> f32 {
            a.floor()
        }

        pub fn ceil(&mut self, a: f32) -> f32 {
            a.ceil()
        }

        pub fn cos(&mut self, a: f32) -> f32 {
            a.cos()
        }

        pub fn sin(&mut self, a: f32) -> f32 {
            a.sin()
        }

        pub fn tan(&mut self, a: f32) -> f32 {
            a.tan()
        }

        pub fn pow(&mut self, a: u32, b: u32) -> u32 {
            a.pow(b)
        }

        pub fn hypot(&mut self, a: f32, b: f32) -> f32 {
            a.hypot(b)
        }
    }
}

#[cfg(test)]
mod test {

    use crate::spartans::Maths;

    fn triangle(m: &mut Maths) -> f32 {
        m.hypot(4.0, 3.0)
    }

    fn is_rectangle(m: &mut Maths) -> bool {
        m.hypot(3.0, 4.0) == 5.0
    }

    fn pythagore(m: &mut Maths) -> &mut Maths {
        m.theorem("hypotenuse must be equals to 5", true, is_rectangle)
            .theory("The triangle must be rectangle", is_rectangle)
    }

    #[test]
    pub fn pow() {
        let mut m = Maths::new("pow");
        assert_eq!(4, m.pow(2, 2));
        assert_eq!(8, m.pow(2, 3));
        assert_eq!(16, m.pow(2, 4));
    }

    #[test]
    pub fn add() {
        let mut m = Maths::new("add");
        assert_eq!(4, m.add(2, 2));
        assert_eq!(8, m.add(4, 4));
    }

    #[test]
    pub fn min() {
        let mut m = Maths::new("min");
        assert_eq!(2, m.min(2, 4));
        assert_eq!(7, m.min(77, 7));
    }

    #[test]
    pub fn sqrt() {
        let mut m = Maths::new("sqrt");
        assert_eq!(5.0, m.sqrt(25.0));
    }

    #[test]
    pub fn max() {
        let mut m = Maths::new("max");
        assert_eq!(4, m.max(2, 4));
        assert_eq!(77, m.max(77, 7));
    }

    #[test]
    pub fn cos() {
        let mut m = Maths::new("cos");
        assert_eq!(1.0, m.cos(0.0));
        assert_eq!(-0.44807363, m.cos(90.0));
    }

    #[test]
    pub fn sin() {
        let mut m = Maths::new("sin");
        assert_eq!(0.0, m.sin(0.0));
        assert_eq!(0.89399666, m.sin(90.0));
    }

    #[test]
    pub fn tan() {
        let mut m = Maths::new("tan");
        assert_eq!(0.0, m.tan(0.0));
    }

    #[test]
    pub fn floor() {
        let mut m = Maths::new("floor");
        assert_eq!(3.0, m.floor(3.4));
    }

    #[test]
    pub fn ceil() {
        let mut m = Maths::new("ceil");
        assert_eq!(4.0, m.ceil(3.8));
    }

    #[test]
    pub fn hypot() {
        let mut m = Maths::new("hypot");
        assert_eq!(5.0, m.hypot(3.0, 4.0));
    }

    #[test]
    pub fn div() {
        let mut m = Maths::new("div");
        assert_eq!(0.5, m.div(1.0, 2.0));
    }

    #[test]
    pub fn log2() {
        let mut m = Maths::new("log2");
        assert_eq!(3.0, m.log2(11.0).floor());
    }

    #[test]
    pub fn log10() {
        let mut m = Maths::new("log10");
        assert_eq!(3.0, m.log10(1000.0));
    }

    #[test]
    pub fn log() {
        let mut m = Maths::new("log");
        assert_eq!(-0.0, m.log(1.0, 0.0));
    }

    #[test]
    pub fn equals() {
        let mut m = Maths::new("equals");
        assert_eq!(true, m.equals(1.0, 1.0));
    }

    #[test]
    pub fn unequals() {
        let mut m = Maths::new("unequals");
        assert_eq!(true, m.unequals(1.0, 14.0));
    }

    #[test]
    pub fn expect() {
        let mut m = Maths::new("expect");
        assert_eq!(true, m.expect(triangle, 5.0));
    }

    #[test]
    pub fn between() {
        let mut m = Maths::new("between");
        let x = triangle(&mut m);
        assert_eq!(true, m.between(x, 4.0, 6.0));
    }

    #[test]
    pub fn theory() {
        let mut m = Maths::new("theory");
        m.describe("test pythagore theorem", pythagore);
    }
}
