use tokio::net::TcpListener;

const ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(ADDRESS).await?;
    println!("Ticket API listening on http://{ADDRESS}");
    axum::serve(listener, outro_08::router()).await
}
