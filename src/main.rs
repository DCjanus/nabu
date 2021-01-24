#[macro_use]
extern crate actix_web;

use crate::util::AnyResult;

mod conf;
mod log;
mod util;
mod view;

#[actix_web::main]
async fn main() -> AnyResult {
    crate::conf::init()?;
    crate::log::init()?;

    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(crate::view::ping)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await?;
    Ok(())
}

// https://api.bilibili.com/x/space/arc/search?mid=54992199&ps=30&tid=0&pn=1&keyword=&order=pubdate&jsonp=jsonp
