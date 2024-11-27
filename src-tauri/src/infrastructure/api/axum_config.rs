use std::sync::Arc;

use axum;
use tokio::sync::Mutex;

use crate::infrastructure::database::store_file::file_store::FileStore;
use crate::presentation::routers::axum_route::AxumRouter;

#[derive(Debug, Clone)]
pub struct AxumInit {
    pub router: AxumRouter,
}

impl AxumInit {
    pub fn new(filestore: Arc<Mutex<FileStore>>) -> Arc<Self> {
        Arc::new(AxumInit {
            router: AxumRouter::new(Arc::clone(&filestore)),
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
