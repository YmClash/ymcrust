use rand::{Rng, thread_rng};


// #001
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//#002
pub fn random(start:i32, end:i32) ->i32 {
    let mut gen = thread_rng();
    let random = gen.gen_range(start..end);
    return random;

}

// #003
pub fn square(n:i32) ->i32{
    let square = n*n ;
    return square;

}
// #004
pub fn rectangle(longeur:i32, largeur:i32) ->i32{
    let aire = longeur*largeur ;
    return aire

}
// #005
pub fn triangle(longeur:i32, largeur:i32) ->i32{
    let aire = (longeur*largeur)/2;
    return aire;
}

// #006
pub fn trapez(longeur:i32, largeur:i32,hauteur:i32) ->i32{
    let aire = ((longeur+largeur)*hauteur)/2;
    return aire;
}

// #007
pub fn circle (rayon:i32) ->f64 {
    let aire = 3.14 * (rayon*rayon) as f64;
    return aire;
}

//008
pub fn ellipse(rayon:i32, rayon2:i32) ->f64 {
    let aire = 3.14 * (rayon*rayon2) as f64;
    return aire;
}

// #009
pub fn cube(a:i32) ->i32{
    let volume = a*a*a;
    return volume
}


// #010

pub fn quader(a:i32,b:i32,c:i32) ->i32 {
    let volume = a*b*c;
    return volume;
}
// #011

pub fn pyramide(a:i32,b:i32,c:i32) ->i32 {
    let volume = (a*b*c)/3;
    return volume;
}

// #012

pub fn zylinder(rayon:i32, hauteur:i32) ->f64 {
    let volume = 3.14 * (rayon*rayon) as f64 * hauteur as f64;
    return volume;
}


// #013

pub fn cone(rayon:i32,hauteur:i32) ->f64{
    let volume = (3.14 * (rayon*rayon) as f64 * hauteur as f64)/3.0;
    return volume;

}

// #014

pub fn sphere(rayon:i32) ->f64 {
    let volume = (4.0/3.0) * 3.14 * (rayon*rayon*rayon) as f64;
    return volume;
}


// Arithmetic

//015


pub fn factorial(n:i32) ->i32 {
    let mut fact = 1;
    for i in 1..n+1 {
        fact = fact * i;
    }
    return fact;
}

pub fn great_common_divisor(a:i32,b:i32) ->i32 {
    let mut i = 1;
    let mut pgcd = 1;
    while i <= a && i <= b {
        if a%i == 0 && b%i == 0 {
            pgcd = i;
        }
        i = i+1;
    }
    return pgcd;
}

pub fn is_prime(number:i32) ->bool{
    if number <= 1 {
        return false;
    }
    for  i in 2..number{
        if number % i == 0{
            return false;
        }
    }
    true
}