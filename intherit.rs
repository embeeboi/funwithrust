

pub struct Stuff
{
    width: u32,
    height: u32,
    length: u32,
}

pub fn code()
{
    let _bigbox = Stuff
    {
        width:50,
        height:50,
        length:50,

    };
    area(&_bigbox);
}

pub fn area(boxsize : &Stuff) -> u32
{
    boxsize.height*boxsize.width*boxsize.length
}