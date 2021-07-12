use std::error::Error;
use std::fs;

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
            let mut text: String = inner_matcher.value_of("TEXT").unwrap().to_string();

            let context = zmq::Context::new();

            let requester = match action {
                "push" => context.socket(zmq::PUSH).unwrap(),
                "req" => context.socket(zmq::REQ).unwrap(),
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

            assert!(requester
                .connect(&format!("tcp://{}:{}", host, port))
                .is_ok());

            requester.send(&text, 0).unwrap();

            match action {
                "push" => {}
                "req" => match requester.recv_string(0).expect("expecting a response") {
                    Result::Ok(data) => {
                        println!("{:?}", data)
                    }
                    Result::Err(_) => {
                        println!("{:?}", "-");
                    }
                },
                &_ => unreachable!(),
            };
        }
        Some("sink") => {
            let (_name, inner_matcher) = matcher.subcommand().unwrap();

            let action = inner_matcher.value_of("ACTION").unwrap();
            let port = inner_matcher.value_of("PORT").unwrap();
            let host = inner_matcher.value_of("HOST").unwrap();

            println!("Starting {} sink at tcp://{}:{}", action, host, port);
            let context = zmq::Context::new();

            let socket = match action {
                "pull" => context.socket(zmq::PULL).unwrap(),
                "rep" => context.socket(zmq::REP).unwrap(),
                &_ => context.socket(zmq::PULL).unwrap(),
            };

            socket.bind(&format!("tcp://{}:{}", host, port)).unwrap();
            loop {
                match socket.recv_string(0).expect("expecting a msg") {
                    Result::Ok(data) => {
                        println!("{:?}", data);

                        match action {
                            "pull" => {}
                            "rep" => {
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
