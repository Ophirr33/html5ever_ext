// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// Additional methods to work with QualName
pub trait QualNameExt
{
	/// Is this name effectively local?
	#[inline(always)]
	fn is_unprefixed_and_html_namespace_or_none(&self) -> bool;
	
	/// Is this qualified name this local-only name (no prefix, no namespace)
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool;
	
	/// Is this qualified name on these local-only names (no prefix, no namespace)
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool;
	
	/// Can this element have children?
	#[inline(always)]
	fn can_have_children(&self) -> bool;
	
	/// Should an immediate child text node have `<`, `>` and `&` characters escaped?
	/// In modern HTML 5, the only *common* nodes which don't need this are `<script>` and `<style>`.
	/// In this case, the immediate child text node's content should not contain, say, `</script>` as this will cause a parse error.
	#[inline(always)]
	fn text_content_should_be_escaped(&self) -> bool;
	
	/// Can this element's descendant text nodes have leading, trailing and interstitial whitespace collapsed?
	#[inline(always)]
	fn can_collapse_whitespace(&self) -> bool;
}

impl QualNameExt for QualName
{
	#[inline(always)]
	fn is_unprefixed_and_html_namespace_or_none(&self) -> bool
	{
		self.prefix.is_none() && match self.ns
		{
			ns!() | ns!(html) => true,
			_ => false,
		}
	}
	
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool
	{
		if self.is_unprefixed_and_html_namespace_or_none()
		{
			self.local == *local_name
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool
	{
		if self.is_unprefixed_and_html_namespace_or_none()
		{
			for local_name in local_names.iter()
			{
				if self.local == *local_name
				{
					return true;
				}
			}
			false
		}
		else
		{
			false
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn can_have_children(&self) -> bool
	{
		if !self.is_unprefixed_and_html_namespace_or_none()
		{
			return false;
		}
		
		match self.local
		{
			local_name!("area") | local_name!("base") | local_name!("basefont") | local_name!("bgsound") | local_name!("br") | local_name!("col") | local_name!("embed") | local_name!("frame") | local_name!("hr") | local_name!("img") | local_name!("input") | local_name!("keygen") | local_name!("link") | local_name!("meta") | local_name!("param") | local_name!("source") | local_name!("track") | local_name!("wbr") => false,
			
			_ => true,
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn text_content_should_be_escaped(&self) -> bool
	{
		if !self.is_unprefixed_and_html_namespace_or_none()
		{
			return false;
		}
		
		match self.local
		{
			local_name!("style") | local_name!("script") | local_name!("xmp") | local_name!("iframe") | local_name!("noembed") | local_name!("noframes") | local_name!("noscript") | local_name!("plaintext") => false,
			
			_ => true,
		}
	}
	
	#[inline(always)]
	fn can_collapse_whitespace(&self) -> bool
	{
		if !self.is_unprefixed_and_html_namespace_or_none()
		{
			return false;
		}
		
		match self.local
		{
			local_name!("pre") | local_name!("code") | local_name!("samp") | local_name!("kbd") => false,
			
			_ => true,
		}
	}
}

impl QualNameExt for RcDom
{
	#[inline(always)]
	fn is_unprefixed_and_html_namespace_or_none(&self) -> bool
	{
		self.document.is_unprefixed_and_html_namespace_or_none()
	}
	
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool
	{
		self.document.is_only_local(local_name)
	}
	
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool
	{
		self.document.is_only_local_of(local_names)
	}
	
	#[inline(always)]
	fn can_have_children(&self) -> bool
	{
		self.document.can_have_children()
	}
	
	#[inline(always)]
	fn text_content_should_be_escaped(&self) -> bool
	{
		self.document.text_content_should_be_escaped()
	}
	
	#[inline(always)]
	fn can_collapse_whitespace(&self) -> bool
	{
		self.document.can_collapse_whitespace()
	}
}

impl QualNameExt for Rc<Node>
{
	#[inline(always)]
	fn is_unprefixed_and_html_namespace_or_none(&self) -> bool
	{
		match self.data
		{
			NodeData::Element { ref name, .. } => name.is_unprefixed_and_html_namespace_or_none(),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool
	{
		match self.data
		{
			NodeData::Element { ref name, .. } => name.is_only_local(local_name),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool
	{
		match self.data
		{
			NodeData::Element { ref name, .. } => name.is_only_local_of(local_names),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn can_have_children(&self) -> bool
	{
		match self.data
		{
			NodeData::Document => true,
			
			NodeData::Element { ref name, .. } => name.can_have_children(),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn text_content_should_be_escaped(&self) -> bool
	{
		match self.data
		{
			NodeData::Element { ref name, .. } => name.text_content_should_be_escaped(),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn can_collapse_whitespace(&self) -> bool
	{
		match self.data
		{
			NodeData::Element { ref name, .. } => name.can_collapse_whitespace(),
			
			_ => false,
		}
	}
}
