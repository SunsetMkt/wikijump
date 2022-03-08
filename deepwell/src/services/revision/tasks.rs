/*
 * services/revision/tasks.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2022 Wikijump Team
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
use crate::models::page_revision::Model as PageRevisionModel;
use crate::services::TextService;
use crate::web::ProvidedValue;

// TODO: Consolidate this.
//
//       Determine where our one source of truth for changes -> outdate calls
//       is, because right now some of these fields aren't used.
//
//       See the code branch "match old_slug" in services/revision/service.rs
//       We want a unified system where diff -> descendent changes, and
//       there's one place or structure or something to consult.

/// A representation of the updating tasks to do for a revision.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct RevisionTasks {
    pub render_and_update_links: bool,
    pub rerender_incoming_links: bool,
    pub rerender_outgoing_includes: bool,
    pub rerender_templates: bool,
}

impl RevisionTasks {
    pub fn determine(revision: &PageRevisionModel, changes: &CreateRevisionBody) -> Self {
        let mut tasks = RevisionTasks::default();

        if let ProvidedValue::Set(ref wikitext) = changes.wikitext {
            if revision.wikitext_hash.as_slice() != TextService::hash(wikitext).as_slice()
            {
                tasks.render_and_update_links = true;
                tasks.rerender_outgoing_includes = true;
                tasks.rerender_templates = true;
            }
        }

        // Don't need to check changes.hidden

        if let ProvidedValue::Set(ref title) = changes.title {
            if &revision.title != title {
                tasks.render_and_update_links = true;
                tasks.rerender_incoming_links = true;
            }
        }

        if let ProvidedValue::Set(ref alt_title) = changes.alt_title {
            if &revision.alt_title != alt_title {
                tasks.render_and_update_links = true;
                tasks.rerender_incoming_links = true;
            }
        }

        if let ProvidedValue::Set(ref slug) = changes.slug {
            if &revision.slug != slug {
                tasks.render_and_update_links = true;
                tasks.rerender_incoming_links = true;
                tasks.rerender_outgoing_includes = true;
                tasks.rerender_templates = true;
            }
        }

        if let ProvidedValue::Set(ref tags) = changes.tags {
            if !string_list_equals_json(&revision.tags, tags) {
                tasks.render_and_update_links = true;
                tasks.rerender_outgoing_includes = true;
                tasks.rerender_templates = true;
            }
        }

        tasks
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        !self.render_and_update_links
            && !self.rerender_incoming_links
            && !self.rerender_outgoing_includes
            && !self.rerender_templates
    }
}