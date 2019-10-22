#![allow(non_snake_case)]
use std::thread;
use std::env;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;


struct CompleteTask{
    startNode: i16,
    paths: Vec<Vec<i16>>,
}

fn main(){
    let mut argI:i8 = 0;
    let mut _totalNodes:String = "0".to_string();
    let mut _maxThreads:String = "8".to_string();
    for argument in env::args() {
        if argI == 1 {
            _totalNodes = argument.to_string();
        } else if argI == 2 {
            _maxThreads = argument.to_string();
        }
        argI += 1;
    }
    println!("You have chosen to order numbers upto {}", _totalNodes);
    println!("You have chosen to hava a max number of threads of {}", _maxThreads);
    let totalNodes:i16 = _totalNodes.parse::<i16>().unwrap();
    let maxThreads:i8 = _maxThreads.parse::<i8>().unwrap();
    let now = Instant::now();

    let paths = getCore(totalNodes, maxThreads, now);

    let endTime = now.elapsed().as_secs();
    printResult(paths, totalNodes);
    println!("It took aproximately {} seconds to compute", endTime);
}

fn getCore(totalNodes:i16, maxThreads:i8, now:Instant) -> Vec<Vec<i16>>{
    let connections:Vec<[i16; 2]> = findConnections(totalNodes);
    let (sender, reciever) = sync_channel(maxThreads as usize);
    let mut paths:Vec<Vec<i16>> = vec![];

    println!("There are {} connections", connections.len());
    
    let mut i:i16 = 0;
    let mut openThreads = 0;
    loop{
        while (openThreads < maxThreads as usize) && (i < totalNodes){
            startCalcAsync(i, connections.clone(), totalNodes.clone(), sender.clone());
            openThreads += 1;
            i += 1;
        }

        let completedTask = reciever.recv().unwrap();
        paths.append(&mut completedTask.paths.clone());
        openThreads -= 1;
        
        println!("finnished task {} of {} at {} seconds", completedTask.startNode, totalNodes, now.elapsed().as_secs());

        if openThreads == 0 {
            break;
        }
    }
    return paths;
}

fn startCalcAsync(startNode:i16, connections:Vec<[i16; 2]>, totalNodes:i16, sender:SyncSender<CompleteTask>){
    println!("Starting task {}", startNode);

    let mut path:Vec<i16> = Vec::with_capacity(totalNodes as usize);
    path.push(startNode);

    thread::spawn(move || {
        let paths = doThread(connections, &mut path.clone(), totalNodes);
        sender.send(CompleteTask{
            startNode:startNode,
            paths:paths,
        }).unwrap();
    });
}
fn printResult(paths:Vec<Vec<i16>>, totalNodes:i16){
    println!("Exporting final data...");
    let result = File::create("Output.txt");
    let mut file:File = match result {
        Ok(x) => x,
        Err(e) => {
            println!("Error on Output Attempt: {}", e);
            return;
        },
    };
    let mut completePaths:i32 = 0;
    for path in paths.clone(){
        if path.len() == totalNodes as usize {
            write!(file, "A working path is ");
            completePaths += 1;
            for node in path {
                write!(file, "{} ", node);
            }
            write!(file, "\n");
        }
    }
    
    println!("DONE!");
    println!("Completed with {} paths", paths.len());
    println!("{} of them are complete paths", completePaths);
    println!("All working paths have been outputted to Output.txt");
}

fn doThread(connections:Vec<[i16; 2]>, path:&mut Vec<i16>, totalNodes:i16) -> Vec<Vec<i16>>{
    let mut paths:Vec<Vec<i16>> = Vec::with_capacity(totalNodes as usize);
    
    loop{
        let node = path[path.len()-1];
        let mut goTo:i16 = 0;
        let mut isFirstConnection:bool = true;
        for connection in connections.clone() {
            let mut connectedTo:i16 = 0;
            if node == connection[0] {
                connectedTo = connection[1];
            }else if node == connection[1] {
                connectedTo = connection[0];
            }else{
                continue;
            }

            if !isNodeInPath(connectedTo, path.clone()) {
                if isFirstConnection {
                    isFirstConnection = false;
                    goTo = connectedTo;
                }else{
                    let mut newPath = path.clone();
                    newPath.push(connectedTo);
                    let newConnections = connections.clone();
                    paths.append(&mut doThread(newConnections, &mut newPath, totalNodes));
                }
            }
        }
        if isFirstConnection {
            break;
        } else {
            path.push(goTo);
        }
    }

    paths.push(path.clone());
    return paths;
}

fn isNodeInPath(node:i16, path:Vec<i16>)->bool{
    for nodeToCheck in path {
        if nodeToCheck == node {
            return true;
        }
    }
    return false;
}

fn findConnections(totalNodes:i16) -> Vec<[i16; 2]> {
    let mut connections:Vec<[i16; 2]> = vec![];
    for i in 1..totalNodes {
        for j in i+1..totalNodes+1 {
            if isSquare(i+j) {
                connections.push([i, j]);
            }
        }
    }
    return connections;
}

fn isSquare(num:i16) -> bool{
    let sqrt:f64 = (num as f64).sqrt();
    return sqrt == sqrt.round();
}