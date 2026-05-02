use std::sync::Arc;

use anyhow::Result;
use librqbit::{Api, Session, SessionOptions, SessionPersistenceConfig};

use crate::paths;

pub async fn build_api() -> Result<Arc<Api>> {
    let downloads = paths::default_downloads_dir()?;
    let data = paths::data_dir()?;

    let opts = SessionOptions {
        fastresume: true,
        persistence: Some(SessionPersistenceConfig::Json {
            folder: Some(data.join("session")),
        }),
        enable_upnp_port_forwarding: true,
        ..Default::default()
    };

    let session = Session::new_with_opts(downloads, opts).await?;
    Ok(Arc::new(Api::new(session, None)))
}
