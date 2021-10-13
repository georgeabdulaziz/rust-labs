//struct Poly([i32;10], usize);

// use std::ops::AddAssign;
// use std::ops::Mul;
//use std::ops::{Add, Sub, Mul};
//use num_traits::Float;
//use num::{Float, NumCast};


fn main(){
    let new_poly = Poly::new([3,4,0,2,0,0,0,0,0,0]);
    println!("{}", new_poly.eval(2.0));
}

pub struct Poly {
    coefficients : [i32;10],
    degree : usize,
}


impl Poly {
    pub fn new(coefficients1: [i32;10]) -> Poly {
        Poly {
            coefficients: coefficients1,
            degree :  degreeVal(coefficients1),
        } 
    }
}


impl Poly {
    pub fn eval(&self,  val: f32) -> f32 {
        let mut results : f32 = 0.0;
        //println!("{}", self.degree);
        for i in (0..(self.degree+1)) {
            results += ((self.coefficients[i])*((val as i32).pow(i as u32))) as f32;
        }
        results
    }
}



fn degreeVal(arr : [i32;10]) -> usize {
    let mut degree1 = 0;
    for n in (0..10) {
        if arr[n] > 0 {degree1 = n;}
    }
    degree1
}
