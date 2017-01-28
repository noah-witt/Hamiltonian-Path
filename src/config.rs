#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate toml;


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

#[derive(Serialize, Deserialize, Debug)]
struct tomlConfig
{
    pub size: Option<i32>,
    pub vector: Option<Vec<tomlVector>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct tomlVector
{
    pub id:Option<char>,
    pub x: Option<i32>,
    pub y: Option<i32>,
}

impl Config
{
    pub fn new() -> Config
    {
        return Config{size:0,movements:Vec::<Movement>::new()};
    }
    pub fn newFromToml() -> Config
    {
        let mut c=Config::new();
        let toml = r#"
        size = 3
        [[vector]]
        id = '0'
        x =0
        y=-1

        [[vector]]
        id = '1'
        x =0
        y=1

        [[vector]]
        id = '2'
        x =1
        y=0

        [[vector]]
        id = '3'
        x =1
        y=-1

        [[vector]]
        id = '4'
        x =1
        y=1
        "#;
        let data:tomlConfig = toml::decode_str(toml).unwrap();
        println!("{}", data.size as i32);
        return c;
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
