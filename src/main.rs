fn main() {
    println!("rust7_conditions");
//operators: < > <= >= != ==
    let mut xcond: bool = 2 < 3;
    println!("2<3 is : {}",xcond);
    xcond = 2 <= 2;
    println!("2<=2 is : {}",xcond);
// by default float is f32
    xcond = (2 as f32) <= 2.2;
    println!("2<=2.2 is : {}",xcond);
// compound condition= conditionals are binded
// by &(and) and ||(or)
    let mut xcond1: bool = xcond && (false==true);
    println!("true&&false : {}", xcond1);
    xcond1 = false || xcond;
    println!("false or true is : {}", xcond1);
    xcond1 = !(false || xcond);
    println!("!(false||true)is: {}", xcond1);    
    xcond1 = false || !xcond;
    println!("(false||!true)is: {}", xcond1);  
// precedence of applying:() -> ! -> && -> ||
   xcond1 =  !true||true&&!false;
   println!("!true||true&&!false is {}", xcond1);
}
