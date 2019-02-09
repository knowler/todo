use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Cli {
    #[structopt(name = "new")]
    New { task: String },

    #[structopt(name = "start")]
    Start { project: String },

    #[structopt(name = "stop")]
    Stop,

    #[structopt(name = "focus")]
    Focus { task: String },

    #[structopt(name = "unfocus")]
    Unfocus {
        #[structopt(long = "complete")]
        complete: bool,
    },
}
