use std::thread;
use std::time;
use std::thread::Thread;
use std::thread::JoinHandle;
use point::Point;
use std::io::Read;
extern crate num_cpus;
use grid::Grid;
mod point;
mod grid;
extern crate curl;
use std::io::{stdout, Write};
use curl::easy::Easy;

static netBuild:bool = false;

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
    netStarter();
    if netBuild
    {
        netStarter();
    }
    else
    {
        runMultithreadAuto();
        holdAwake();
    }

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
fn runMultithreadAuto()
{
    runMultithread(num_cpus::get() as u8);
}

#[allow(non_snake_case)]
fn runMultithread(num:u8)
{
    let mut i=0;
    let mut threads = vec![];
    while i<num&&i<8
    {
        threads.push(spawnThread(i,Point::new(0,0)));
        i+=1;
    }
}

#[allow(non_snake_case)]
//passed 0 through 7 inclusive. representing the starting direction

fn spawnThread(direction:u8,start:Point) -> JoinHandle<()>
{
    let mut d:String = String::from("0");
    match direction {
        0 => {
            d=String::from("0");
        },
        1 => {
            d=String::from("1");
        },
        2 => {
            d=String::from("2");
        },
        3 => {
            d=String::from("3");
        },
        4 => {
            d=String::from("4");
        },
        5 => {
            d=String::from("5");
        },
        6 => {
            d=String::from("6");
        },
        7 => {
            d=String::from("7");
        },
        _=> {
            panic!("thread");
        }
    }
    let df:String = d;
    //println!("spawnThread direction:{}",df);
    let handle = thread::spawn(move || {
       looper(df,start);
   });
   return handle;
}

#[allow(non_snake_case)]
fn spawnThreadNet() -> JoinHandle<()>
{
    let handle = thread::spawn(move || {
       netStarter();
   });
   return handle;
}

#[allow(non_snake_case)]
fn getAssignment() -> String
{
    //filler to test.
    //print!("ASSINGNMENT GET");
    //return String::from("0,0-41224450002444700502444603003050344600505033050522446050050333416560250531631746003541160147435277");
    makeRequest(String::from("page=get"));
    return String::from("0,0-4122445000244470050244460300305034460050503305052244605005033341656025053163");
}

#[allow(non_snake_case)]
fn reportSulution(start: Point,path:String){
    //filler to test
    //println!("SULUTION REPORT");
}

#[allow(non_snake_case)]
fn reportConclusion(){
    //filler to test
    //println!("CONCLUSION REPORT");
}

fn makeRequest(params:String) ->String
{
    return String::from("filler");
}

#[allow(non_snake_case)]
fn netStarter()
{
    let assignment: String = getAssignment();
    let assignmentVector: Vec<&str> = assignment.split("-").collect();
    if assignmentVector.len()!=2
    {
        panic!("INCORRECT assignmentVector");
    }
    let order = String::from(assignmentVector[1]);
    let coordinateVector: Vec<&str> = assignmentVector[0].split(",").collect();
    if coordinateVector.len()!=2
    {
        panic!("INCORRECT coordinateVector");
    }
    let startCoordinate = Point::new(coordinateVector[0].parse::<i32>().unwrap(),coordinateVector[1].parse::<i32>().unwrap());
    //println!("\n Start:{} \n order:{}",startCoordinate.toString(),order );
    startAtStage(order,startCoordinate);
    reportConclusion();
}

fn looper(d:String, start:Point)
{
    //println!("looper: \n    d:{}\n  start:{}",d,start.toString());
    let mut x=start.getX();
    while x<10
    {
        let mut y=start.getY();

        while y<10
        {
            ///println!("d:{}", d);
            startAtStage(d.clone(),Point::new(x,y));
            y+=1;
        }
        x+=1;
    }
}


#[allow(non_snake_case)]
fn startAtStage(moves:String,start:Point) ->bool
{
    //println!("MOVES STR:{}", moves);
    let length:usize = moves.chars().count();
    let length:u8 = length as u8;
    let mut p = start.clone();
    let mut g = Grid::new();
    g.land(start);
    let mut i:u8 =0;
    //println!("l:{}", length);
    while i<length
    {
        let ch:char = moves.chars().nth(i as usize).unwrap();
        //println!("v:{}", ch);
        let v: u8= ch.to_digit(10).unwrap() as u8;
        //println!("v:{}", v);
        p.moveToVector(v);
        let np:bool = !g.attempt(p);
        if np
        {
            //tile not used
            //println!("np");
            return false;
        }
        i+=1;
    }
    //println!("leng:{}", length);
    //done setupCoorrectly
    rec(start,p,moves,length,g);
    return true;
}

#[allow(non_snake_case)]
fn startAtStageNet(moves:String,start:Point) -> bool
{
    //println!("MOVES STR:{}", moves);
    let length:usize = moves.chars().count();
    let length:u8 = length as u8;
    let mut p = start.clone();
    let mut g = Grid::new();
    g.land(start);
    let mut i:u8 =0;
    //println!("l:{}", length);
    while i<length
    {
        let ch:char = moves.chars().nth(i as usize).unwrap();
        //println!("v:{}", ch);
        let v: u8= ch.to_digit(10).unwrap() as u8;
        //println!("v:{}", v);
        p.moveToVector(v);
        let np:bool = !g.attempt(p);
        if np
        {
            //tile not used
            //println!("np");
            return false;
        }
        i+=1;
    }
    //println!("leng:{}", length);
    //done setupCoorrectly
    rec(start,p,moves,length,g);
    return true;
}

#[allow(non_snake_case)]
fn rec(start:Point,c:Point,path:String,depth:u8,table:Grid)
{
    //squelch for new path found. make sure squelch is only run when greater than 0 or undocumented bahavior may arise
    if depth>98
    {
        println!("reached Depth:{} \n using path:{} \n starting at:{}",depth, path,start.toString());
        reportSulution(start,path.clone());
    }
    let mut newC = c.clone();
    let mut newT = table.clone();
    for e in 0..8
    {
        newC.moveToVector(e);
        if newT.attempt(newC)
        {
            rec(start,newC,format!("{}{}",path,e),depth+1,newT)
        }
        newC = c.clone();
        newT = table.clone();
    }
}
/*
struct Point {
    x: i32,
    y: i32
}*/


#[test]
fn it_works() {
    thread::spawn(|| {
       runMultithreadAuto();
   });
   let ten_second = time::Duration::from_millis(10000);
   thread::sleep(ten_second);
   println!("Passing");
}
