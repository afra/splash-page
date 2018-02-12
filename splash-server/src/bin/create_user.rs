extern crate splash_server as afra;

#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {

    #[structopt(short = "n", long = "user")]
    username: String,

    #[structopt(short = "p", long = "passwd")]
    password: String,
}

fn main() {
    let opt = Opt::from_args();
    
    let db = afra::establish_connection();
    afra::create_user(&db, &opt.username, &opt.password);
}