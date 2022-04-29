
use anyhow::{Context, Result, Ok, Error};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use futures::{StreamExt, Future};

pub async fn block<T>(future: T) -> Result<(), Error>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static, 
{

    let (s, r) = tokio::sync::oneshot::channel::<()>();
    let start_task = tokio::spawn(async {
        future.await;
        s.send(()).unwrap();
    });

    let mut signals = Signals::new(&[
        SIGTERM,
        SIGINT,
        SIGQUIT,
    ]).context("Signals::new error")?;
    let handle = signals.handle();

    let signals_task = tokio::spawn(async move {
        if let Some(_) = signals.next().await {
            log::warn!("Got interrupt, shutting down...");
        }
    });

    tokio::select! {
        _ = r => {},
        _ = signals_task => {},
    };

    handle.close();
    start_task.abort();

    Ok(())
}
