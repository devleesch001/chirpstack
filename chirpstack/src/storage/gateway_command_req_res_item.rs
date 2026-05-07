use anyhow::Result;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::info;

use lrwn::EUI64;

use super::schema::gateway_command_req_res_item;
use super::{error::Error, fields, get_async_db_conn};

#[derive(Queryable, Insertable, PartialEq, Eq, Debug, Clone)]
#[diesel(table_name = gateway_command_req_res_item)]
pub struct GatewayCommandReqResItem {
    pub id: fields::Uuid,
    pub gateway_id: EUI64,
    pub created_at: DateTime<Utc>,
    pub exec_id: i32,
    pub command: String,
    pub stdin: Vec<u8>,
    pub environment: fields::KeyValue,
    pub response_at: Option<DateTime<Utc>>,
    pub stdout: Option<Vec<u8>>,
    pub stderr: Option<Vec<u8>>,
    pub error: Option<String>,
}

pub async fn create(gc: GatewayCommandReqResItem) -> Result<GatewayCommandReqResItem, Error> {
    let mut c = get_async_db_conn().await?;
    let gc: GatewayCommandReqResItem = diesel::insert_into(gateway_command_req_res_item::table)
        .values(&gc)
        .get_result(&mut c)
        .await
        .map_err(|e| Error::from_diesel(e, gc.id.to_string()))?;
    info!(id = %gc.id, "Gateway command created");
    Ok(gc)
}

pub async fn get(id: fields::Uuid) -> Result<GatewayCommandReqResItem, Error> {
    let mut c = get_async_db_conn().await?;
    let gc = gateway_command_req_res_item::dsl::gateway_command_req_res_item
        .find(&id)
        .first(&mut c)
        .await
        .map_err(|e| Error::from_diesel(e, id.to_string()))?;
    Ok(gc)
}

pub async fn update(gc: GatewayCommandReqResItem) -> Result<GatewayCommandReqResItem, Error> {
    let mut c = get_async_db_conn().await?;
    let gc: GatewayCommandReqResItem =
        diesel::update(gateway_command_req_res_item::dsl::gateway_command_req_res_item.find(&gc.id))
            .set((
                gateway_command_req_res_item::response_at.eq(&gc.response_at),
                gateway_command_req_res_item::stdout.eq(&gc.stdout),
                gateway_command_req_res_item::stderr.eq(&gc.stderr),
                gateway_command_req_res_item::error.eq(&gc.error),
            ))
            .get_result(&mut c)
            .await
            .map_err(|e| Error::from_diesel(e, gc.id.to_string()))?;
    info!(id = %gc.id, "Gateway command updated");
    Ok(gc)
}
