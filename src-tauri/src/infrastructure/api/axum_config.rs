use std::sync::Arc;

use axum;
use tokio::sync::Mutex;

use crate::presentation::routers::axum_route::AxumRouter;
use crate::shared::stores::store_json::Stores;

#[derive(Debug, Clone)]
pub struct AxumInit {
    pub router: AxumRouter,
}

impl AxumInit {
    pub fn new(store: Arc<Mutex<Stores>>) -> Arc<Self> {
        Arc::new(AxumInit {
            router: AxumRouter::new(Arc::clone(&store)),
        })
    }

    pub async fn start(self: Arc<Self>) {
        let app = self.router.route();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!(
            "Listener start {}:{}",
            listener.local_addr().unwrap().ip(),
            listener.local_addr().unwrap().port()
        );
        axum::serve(listener, app).await.unwrap();
    }
}
