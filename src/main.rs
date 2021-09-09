use imap::types::UnsolicitedResponse;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "idle")]
struct Opt {
    #[structopt(short, long)]
    server: String,

    #[structopt(short, long, default_value = "993")]
    port: u16,

    #[structopt(short, long)]
    username: String,

    #[structopt(long)]
    password_file_path: String,

    // The mailbox to IDLE on
    #[structopt(short, long, default_value = "INBOX")]
    mailbox: String,
}

fn main() {
    let opt = Opt::from_args();

    let client = imap::ClientBuilder::new(opt.server.clone(), opt.port)
        .native_tls()
        .expect("Could not connect to imap server");

    let mut imap = client
        .login(
            opt.username,
            fs::read_to_string(opt.password_file_path)
                .expect("Failed to retrive password from file")
                .trim(),
        )
        .expect("Could not authenticate");

    imap.examine(opt.mailbox)
        .expect("Could not examine mailbox");

    let unseen = imap
        .search("UNSEEN")
        .expect("Failed to fetch unseen messages")
        .len();
    println!("{}", unseen);

    imap.idle()
        .expect("Failed to monitor mailbox")
        .wait_while(|response| {
            match response {
                UnsolicitedResponse::Recent(how_many) => println!("{}", how_many),
                _ => {}
            }
            true
        })
        .expect("Failed to monitor inbox for changes");

    imap.logout().expect("Could not log out");
}
