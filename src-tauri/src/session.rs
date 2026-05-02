use std::sync::Arc;

use anyhow::Result;
use librqbit::limits::LimitsConfig;
use librqbit::{Api, Session, SessionOptions, SessionPersistenceConfig};

use crate::{paths, settings::AppSettings, settings};

pub async fn build_api(s: &AppSettings) -> Result<Arc<Api>> {
    let downloads = settings::resolve_download_dir(s)?;
    let data = paths::data_dir()?;

    let listen_port_range = if s.listen_port_min == 0 || s.listen_port_max == 0
        || s.listen_port_min >= s.listen_port_max
    {
        None
    } else {
        Some(s.listen_port_min..s.listen_port_max)
    };

    let opts = SessionOptions {
        fastresume: true,
        persistence: Some(SessionPersistenceConfig::Json {
            folder: Some(data.join("session")),
        }),
        enable_upnp_port_forwarding: s.enable_upnp,
        disable_dht: !s.enable_dht,
        listen_port_range,
        ratelimits: LimitsConfig {
            upload_bps: settings::kbps_to_nz_bps(s.upload_limit_kbps),
            download_bps: settings::kbps_to_nz_bps(s.download_limit_kbps),
        },
        ..Default::default()
    };

    let session = Session::new_with_opts(downloads, opts).await?;
    Ok(Arc::new(Api::new(session, None)))
}
