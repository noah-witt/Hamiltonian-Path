use point::Point;
use config::Config;
pub struct Grid
{
    grid: [[bool; 100]; 100],
    s: i32
}
impl Clone for Grid{
    fn clone(&self) -> Grid
    {
        let mut g = Grid::newl(self.s);
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
    pub fn new(c:Config) -> Grid
    {
        /*let mut obj:Grid = Grid {
            grid: {
                {false,false,false,false},
                {false,false,false,false},
                {false,false,false,false},
                {false,false,false,false}
            }
        };*/
        let mut obj:Grid = Grid {grid:[[false; 100];100],s:c.size as i32};
        return obj;
    }
    fn newl(sL:i32) -> Grid
    {
        /*let mut obj:Grid = Grid {
            grid: {
                {false,false,false,false},
                {false,false,false,false},
                {false,false,false,false},
                {false,false,false,false}
            }
        };*/
        let mut obj:Grid = Grid {grid:[[false; 100];100],s:sL};
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
        return !(0<=c.getX()&&self.s>c.getX()&&0<=c.getY()&&self.s>c.getY());
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
