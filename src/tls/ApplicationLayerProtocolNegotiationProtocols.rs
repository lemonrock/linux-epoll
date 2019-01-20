// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


/// Application layer protocols to negotiate in preference order.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplicationLayerProtocolNegotiationProtocols(pub IndexSet<ApplicationLayerProtocolNegotiationProtocol>);

impl Deref for ApplicationLayerProtocolNegotiationProtocols
{
	type Target = IndexSet<ApplicationLayerProtocolNegotiationProtocol>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for ApplicationLayerProtocolNegotiationProtocols
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl ApplicationLayerProtocolNegotiationProtocols
{
	/// To the form required by rustls.
	#[inline(always)]
	pub fn to_rustls_form(&self) -> Vec<String>
	{
		let mut protocols = Vec::with_capacity(self.application_layer_protocol_negotiation_protocols.len());
		for application_layer_protocol_negotiation_protocol in self.application_layer_protocol_negotiation_protocols
		{
			assert_ne!(application_layer_protocol_negotiation_protocol, ApplicationLayerProtocolNegotiationProtocol::HTTP_2_over_TCP, "HTTP_2_over_TCP can not be used with TLS");
			protocols.push(application_layer_protocol_negotiation_protocol.to_string())
		}
	}
}
