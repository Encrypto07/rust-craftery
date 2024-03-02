fn main() {
    
    // in rust we creata variable by using let,
    // in rust we can put variable nature, like it mutable or unmutable
    // if we creat a variable by let it a unmutable in nature, 
    //if we want a vairiable to be mutable in nature then after let we need to put 'mut'
    // { NOTE } if the var is mutable in nature, that does not mean we resassign a 'string' value to int type variable

    // unmutable variable 
    let x = 6;
    println!("the value of x is { }", x);

    // created a mutable by mut
    let mut y = 12;
    println!("the value of y is { }", y);

    y = 36;
    println!("After reassigning the value of y is {}", y);


    // y = "adasdsa";  -> this will give an error
}
