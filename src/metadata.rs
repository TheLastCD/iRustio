use icy_metadata::IcyHeaders;
use std::error::Error;


pub async fn get_meta(url: &str)-> Result<(), Box<dyn Error>> {
    let stream = reqwest::get(url).await?;

    let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
    // println!("{icy_headers:?}");
    println!("Icecast headers: {icy_headers:#?}\n");
    // println!("content type={:?}\n", stream.content_type());



    Ok(())
}

// pub async fn get_meta(url: &str)-> Result<(), Box<dyn Error>> {
//     let stream = reqwest::get(url).await?;

//     let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
//     // println!("{icy_headers:?}");
//     println!("Icecast headers: {icy_headers:#?}\n");
//     // println!("content type={:?}\n", stream.content_type());



//     Ok(())
// }