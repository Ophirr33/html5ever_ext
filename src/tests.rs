// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


use super::*;


#[test]
fn smoke()
{
	let mut rc_dom = RcDom::from_file_path_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type("src/tests.html").expect("invalid HTML");
	
	eprintln!("{}", rc_dom.debug_string());
	
	
	let selector = parse_css_selector("main").unwrap();
	assert!(!rc_dom.matches(&selector));
	
	rc_dom.find_all_matching_child_nodes_depth_first_including_this_one(&selector, &mut |node|
	{
		eprintln!("{}", node.children.debug_string());
		
		const SHORTCUT: bool = false;
		SHORTCUT
	});
	
	let mut first_style_node = None;
	rc_dom.find_all_matching_child_nodes_depth_first_including_this_one(&parse_css_selector("head > style[amp]").unwrap(), &mut |node|
	{
		first_style_node = Some(node.clone());
		true
	});
	if let Some(ref first_style_node) = first_style_node
	{
		first_style_node.append_text(&mut rc_dom, "some-custom-css");
	}
	
	eprintln!("{}", rc_dom.debug_string());
}
