use tokio::sync::mpsc;

use crate::RoomSocket;

mod db;
mod oauth;
mod api;

pub async fn start_actix_server(room_update_sender: mpsc::Sender<RoomSocket>) {
    println!("Starting actix server...");

    let db = db::Database::new();

    let client = oauth::setup_oauth_client();

    api::start_actix_server(db, client).await;
}
