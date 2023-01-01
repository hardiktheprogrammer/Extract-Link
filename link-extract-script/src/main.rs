use error_chain::error_chain;
use select::document::Document; //document selector on pages
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error); //give a error name
        IoError(std::io::Error);



    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str()) //
        .find(Name("a")) // a tags
        .filter_map(|n| n.attr("href")) //href is link
        .for_each(|x| println!("{}", x)); //print the href link

    Ok(())
}
