pub fn triangle(n :i32){
    for i in (0..n).rev(){
        let mut asteric = String::new();
        for _ in 0..i+1{
            asteric.push('*');
        };
        println!("{}",asteric);
    }

}

pub fn triangle_inv(n :i32){
    for i in 0..n{
        let mut asteric = String::new();
        for _ in 0..i+1{
            asteric.push('*');
        };
        println!("{}",asteric);
    }

}

pub fn triangle_inv_odd(n :i32){
    for i in 0..n{
        let mut asteric = String::new();
        if i%2==0{
            for _ in 0..i+1{
                asteric.push('*');
            }
            println!("{}",asteric);
        }
        
    }

}
