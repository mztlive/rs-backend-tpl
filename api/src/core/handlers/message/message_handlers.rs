use axum::{
    extract::{Path, Query, State},
    Json,
};
use database::repositories::{InternalMessageRepository, MessageRepository};
use log::{error, info};

use crate::{
    app_state::AppState,
    core::{errors::Result, response::ApiResponse},
};

use super::types::{MessageQueryRequest, MessageResponse, SendMessageRequest};
use services::MessageService;

pub async fn send_message(State(state): State<AppState>, Json(req): Json<SendMessageRequest>) -> Result<()> {
    info!("Sending message to {}: {}", req.recipient, req.subject);

    let message_repo = MessageRepository::new(state.db_state.db.clone());
    let internal_message_repo = InternalMessageRepository::new(state.db_state.db.clone());

    let params = req.into_params()?;
    MessageService::new(message_repo, internal_message_repo)
        .send_message(params)
        .await?;

    ApiResponse::<()>::ok()
}

pub async fn get_message_list(
    State(state): State<AppState>,
    Query(query): Query<MessageQueryRequest>,
) -> Result<Vec<MessageResponse>> {
    let query = query.into_query()?;
    let message_repo = MessageRepository::new(state.db_state.db.clone());
    let internal_message_repo = InternalMessageRepository::new(state.db_state.db.clone());
    let messages = MessageService::new(message_repo, internal_message_repo)
        .get_message_list(query)
        .await?;

    ApiResponse::ok_with_data(
        messages
            .into_iter()
            .map(|m| MessageResponse {
                id: m.base.id,
                channel: format!("{:?}", m.channel),
                recipient: m.recipient,
                subject: m.subject,
                content: m.content,
                status: format!("{:?}", m.status),
                error: m.error,
                created_at: m.base.created_at,
            })
            .collect(),
    )
}

pub async fn retry_message(State(state): State<AppState>, Path(id): Path<String>) -> Result<()> {
    let message_repo = MessageRepository::new(state.db_state.db.clone());
    let internal_message_repo = InternalMessageRepository::new(state.db_state.db.clone());

    MessageService::new(message_repo, internal_message_repo)
        .retry_by_id(&id)
        .await?;

    ApiResponse::<()>::ok()
}
