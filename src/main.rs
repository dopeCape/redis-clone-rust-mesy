
#![allow(dead_code)]
#![allow(unused_variables)]


use std::{
    net::{ TcpListener, TcpStream },
    io::Read,
    io::Write,
    collections::HashMap,
    time::{ SystemTime, UNIX_EPOCH },
};

use threds::ThreadPool;
mod threds;
mod executor;
use std::sync::{ Mutex, Arc };

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let store: Arc<Mutex<HashMap<String, (String, u64)>>> = Arc::new(
        Mutex::new(std::collections::HashMap::new())
    );
    let th = ThreadPool::new(5);
    for stream in listener.incoming() {
        let store = Arc::clone(&store);
        match stream {
            Ok(mut stream) => {
                th.execute(move || {
                    loop {
                        responder(&mut stream, store.clone());
                    }
                });
            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn simple_string_encoder(data: &String) -> String {
    if data.contains("sdfjajdfaj$") {
        return data.to_string();
    }
    let res = format!("+{}\r\n", data);
    println!("{}", res);
    res
}
fn the_fn_not_used() {
    println!("tejas komawar ");
    let var = "tejas";
}
fn responder(stream: &mut TcpStream, mut store: Arc<Mutex<HashMap<String, (String, u64)>>>) {
    let mut buf = [0; 128];
    let mut vec_of_commands: Vec<executor::Command> = Vec::new();
    let i = stream.read(&mut buf).expect("error encodoing to string");
    let stream_string = String::from_utf8(buf[..i].to_vec()).expect("asas");
    convert_to_vec_of_msg(stream_string, &mut vec_of_commands);

    println!("{:?}", vec_of_commands);
    for tup in vec_of_commands.iter() {
        if
            tup.command[0] == Some("PING".to_string()) ||
            tup.command[0] == Some("ping".to_string())
        {
            // println!("{:?}", stream);

            write!(stream, "{}", simple_string_encoder(&"PONG".to_string())).expect(
                "error writeing to stream"
            );
        } else if tup.ty == Some("print".to_string()) {
            for i in &tup.command {
                match i {
                    Some(x) => {
                        write!(stream, "{}", simple_string_encoder(&x)).expect("erooorrrr");
                    }
                    None => {
                        println!("some thing weird");
                    }
                }
            }
        } else if tup.ty == Some("set".to_string()) {
            write!(
                stream,
                "{}",
                get_set_cahcer(tup.ty.to_owned().unwrap(), &tup.command, &mut store)
            ).expect("erooorrrr");
        } else if tup.ty == Some("get".to_string()) {
            write!(
                stream,
                "{}",
                get_set_cahcer(tup.ty.to_owned().unwrap(), &tup.command, &mut store)
            ).expect("erooorrrr");
        }
    }
}

fn get_set_cahcer(
    method: String,
    commands: &Vec<Option<String>>,
    store: &mut Arc<Mutex<HashMap<String, (String, u64)>>>
) -> String {
    if method == "set" {
        if commands.len() == 3 {
            let key = &commands[0].to_owned().unwrap();
            let value = &commands[1].to_owned().unwrap();

            let res = store.lock().unwrap().insert(key.to_string(), (value.to_string(), 0));

            // println!("{:?}",store);
            if res == None {
                return simple_string_encoder(&"OK".to_string());
            } else {
                return simple_string_encoder(&res.unwrap().0.to_string());
            }
        } else {
            let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let x =
                since_the_epoch.as_secs() * 1000 +
                (since_the_epoch.subsec_nanos() as u64) / 1_000_000 +
                commands[3].to_owned().unwrap().parse::<u64>().unwrap();
            let val = commands[1].to_owned().unwrap();
            let res = store.lock().unwrap().insert(commands[0].to_owned().unwrap(), (val, x));
            println!("{:?}", store.lock().unwrap());

            return simple_string_encoder(&"OK".to_string());
        }
    } else {
        let key = &commands[0].to_owned().unwrap();
        let mut store_unlocked = store.lock().unwrap();

        let res = store_unlocked.get(key);
        let mut since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let x =
            since_the_epoch.as_secs() * 1000 + (since_the_epoch.subsec_nanos() as u64) / 1_000_000;
        println!("{:?} && {:?}", res, x);
        if res == None {
            return simple_string_encoder(&"nil".to_string());
        } else {
            if res.unwrap().1 == 0 {
                return simple_string_encoder(&res.unwrap().0.to_string());
            } else {
                if res.unwrap().1 > x {
                    return simple_string_encoder(&res.unwrap().0.to_string());
                } else {
                    let y = store_unlocked.remove(key);
                    println!("{:?}", y);
                    if y != None {
                        println!("{:?}", y);

                        return "$-1\r\n".to_string();
                    } else {
                        return "some shiity as eeroro".to_string();
                    }
                }
            }
        }
    }
}

fn convert_to_vec_of_msg(s: String, vec_of_commands: &mut Vec<executor::Command>) {
    let mut count = 0;

    let mut t: executor::Command = executor::Command { ty: None, command: Vec::new() };
    t.command.push(None);
    vec_of_commands.push(t);

    println!("{}", s);
    for i in s.lines() {
        let mut t: executor::Command = executor::Command { ty: None, command: Vec::new() };

        t.command.push(None);
        if i.contains("*") {
        } else if i.contains("$") {
        } else {
            if vec_of_commands[count].ty != None && vec_of_commands[count].command[0] != None {
            }

            if vec_of_commands[count].ty == None {
                if i.contains("PING") || i.contains("ping") {
                    vec_of_commands[count].ty = Some("print".to_string());

                    vec_of_commands[count].command.pop();
                    vec_of_commands[count].command.push(Some("PING".to_string()));

                    continue;
                }
                if i.contains("ECHO") || i.contains("echo") {
                    vec_of_commands[count].ty = Some("print".to_string());
                    continue;
                }
                if i.contains("GET") || i.contains("get") {
                    vec_of_commands[count].ty = Some("get".to_string());
                }
                if i.contains("SET") || i.contains("set") {
                    vec_of_commands[count].ty = Some("set".to_string());
                }
            }
            if
                vec_of_commands[count].ty != None &&
                Some(i.to_string()) != vec_of_commands[count].ty
            {
                vec_of_commands[count].command.pop();
                vec_of_commands[count].command.push(Some(i.to_string()));
                if Some("print".to_string()) == vec_of_commands[count].ty {
                    vec_of_commands.push(t);
                    count += 1;
                } else if
                    Some("get".to_string()) == vec_of_commands[count].ty ||
                    Some("set".to_string()) == vec_of_commands[count].ty
                {
                    vec_of_commands[count].command.push(None);
                }
            }
        }
    }
}
