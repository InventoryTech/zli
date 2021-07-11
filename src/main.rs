use std::error::Error;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let app = cli::build();

    let matcher = app.get_matches();

    match matcher.subcommand_name() {
        Some("talk") => {
            let (_name, inner_matcher) = matcher.subcommand().unwrap();

            let action = inner_matcher.value_of("ACTION").unwrap();
            let port = inner_matcher.value_of("PORT").unwrap();
            let host = inner_matcher.value_of("HOST").unwrap();
            let text = inner_matcher.value_of("TEXT").unwrap();

            let context = zmq::Context::new();

            let requester = match action {
                "push" => context.socket(zmq::PUSH).unwrap(),
                "req" => context.socket(zmq::REQ).unwrap(),
                &_ => context.socket(zmq::REQ).unwrap(),
            };

            assert!(requester
                .connect(&format!("tcp://{}:{}", host, port))
                .is_ok());

            requester.send(text, 0).unwrap();
        }
        Some("sink") => {
            let (_name, inner_matcher) = matcher.subcommand().unwrap();

            let port = inner_matcher.value_of("PORT").unwrap();
            let host = inner_matcher.value_of("HOST").unwrap();

            println!("Starting sink @ tcp://{}:{}", host, port);
            let ctx = zmq::Context::new();
            let socket = ctx.socket(zmq::PULL).unwrap();
            socket.bind(&format!("tcp://{}:{}", host, port)).unwrap();
            loop {
                match socket.recv_msg(0) {
                    Result::Ok(data) => {
                        let s = match std::str::from_utf8(&data) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };

                        println!("{:?}", s)
                    }
                    Result::Err(_) => {
                        println!("{:?}", "-");
                    }
                }
            }
        }
        None => {}
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
    Ok(())
}
