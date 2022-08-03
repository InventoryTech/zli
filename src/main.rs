use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;

mod cli;

const FILEPREFIX: &str = "fileb://";

fn main() -> Result<(), Box<dyn Error>> {
    let app = cli::build();

    let matcher = app.get_matches();

    match matcher.subcommand_name() {
        Some("talk") => {
            let (_name, inner_matcher) = matcher.subcommand().unwrap();

            let action = inner_matcher.value_of("ACTION").unwrap();
            let port = inner_matcher.value_of("PORT").unwrap();
            let host = inner_matcher.value_of("HOST").unwrap();
            let topic = inner_matcher.value_of("TOPIC");
            let mut text: String = inner_matcher.value_of("TEXT").unwrap().to_string();

            let context = zmq::Context::new();

            let requester = match action {
                "push" => {
                    let ctx = context.socket(zmq::PUSH).unwrap();
                    ctx.connect(&format!("tcp://{}:{}", host, port)).unwrap();
                    ctx
                }
                "req" => {
                    let ctx = context.socket(zmq::REQ).unwrap();
                    ctx.connect(&format!("tcp://{}:{}", host, port)).unwrap();
                    ctx
                }
                "pub" => {
                    let ctx = context.socket(zmq::PUB).unwrap();
                    ctx.bind(&format!("tcp://{}:{}", host, port))
                        .expect("could not bind publisher socket");
                    ctx
                }
                &_ => context.socket(zmq::REQ).unwrap(),
            };

            if text.starts_with(FILEPREFIX) {
                match fs::read_to_string(text.trim_start_matches(FILEPREFIX)) {
                    Result::Ok(file) => match file.parse::<String>() {
                        Result::Ok(filecontents) => {
                            text = filecontents;
                        }
                        Result::Err(err) => return Err(Box::new(err)),
                    },
                    Result::Err(err) => return Err(Box::new(err)),
                }
            }

            match action {
                "push" => {
                    requester.send(&text, 0).unwrap();
                }
                "pub" => {
                    thread::sleep(Duration::from_millis(250));
                    match topic {
                        Some(t) => {
                            requester.send(t.as_bytes(), zmq::SNDMORE).unwrap();
                        }
                        None => {
                            requester.send("", zmq::SNDMORE).unwrap();
                        }
                    }

                    requester.send(&text, 0).unwrap();
                }
                "req" => {
                    requester.send(&text, 0).unwrap();
                    match requester.recv_string(0).expect("expecting a response") {
                        Result::Ok(data) => {
                            println!("{:?}", data)
                        }
                        Result::Err(_) => {
                            println!("{:?}", "-");
                        }
                    }
                }
                &_ => unreachable!(),
            };
        }
        Some("sink") => {
            let (_name, inner_matcher) = matcher.subcommand().unwrap();

            let action = inner_matcher.value_of("ACTION").unwrap();
            let port = inner_matcher.value_of("PORT").unwrap();
            let host = inner_matcher.value_of("HOST").unwrap();
            let topic = inner_matcher.value_of("TOPIC");

            println!("Starting {} sink at tcp://{}:{}", action, host, port);
            let context = zmq::Context::new();

            let socket = match action {
                "sub" => {
                    let ctx = context.socket(zmq::SUB).unwrap();
                    match topic {
                        Some(t) => {
                            ctx.set_subscribe(t.as_bytes()).expect("failed subscribing");
                        }
                        None => {
                            ctx.set_subscribe(b"").expect("failed subscribing");
                        }
                    }
                    ctx.connect(&format!("tcp://{}:{}", host, port))
                        .expect("failed to subscribe");
                    ctx
                }
                "pull" => {
                    let ctx = context.socket(zmq::PULL).unwrap();
                    ctx.bind(&format!("tcp://{}:{}", host, port)).unwrap();
                    ctx
                }
                "rep" => {
                    let ctx = context.socket(zmq::REP).unwrap();
                    ctx.bind(&format!("tcp://{}:{}", host, port)).unwrap();
                    ctx
                }
                &_ => context.socket(zmq::PULL).unwrap(),
            };

            loop {
                match socket.recv_string(0).expect("expecting a msg") {
                    Result::Ok(data) => {
                        match action {
                            "sub" => {
                                println!("Topic: {:?}", data);
                                match socket.recv_string(0).expect("expecting a msg") {
                                    Result::Ok(data) => {
                                        println!("Content: {:?}", data);
                                    }
                                    Result::Err(_) => {
                                        return Err(Box::from("failed"));
                                    }
                                }
                            }
                            "pull" => {
                                println!("Content: {:?}", data);
                            }
                            "rep" => {
                                println!("Content: {:?}", data);
                                socket.send(&data, 0).unwrap();
                            }
                            &_ => {}
                        };
                    }
                    Result::Err(_) => {
                        return Err(Box::from("failed"));
                    }
                }
            }
        }
        None => {}
        _ => unreachable!(),
    }
    Ok(())
}
