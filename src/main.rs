#[macro_use] extern crate rocket;
mod db;

#[launch]
async fn rocket() -> _ {

    #[get("/")]
    async fn index() -> String {
        "Welcome to the Mayday API, add to url to request data from database".to_string()
    }

    #[get("/tracks")]
    async fn tracks() -> String {
        let mut pscale = db::PScale { pool: db::new().await.unwrap(), };
        let tracks = pscale.get_all_tracks().await.unwrap();
        let string = tracks.join("\n");
        return string + "\n";
    }

    rocket::build()
        .mount("/", routes![index, tracks])
}

