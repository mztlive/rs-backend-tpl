use axum::extract::{Extension, Path, Query, State};
use log::info;

use crate::{
    app_state::AppState,
    core::{
        errors::Result,
        response::ApiResponse,
        schema::{Account, UserID},
    },
};

use super::types::{GetMessagesRequest, InternalMessageResponse};
use services::InternalMessageService;

pub async fn get_my_messages(
    State(state): State<AppState>,
    Query(query): Query<GetMessagesRequest>,
    user_id: Extension<UserID>,
) -> Result<Vec<InternalMessageResponse>> {
    let messages = InternalMessageService::new(state.db_state.db)
        .get_my_messages(user_id.0.into(), query.page, query.page_size, query.status)
        .await?;

    ApiResponse::ok_with_data(messages.into_iter().map(|m| m.into()).collect())
}

pub async fn mark_message_as_read(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user_id: Extension<UserID>,
) -> Result<()> {
    InternalMessageService::new(state.db_state.db)
        .mark_as_read(id, user_id.0.into())
        .await?;

    ApiResponse::<()>::ok()
}
