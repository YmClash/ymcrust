// File: math_test.rs


#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
        use ymcrust::add;

        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]

    fn test_random(){
        use ymcrust::random;

        let random = random(1,10);
        let result = random;
        assert_eq!(result, if result >= 1 && result <= 10 {result}else{0});
    }

    #[test]
    fn test_square(){
        use ymcrust::square;
        let result = square(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_rectangle(){
        use ymcrust::rectangle;
        let result = rectangle(2,3);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_triangle(){
        use ymcrust::triangle;
        let result = triangle(2,3);
        assert_eq!(result,3);
    }

    #[test]
    fn test_trapez(){
        use ymcrust::trapez;
        let result = trapez(2,3,4);
        assert_eq!(result,10);
    }

    #[test]
    fn test_circle(){
        use ymcrust::circle;
        let result = circle(2);
        assert_eq!(result,12.56);
    }

    #[test]
    fn test_ellipsis(){
        use ymcrust::ellipse;
        let result= ellipse(2,3);
        assert_eq!(result,18.84);


    }

    #[test]
    fn test_cube(){
        use ymcrust::cube;
        let result = cube(2);
        assert_eq!(result,8);

    }

    #[test]
    fn test_quader(){
        use ymcrust::quader;
        let result = quader(2,3,4);
        assert_eq!(result,24);
    }

    #[test]
    pub fn test_pyramide(){
        use ymcrust::pyramide;
        let result = pyramide(2,3,4);
        assert_eq!(result,8);
    }

    #[test]
    pub fn test_zylinder(){
        use ymcrust::zylinder;
        let result = zylinder(2,3);
        assert_eq!(result,37.68);
    }

    #[test]
    pub fn test_cone(){
        use ymcrust::cone;
        let result = cone(2,3);
        assert_eq!(result,12.56);
    }

    #[test]
    fn test_sphere(){
        use ymcrust::sphere;
        let result = sphere(2);
        assert_eq!(result,33.49333333333333);
    }

    #[test]
    fn test_factorial(){
        use ymcrust::factorial;
        let result = factorial(5);
        assert_eq!(result,120);
    }

    #[test]
    fn test_great_divisor(){
        use ymcrust::great_common_divisor;
        let result = great_common_divisor(10,5);
        assert_eq!(result,5);
    }

    #[test]
    fn test_is_prime(){
        use ymcrust::is_prime;
        let result = is_prime(5);
        assert_eq!(result,true);
        assert_eq!(is_prime(4),false);
        assert_eq!(is_prime(1),false);
        assert_eq!(is_prime(97),true);
        assert_eq!(is_prime(100),false);
    }




}

