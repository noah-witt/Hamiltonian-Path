use config::Movement;

pub struct Point {
    x: i32,
    y: i32
}
impl Clone for Point{
    fn clone(&self) -> Point
    {
        return Point {x:self.x,y:self.y};
    }
}
impl Copy for Point{}
impl Point {
    pub fn new(xl:i32,yl:i32) -> Point
    {
        return Point {x:xl,y:yl};
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn getX(&self) -> i32
    {
        return self.x;
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn getY(&self) -> i32
    {
        return self.y;
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn xAsUsize(&self) -> usize
    {
        return self.x as usize;
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn yAsUsize(&self) -> usize
    {
        return self.y as usize;
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn changeX(&mut self,chX:i32) -> i32
    {
        self.x+=chX;
        return self.x;
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn changeY(&mut self,chY:i32) -> i32
    {
        self.y+=chY;
        return self.y;
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn shift(&mut self,chX:i32, chY:i32) -> (i32,i32)
    {
        self.x+=chX;
        self.y+=chY;
        return (self.x,self.y);
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn moveToVector(&mut self,m:Movement)
    {
        self.shift(m.X,m.Y);
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    //returns string
    pub fn toString(&self) -> String
    {
        return format!("({},{})",self.x,self.y);
    }
}
