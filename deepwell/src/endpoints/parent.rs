/*
 * endpoints/parent.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2024 Wikijump Team
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
use crate::models::page_parent::Model as PageParentModel;
use crate::services::page::GetPageReference;
use crate::services::parent::{
    GetParentRelationships, ModifyParentOutput, ParentDescription,
    ParentModifyDescription, RemoveParentOutput,
};
use crate::web::Reference;

pub async fn parent_relationships_get(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<Vec<PageParentModel>> {
    let GetParentRelationships {
        site_id,
        page: reference,
        relationship_type,
    } = params.parse()?;

    info!(
        "Getting all {} pages from {:?} in site ID {}",
        relationship_type.name(),
        reference,
        site_id,
    );

    ParentService::get_relationships(ctx, site_id, reference, relationship_type).await
}

pub async fn parent_get(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<Option<PageParentModel>> {
    let input: ParentDescription = params.parse()?;

    info!(
        "Getting parental relationship {:?} -> {:?} in site ID {}",
        input.parent, input.child, input.site_id,
    );

    ParentService::get_optional(ctx, input).await
}

pub async fn parent_set(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<Option<PageParentModel>> {
    let input: ParentDescription = params.parse()?;

    info!(
        "Creating parental relationship {:?} -> {:?} in site ID {}",
        input.parent, input.child, input.site_id,
    );

    ParentService::create(ctx, input).await
}

pub async fn parent_remove(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<RemoveParentOutput> {
    let input: ParentDescription = params.parse()?;

    info!(
        "Removing parental relationship {:?} -> {:?} in site ID {}",
        input.parent, input.child, input.site_id,
    );

    ParentService::remove(ctx, input).await
}

pub async fn parent_get_all(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<Vec<String>> {
    let GetPageReference { site_id, page } = params.parse()?;

    info!(
        "Getting parents for child {:?} in site ID {}",
        page, site_id,
    );

    let parents: Vec<Reference<'_>> = ParentService::get_parents(ctx, site_id, page)
        .await?
        .iter()
        .map(|p| Reference::from(p.parent_page_id))
        .collect();

    let pages: Vec<String> = PageService::get_pages(ctx, site_id, parents.as_slice())
        .await?
        .iter()
        .map(|p| p.slug.clone())
        .collect();

    Ok(pages)
}

pub async fn parent_modify(
    ctx: &ServiceContext<'_>,
    params: Params<'static>,
) -> Result<ModifyParentOutput> {
    let input: ParentModifyDescription = params.parse()?;

    info!(
        "Modifying multiple parental relationship for child {:?} in site ID {}",
        input.child, input.site_id,
    );

    let creation = match input.added {
        Some(parents) => {
            let mut creation = Vec::new();
            for parent in parents {
                if let Ok(Some(model)) = ParentService::create(
                    ctx,
                    ParentDescription {
                        site_id: input.site_id,
                        parent: parent.clone(),
                        child: input.child.clone(),
                    },
                )
                .await
                {
                    creation.push(model.parent_page_id);
                };
            }
            Some(creation)
        }
        None => None,
    };

    let removal = match input.removed {
        Some(parents) => {
            let mut removal = Vec::new();
            for parent in parents {
                if let Ok(res) = ParentService::remove(
                    ctx,
                    ParentDescription {
                        site_id: input.site_id,
                        parent: parent.to_owned(),
                        child: input.child.clone(),
                    },
                )
                .await
                {
                    removal.push(res.was_deleted);
                };
            }
            Some(removal)
        }
        None => None,
    };

    Ok(ModifyParentOutput {
        added: creation,
        removed: removal,
    })
}
