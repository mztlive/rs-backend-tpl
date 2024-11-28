use axum::extract::{Extension, Path, Query, State};

use crate::{
    app_state::AppState,
    core::{
        errors::Result,
        response::ApiResponse,
        schema::UserID,
    },
};

use super::types::{GetMessagesRequest, InternalMessageResponse};

pub async fn get_my_messages(
    State(state): State<AppState>,
    Query(query): Query<GetMessagesRequest>,
    user_id: Extension<UserID>,
) -> Result<Vec<InternalMessageResponse>> {
    let messages = state.services.internal_message_service()
        .get_my_messages(user_id.0.into(), query.page, query.page_size, query.status)
        .await?;

    ApiResponse::ok_with_data(messages.into_iter().map(|m| m.into()).collect())
}

pub async fn mark_message_as_read(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user_id: Extension<UserID>,
) -> Result<()> {
    state.services.internal_message_service()
        .mark_as_read(id, user_id.0.into())
        .await?;

    ApiResponse::<()>::ok()
}
