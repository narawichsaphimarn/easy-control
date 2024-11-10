use std::sync::Arc;

use axum;
use log;

use crate::presentation::routers::axum_route::AxumRouter;
use crate::shared::stores::stores::Stores;

#[derive(Debug, Clone)]
pub struct AxumInit {
    pub router: AxumRouter,
}

impl AxumInit {
    pub fn new(stores: Arc<Stores>) -> Arc<Self> {
        Arc::new(AxumInit {
            router: AxumRouter::new(Arc::clone(&stores)),
        })
    }

    pub async fn start(self: Arc<Self>) {
        let app = self.router.route();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        log::debug!(
            "Listener start {}:{}",
            listener.local_addr().unwrap().ip(),
            listener.local_addr().unwrap().port()
        );
        axum::serve(listener, app).await.unwrap();
    }
}
