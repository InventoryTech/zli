use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};

pub fn build() -> App<'static> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("talk")
                .arg(
                    Arg::new("ACTION")
                        .about("Talk action to take (push, req or pub)")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("HOST")
                        .about("Sets the input file to use")
                        .required(true)
                        .index(2),
                )
                .arg(Arg::new("PORT").about("Set port").required(true).index(3))
                .arg(Arg::new("TEXT").about("Set text").required(true).index(4))
                .arg(
                    Arg::new("TOPIC")
                        .about("Optional topic for pub action")
                        .required(false)
                        .index(5),
                ),
        )
        .subcommand(
            App::new("sink")
                .arg(
                    Arg::new("ACTION")
                        .about("Sink action to take (pull, rep or sub")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("HOST")
                        .about("Sets the input file to use")
                        .required(true)
                        .index(2),
                )
                .arg(Arg::new("PORT").about("Set port").required(true).index(3))
                .arg(
                    Arg::new("TOPIC")
                        .about("Optional topic for sub action")
                        .required(false)
                        .index(4),
                ),
        );
    app
}
