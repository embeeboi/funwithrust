unsafe fn hello()
{
    let x = 32;
    let y = 23;
    let pointx = &x;
    let pointy = &y;
    println!("{}",pointx);
    println!("{}",pointy);
    
}



fn main()
{
    unsafe
    {
        hello();
    }
}
