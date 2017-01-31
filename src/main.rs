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

    runMultithreadTile(c);
    holdAwake();

}

#[allow(non_snake_case)]
fn configTest() -> Config
{
    let mut config = Config::new();
    config.setSize(2);
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
fn runMultithreadTile(config:Config)
{
    let mut threads = vec![];
    for x in 0..config.size
    {
        for y in 0..config.size
        {
            threads.push(spawnThreadTile(Point::new(x as i32,y as i32),config.clone()));
        }
    }
}

#[allow(non_snake_case)]
fn spawnThreadTile(p:Point,config:Config) -> JoinHandle<()>
{
    let start = p.clone();
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
    rec(start.clone(),start.clone(),moves,1,g,config);
    return true;
}

#[allow(non_snake_case)]
fn rec(start:Point,c:Point,path:String,depth:u8,table:Grid,config:Config) -> bool
{
    if depth==(config.size*config.size)
    {
        println!("reached Depth:{} \n using path:{} \n starting at:{}",depth, path,start.toString());
    }
    let mut newC = c.clone();
    let mut newT = table.clone();
    for e in 0..config.movements.len()
    {
        newC.moveToVector(config.movements[e as usize]);
        if newT.attempt(newC)
        {
            rec(start,newC,format!("{}{}",path,config.movements[e as usize].id),depth+1,newT,config.clone());
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
