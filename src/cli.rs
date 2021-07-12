use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub fn build() -> App<'static> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            App::new("talk")
                .arg(
                    Arg::new("ACTION")
                        .about("Choose action to take")
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
                .arg(Arg::new("TEXT").about("Set text").required(true).index(4)),
        )
        .subcommand(
            App::new("sink")
                .arg(
                    Arg::new("ACTION")
                        .about("Choose action to take")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("HOST")
                        .about("Sets the input file to use")
                        .required(true)
                        .index(2),
                )
                .arg(Arg::new("PORT").about("Set port").required(true).index(3)),
        );
    app
}
