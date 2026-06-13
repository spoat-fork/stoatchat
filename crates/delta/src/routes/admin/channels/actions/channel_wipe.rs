use rocket::State;
use rocket_empty::EmptyResponse;
use revolt_result::{create_error, Result};
use revolt_database::{AdminAuthorization, Database};
use revolt_database::util::reference::Reference;
use revolt_models::v0;
use crate::routes::admin::util::{create_audit_action, flatten_authorized_user, user_has_permission};

/// Wipe all the messages from a channel
#[openapi(tag = "Admin")]
#[delete("/admin/channels/<channel_id>/wipe?<case>")]
pub async fn admin_channel_wipe(
    db: &State<Database>,
    auth: AdminAuthorization,
    channel_id: Reference<'_>,
    case: Option<&str>
) -> Result<EmptyResponse> {
    let admin = flatten_authorized_user(&auth);
    if !user_has_permission(admin, v0::AdminUserPermissionFlags::ManageChannels) {
        return Err(create_error!(MissingPermission {
            permission: "ManageChannels".to_string()
        }));
    }

    let channel = channel_id.as_channel(db).await?;

    channel.wipe(db).await?;

    create_audit_action(
        &db,
        &admin.id,
        v0::AdminAuditItemActions::WipeChannel,
        case,
        Some(channel_id.id),
        None,
    )
        .await?;

    Ok(EmptyResponse)
}