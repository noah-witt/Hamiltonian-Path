
pub struct Config
{
    pub size: u8,
    pub movements: Vec<Movement>

}

impl Clone for Config{
    fn clone(&self) -> Config
    {
        return Config {size:self.size,movements:self.movements.to_vec()};
    }
}
//impl Copy for Config{}

impl Config
{
    pub fn new() -> Config
    {
        return Config{size:0,movements:Vec::<Movement>::new()};
    }
    pub fn setSize(&mut self,s :u8)
    {
        self.size = s;
    }
    pub fn insertMovement(&mut self,m : Movement)
    {
        self.movements.push(m.clone());
    }
    pub fn print(&self) ->String
    {
        let r= format!("size:{} ",self.size);
        let mut collector = format!("");
        for m in &self.movements
        {
            collector = format!("{}\n{}",collector,m.print());
        }
        return format!("\n{}\n movement rules\n------------{}",r,collector);
    }
}

pub struct Movement
{
    pub id: char,
    pub X: i32,
    pub Y: i32
}

impl Clone for Movement{
    fn clone(&self) -> Movement
    {
        return Movement {id:self.id,X:self.X,Y:self.Y};
    }
}
impl Copy for Movement{}

impl Movement
{
    pub fn new(i:char,x:i32,y:i32) -> Movement
    {
        return Movement{id:i,X:x,Y:y};
    }
    pub fn print(&self) -> String
    {
        return format!("id:{}, moves:({},{})",self.id,self.X,self.Y);
    }
}
