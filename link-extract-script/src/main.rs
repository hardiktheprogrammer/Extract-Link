use error_chain::error_chain;
use select::document::Document; //document selector on pages
use select::predicate::Name;

error_chain! {
    foreign_chain!{
        ReqError(reqwest::Error); //give a error name
        IoError(std::io::Error);



    }
}
#[tokio::main]

async fn main() -> Result<()> {
    let res = reqwest 
} //asyc request
