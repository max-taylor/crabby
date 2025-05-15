use common::{types::position::Position, user::user_state::UserState};
use dioxus::{
    logger::tracing::info,
    prelude::spawn,
    signals::{Signal, Writable},
};
use futures::{stream::SplitSink, SinkExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_timers::future::TimeoutFuture;

pub fn spawn_sync_server_state(
    mut write: SplitSink<WebSocket, Message>,
    mut user_state_signal: Signal<Option<UserState>>,
    cursor_position: Signal<Position>,
) {
    spawn(async move {
        loop {
            TimeoutFuture::new(50).await;

            if user_state_signal().is_none() {
                continue;
            }

            let user_state = user_state_signal().unwrap();

            let direction = user_state.position.angle_to(&cursor_position());

            let mut updated_state = UserState {
                position: user_state.position,
                direction_deg: direction as u64,
            };
            updated_state.update(100);

            let user_state_json = serde_json::to_string(&updated_state);

            user_state_signal.set(Some(updated_state));

            if let Err(e) = user_state_json {
                info!("Error sending user state: {}", e);
                break;
            }

            let user_state_json = user_state_json.unwrap();
            let message = Message::Text(user_state_json.into());
            if let Err(e) = write.send(message).await {
                info!("Error sending message to server: {}", e);
                break;
            }
        }
    });
}
