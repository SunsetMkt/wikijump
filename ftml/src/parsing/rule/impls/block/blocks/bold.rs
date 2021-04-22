/*
 * parsing/rule/impls/block/blocks/bold.rs
 *
 * ftml - Library to parse Wikidot text
 * Copyright (C) 2019-2021 Wikijump Team
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

pub const BLOCK_BOLD: BlockRule = BlockRule {
    name: "block-bold",
    accepts_names: &["b", "bold", "strong"],
    accepts_special: false,
    accepts_modifier: false,
    accepts_newlines: false,
    parse_fn,
};

fn parse_fn<'r, 't>(
    log: &Logger,
    parser: &mut Parser<'r, 't>,
    name: &'t str,
    special: bool,
    modifier: bool,
    in_head: bool,
) -> ParseResult<'r, 't, Elements<'t>> {
    debug!(
        log,
        "Parsing bold block";
        "in-head" => in_head,
        "name" => name,
    );

    assert_eq!(special, false, "Bold doesn't allow special variant");
    assert_eq!(modifier, false, "Bold doesn't allow modifier variant");
    assert_block_name(&BLOCK_BOLD, name);

    let arguments = parser.get_head_map(&BLOCK_BOLD, in_head)?;

    // Get body content, without paragraphs
    let (elements, exceptions, paragraph_safe) =
        parser.get_body_elements(&BLOCK_BOLD, false)?.into();

    let element = Element::Container(Container::new(
        ContainerType::Bold,
        elements,
        arguments.to_hash_map(),
    ));

    ok!(paragraph_safe; element, exceptions)
}
