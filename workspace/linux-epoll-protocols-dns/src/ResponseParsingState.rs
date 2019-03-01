// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


#[derive(Debug)]
struct ResponseParsingState
{
	have_yet_to_see_an_answer_section_cname_resource_record: bool,
	have_yet_to_see_an_answer_section_dname_resource_record: bool,
	have_yet_to_see_a_soa_resource_record: bool,
	have_yet_to_see_an_edns_opt_resource_record: bool,
}