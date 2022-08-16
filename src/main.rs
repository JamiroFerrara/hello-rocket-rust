#[macro_use] extern crate rocket;
mod db;

#[launch]
async fn rocket() -> _ {

    #[get("/")]
    async fn index() -> String {
        "Welcome to the Mayday API, add to url to request data from database".to_string()
    }

    #[get("/tracks")]
    async fn tracks(pscale: &rocket::State<db::PScale>) -> String {
        match pscale.get_all_tracks().await {
            Ok(tracks) => {
                let mut result = String::new();
                for track in tracks {
                    result.push_str(&format!("{}", track));
                }
                result
            }
            Err(e) => format!("{}", e),
        }
    }

    rocket::build()
        .manage(db::PScale { pool: db::new().await.unwrap() })
        .mount("/", routes![index, tracks])
}

