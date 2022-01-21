
    #[test]
    #[should_panic(expected = "assertion failed")]
    pub fn test_tuple() {
        let tup = (1, 2, 3);
        let (x, y, z) = tup;
        println!("{x},{y},{z}",x=x,y=y,z=z);
        assert_eq!(2, x)
    }

    #[test]
    pub fn test_array() {
        let arr = [1, 2, 3, 4, 5];
        for i in arr {
            println!("{i}",i=i)
        }
    }

