use std ::io;
fn main()-> io::Result<()> {
    println!("enter a number");
    let mut input1= String::new();
    io::stdin().read_line(&mut input1)?;
    println!("entered number: {}", input1);
    let num1: i32 = input1.trim().parse().unwrap();


    println!("enter the second number");
    let mut input2= String::new();
    io::stdin().read_line(&mut input2)?;
    println!("2nd entered numeber: {}", input2);
    let num2: i32 = input2.trim().parse().unwrap();
        
    println!("the sum is: {}", num1+num2);

    Ok(())
}