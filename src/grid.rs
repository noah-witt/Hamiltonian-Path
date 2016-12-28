use point::Point;
pub struct Grid
{
    grid: [[bool; 10]; 10]
}
impl Clone for Grid{
    fn clone(&self) -> Grid
    {
        let mut g = Grid::new();
        for x in 0..10
        {
            for y in 0..10
            {
                g.set(Point::new(x,y),self.get(Point::new(x,y)));
            }
        }
        return g;
    }
}
impl Copy for Grid{}
impl Grid
{
    #[allow(unused_mut)]
    pub fn new() -> Grid
    {
        /*let mut obj:Grid = Grid {
            grid: {
                {false,false,false,false},
                {false,false,false,false},
                {false,false,false,false},
                {false,false,false,false}
            }
        };*/
        let mut obj:Grid = Grid {grid:[[false; 10];10]};
        return obj;
    }
    //set to specific value
    pub fn set(&mut self,c:Point,v:bool)
    {
        self.grid[c.xAsUsize()][c.yAsUsize()] = v;
    }
    //get specific value
    pub fn get(&self,c:Point) -> bool
    {
        return self.grid[c.xAsUsize()][c.yAsUsize()];
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    //returns false if unused. true if used or invalid square.
    pub fn isUsed(&self,c:Point) -> bool
    {
        if self.invalidC(c)
        {
            return true;
        }
        //println!("({},{})", c.getX(),c.getY());
        //println!("({},{})", c.xAsUsize(),c.yAsUsize());
        return self.grid[c.xAsUsize()][c.yAsUsize()];
    }
    #[allow(non_snake_case)]
    //returns true if the corrdinate is invalid
    pub fn invalidC(&self,c:Point) -> bool
    {
        return !(0<=c.getX()&&10>c.getX()&&0<=c.getY()&&10>c.getY());
    }
    pub fn land(&mut self,c:Point)
    {
        self.grid[c.xAsUsize()][c.yAsUsize()] = true;
    }
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    //return true if it is a valid move. return false if invalid.
    pub fn attempt(&mut self,c:Point) -> bool
    {
        let state:bool = ! self.isUsed(c);
        if state
        {
            //tile not used
            self.land(c);
            return true; //it worked
        }
        return false; //it did not work, the tile was allready used.
    }
}
