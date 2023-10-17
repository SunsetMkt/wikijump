/*
 * endpoints/misc.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2023 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;
use crate::info;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use std::path::PathBuf;
use wikidot_normalize::normalize;

pub async fn ping(
    ctx: &ServiceContext<'_>,
    _params: Params<'static>,
) -> Result<&'static str> {
    tide::log::info!("Ping request");

    // Ensure the database is connected
    ctx.transaction()
        .execute(Statement::from_string(
            DatabaseBackend::Postgres,
            str!("SELECT 1"),
        ))
        .await?;

    // Seems good, respond to user
    Ok("Pong!")
}

pub async fn version(
    _ctx: &ServiceContext<'_>,
    _params: Params<'static>,
) -> Result<&'static str> {
    tide::log::info!("Getting DEEPWELL version");
    Ok(info::VERSION.as_str())
}

pub async fn full_version(
    _ctx: &ServiceContext<'_>,
    _params: Params<'static>,
) -> Result<&'static str> {
    tide::log::info!("Getting DEEPWELL version (full)");
    Ok(info::FULL_VERSION.as_str())
}

pub async fn hostname(
    _ctx: &ServiceContext<'_>,
    _params: Params<'static>,
) -> Result<&'static str> {
    tide::log::info!("Getting DEEPWELL hostname");
    Ok(info::HOSTNAME.as_str())
}

pub async fn config_dump(
    ctx: &ServiceContext<'_>,
    _params: Params<'static>,
) -> Result<String> {
    tide::log::info!("Dumping raw DEEPWELL configuration for debugging");
    Ok(ctx.config().raw_toml.to_string())
}

pub async fn config_path(
    ctx: &ServiceContext<'_>,
    _params: Params<'static>,
) -> Result<PathBuf> {
    tide::log::info!("Dumping DEEPWELL configuration path for debugging");
    Ok(ctx.config().raw_toml_path.to_path_buf())
}

pub async fn normalize_method(
    _ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<String> {
    let mut value: String = params.one()?;
    tide::log::info!("Running normalize on string: {value:?}");
    normalize(&mut value);
    Ok(value)
}
