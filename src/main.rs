use std::thread;
use std::time;
use std::thread::Thread;
use std::thread::JoinHandle;
use point::Point;
use std::io::Read;
extern crate num_cpus;
use grid::Grid;
use config::Config;
use config::Movement;
mod config;
mod point;
mod grid;
extern crate curl;
use std::io::{stdout, Write};
use curl::easy::Easy;


fn main() {
    /*
    let mut g = Grid::new();
    g.land(Point::new(0,0));
    rec(Point::new(0,0),Point::new(0,0),String::new(),0,g);
    startAtStage("",Point::new(0,0));*/
    //println!("{}",num_cpus::get());
    //startAtStage(String::from("22244460014460030344600144460003306414460030644500034235060054470250354216600250344600345270352"),Point::new(0,0));
    //runMultithreadAuto();
    //holdAwake();
    let c = configTest();
    print!("{}\n",c.print());

    runMultithreadAuto(c);
    holdAwake();

}

#[allow(non_snake_case)]
fn configTest() -> Config
{
    let mut config = Config::new();
    config.setSize(4);
    config.insertMovement(Movement::new('0',0,-1));
    config.insertMovement(Movement::new('1',0,1));
    config.insertMovement(Movement::new('2',1,0));
    config.insertMovement(Movement::new('3',1,-1));
    config.insertMovement(Movement::new('4',1,1));
    return config;
}

#[allow(non_snake_case)]
fn holdAwake()
{
    #[allow(while_true)]
    while true
    {
        let ten_second = time::Duration::from_millis(10000);
        thread::sleep(ten_second);
    }
}

#[allow(non_snake_case)]
fn runMultithreadAuto(config : Config)
{
    runMultithread(num_cpus::get() as u8, config);
}

#[allow(non_snake_case)]
fn runMultithread(num:u8, config:Config)
{
    let mut i=0;
    let mut threads = vec![];
    while i<num&&i<config.size
    {
        threads.push(spawnThread(i,config.clone()));
        i+=1;
    }
}

#[allow(non_snake_case)]
//passed 0 through 7 inclusive. representing the starting direction

fn spawnThread(start:u8,config:Config) -> JoinHandle<()>
{
    let handle = thread::spawn(move || {
       looper(start,config);
   });
   return handle;
}

fn looper(d:u8, start:Point, config:Config)
{
    //println!("looper: \n    d:{}\n  start:{}",d,start.toString());
    let mut x=start.getX();
    while x<10
    {
        let mut y=start.getY();

        while y<10
        {
            ///println!("d:{}", d);
            startDirection(d,Point::new(x,y),config.clone());
            y+=1;
        }
        x+=1;
    }
}

#[allow(non_snake_case)]
fn runMultithreadTile(num:u8, config:Config)
{
    let mut i=0;
    let mut threads = vec![];
    while i<num&&i<config.size
    {
        threads.push(spawnThreadTile(i,config.clone()));
        i+=1;
    }
}

#[allow(non_snake_case)]
fn spawnThreadTile(id:u8,config:Config) -> JoinHandle<()>
{
    let mut start = Point::new(0,0);
    start.setX(id%config.size);
    start.setY(config.size/id);
    println!("{:?}", );
    let handle = thread::spawn(move || {
       startAtTile(start,config);
   });
   return handle;
}

#[allow(non_snake_case)]
fn startAtTile(start:Point,config:Config) ->bool
{
    let mut g = Grid::new(config.clone());
    g.land(start);
    let moves = String::from("");
    rec(start.copy(),start.copy(),moves,1,g,config);
    return true;
}

#[allow(non_snake_case)]
fn rec(start:Point,c:Point,path:String,depth:u8,table:Grid,config:Config) -> bool
{
    //squelch for new path found. make sure squelch is only run when greater than 0 or undocumented bahavior may arise
    if depth>(config.size*config.size)-2
    {
        println!("reached Depth:{} \n using path:{} \n starting at:{}",depth, path,start.toString());
        return true;
    }
    let mut newC = c.clone();
    let mut newT = table.clone();
    for e in 0..config.movements.len()-1
    {
        newC.moveToVector(config.movements[e as usize]);
        if newT.attempt(newC)
        {
            rec(start,newC,format!("{}{}",path,e),depth+1,newT,config.clone());
        }
        newC = c.clone();
        newT = table.clone();
    }
    return false;
}


#[test]
fn it_works() {
    thread::spawn(|| {
       runMultithreadAuto();
   });
   let ten_second = time::Duration::from_millis(10000);
   thread::sleep(ten_second);
   println!("Passing");
}
