//! Server-Sent Events endpoint streaming task-runtime broadcast events.

use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures_util::{stream, Stream};

use crate::WebState;

pub(crate) async fn task_events(
    State(state): State<Arc<WebState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let receiver = state.core.subscribe();
    let stream = stream::unfold(receiver, |mut receiver| async move {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    let frame = match serde_json::to_string(&event) {
                        Ok(json) => Event::default().data(json),
                        Err(_) => continue,
                    };
                    return Some((Ok(frame), receiver));
                }
                // slow consumer skipped some events; the next event still
                // carries a full snapshot, so just continue
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => return None,
            }
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}
