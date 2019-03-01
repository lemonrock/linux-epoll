// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


macro_rules! guard_hash_digest_if_final_field
{
	($resource_data: ident, $digest_offset: ident, $digest_size_in_bits: expr, $name: ident, $dns_protocol_error: ident) =>
	{
		{
			let digest_data = &$resource_data[$digest_offset .. ];

			let length = digest_data.len();

			const BitsInAByte: usize = 8;

			if unlikely!(length != $digest_size_in_bits / BitsInAByte)
			{
				return Err($dns_protocol_error(length))
			}

			$name(unsafe { & * (digest_data.as_ptr() as *const [u8; $digest_size_in_bits / BitsInAByte]) })
		}
	}
}

macro_rules! ipsec_like_public_key
{
	($public_key_algorithm_type: ident, $resource_data: ident, $public_key_starts_at_offset: ident, $public_key_length: ident, $resource_data_end_pointer: ident, $dsa_callback: block, $unassigned_callback: block) =>
	{
		{
			use self::PublicKey::*;

			match $public_key_algorithm_type
			{
				0 => if unlikely!($public_key_length != 0)
				{
					return Err(ResourceDataForTypeIPSECKEYOrHIPHasWrongLengthForNoPublicKey($public_key_length))
				}
				else
				{
					None
				},

				1 =>
				{
					$unassigned_callback;
					return Ok($resource_data_end_pointer)
				}

				2 =>
				{
					if unlikely!($public_key_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKey($public_key_length))
					}

					let public_key_data = &$resource_data[$public_key_starts_at_offset .. $public_key_starts_at_offset + $public_key_length];

					const FirstByteSize: usize = 1;

					let first_byte_of_exponent_length = public_key_data.u8(0);
					let (exponent_and_modulus, exponent_length) = if first_byte_of_exponent_length == 0
					{
						const SecondAndThirdBytesSize: usize = 2;

						const SizeSize: usize = FirstByteSize + SecondAndThirdBytesSize;

						if unlikely!(public_key_data.len() < SizeSize)
						{
							return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKeyForAThreeByteExponentLength(length))
						}

						(&public_key_data[SizeSize .. ], public_key_data.u16_as_usize(FirstByteSize))
					}
					else
					{
						(&public_key_data[FirstByteSize .. ], first_byte_of_exponent_length as usize)
					};

					if unlikely!(exponent_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAZeroExponentForARSAPublicKey)
					}

					if unlikely!(exponent_and_modulus.len() < exponent_length)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForARSAPublicKeyForExponentLength)
					}

					let modulus_length = exponent_and_modulus.len() - exponent_length;
					if unlikely!(modulus_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAZeroModulusForARSAPublicKey)
					}

					let rsa_public_key = RsaPublicKey
					{
						exponent: &exponent_and_modulus[ .. exponent_length],
						modulus: &exponent_and_modulus[exponent_length .. ],
					};

					Some(RSA(rsa_public_key))
				}

				3 =>
				{
					const BitsInAByte: usize = 8;

					if unlikely!($public_key_length != 256 / BitsInAByte || $public_key_length != 384 / BitsInAByte)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAUnusualLengthForAnECDSAPublicKey($public_key_length))
					}

					let public_key_data = &$resource_data[$public_key_starts_at_offset .. $public_key_starts_at_offset + $public_key_length];

					let ec_dsa_public_key = EcDsaPublicKey
					{
						Q: public_key_data,
					};

					Some(ECDSA(ec_dsa_public_key))
				}

				_ =>
				{
					$unassigned_callback;
					return Ok($resource_data_end_pointer)
				}
			}
		}
	}
}


#[derive(Debug)]
struct ResponseParsingState
{
	have_yet_to_see_a_soa_resource_record: bool,
	have_yet_to_see_an_edns_opt_resource_record: bool,
}

extern
{
	type ResourceRecord;
}

impl ResourceRecord
{
	const MinimumSize: usize = Name::MinimumSize + ResourceRecordFooter::MinimumSize;

	const BitsInAByte: usize = 8;

	const MinimumCharacterStringSize: usize = 1;

	const MinimumNameSize: usize = 1;

	/// Returns `Ok(end_of_resource_data_pointer)` unless there is an error.
	#[inline(always)]
	pub(crate) fn parse_answer_section_resource_record_in_response<'a>(&'a self, question_q_type: DataType, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, response_parsing_state: &mut ResponseParsingState) -> Result<usize, DnsProtocolError>
	{
		let (parsed_name_iterator, end_of_name_pointer, resource_record_type) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_labels)?;

		if likely!(question_q_type.is(resource_record_type))
		{
			()
		}
		else
		{
			match type_upper
			{
				0x00 => match type_lower
				{
					// TODO: x; should there only ever be one DNAME / CNAME in the answer section?

					/*
						DNS Flag Day
							https://dnsflagday.net/
								Starting February 1st, 2019 there will be no attempt to disable EDNS in reaction to a DNS query timeout.

								This effectively means that all DNS servers which do not respond at all to EDNS queries are going to be treated as dead.

								Please test your implementations using the ednscomp tool to make sure that you handle EDNS properly. Source code for the tool is available as well.

								It is important to note that EDNS is still not mandatory. If you decide not to support EDNS it is okay as long as your software replies according to EDNS standard section 7 (https://tools.ietf.org/html/rfc6891#section-7).

						DNSSEC
							- most top level domains are now signed: http://stats.research.icann.org/dns/tld_report/
							- some second-level domains suprisingly don't, eg microsoft.com

							https://metebalci.com/blog/a-minimum-complete-tutorial-of-dnssec/

							https://www.cloudflare.com/dns/dnssec/how-dnssec-works/
							https://www.cloudflare.com/dns/dnssec/dnssec-complexities-and-considerations/?utm_referrer=https://duckduckgo.com/
							https://blog.cloudflare.com/dnssec-done-right/?utm_referrer=https://duckduckgo.com/

							https://tools.ietf.org/html/rfc4033
							https://tools.ietf.org/html/rfc4034
							https://tools.ietf.org/html/rfc4035


						NAMESERVERS for TLDs
							eg https://www.iana.org/domains/root/db/com.html



						LOOKING UP NAMES
							found via MX, SRV, KX, NS, SOA and the like
							result MUST be a A / AAAA records; CNAME and DNAME are NOT ALLOWED


						FINDING THE AUTHORITATIVE SERVER
							So if we are to trust responses, we really need to find the authoritative server for a domain name.
								- trust resolver to lookup NS for example.com
								- trust resolver to lookup A for NS (no aliases allowed)
								- now open new TCP connection to As for NS, to do actual query look up.

							Slightly ?less secure?
								- trust resolver to lookup CNAME for blog.cloudflare.com
								- get CNAME back (OK, same domain) discard A answers; can keep the DNAME answer that created a synthetic CNAME but it's f--k all use to us, really.
								- return to client an alias result
								- client now does a second A or AAAA query, as desired

							Or we should just blindly trust our DNS resolver, and then use all of its records, including additional data.



						RFC2181
						Unauthenticated RRs received and cached from the least trustworthy of those groupings, that is data from the additional data section, and data from the authority section of a non-authoritative answer, should not be cached in such a way that they would ever be returned as answers to a received query. They may be returned as additional information where appropriate. Ignoring this would allow the trustworthiness of relatively untrustworthy data to be increased without cause or excuse.

						 However when the name sought is an alias (see section 10.1.1) only the record describing that alias is necessarily authoritative.
						 	ie the CNAME record is OK, the A records may not be authoritative.

						 Frankly, at the end of the day, do we care?

						 If we do care, we have to make sure EVERY record has a NAME for which the sending server is authoritative, ie IT MUST COME FROM THE NAMESERVER FOR THAT DOMAIN.

						 eg dig blog.cloudflare.com would require us to throw away all the `A` records for `cloudflare.ghost.io`

/*
https://tools.ietf.org/html/rfc6840#section-2

[RFC5155] describes the use and behavior of the NSEC3 and NSEC3PARAM
   records for hashed denial of existence.  Validator implementations
   are strongly encouraged to include support for NSEC3 because a number
   of highly visible zones use it.  Validators that do not support
   validation of responses using NSEC3 will be hampered in validating
   large portions of the DNS space.

resolvers MUST ignore the DO bit in responses even if they set in in requests due to broken implementations

*/




						RFC5452
						 Incoming responses should be verified to have a question section that
   is equivalent to that of the outgoing query. (ID, QNAME, QCLASS, QTYPE)

6.  Accepting Only In-Domain Records

   Responses from authoritative nameservers often contain information
   that is not part of the zone for which we deem it authoritative.  As
   an example, a query for the MX record of a domain might get as its
   responses a mail exchanger in another domain, and additionally the IP
   address of this mail exchanger.

   If accepted uncritically, the resolver stands the chance of accepting
   data from an untrusted source.  Care must be taken to only accept
   data if it is known that the originator is authoritative for the
   QNAME or a parent of the QNAME.

One very simple way to achieve this is to only accept data if it is
   part of the domain for which the query was intended.



   						WEIRD
   							dig +tries=1 +rrcomments +nofail +qr +multiline AAAA blog.cloudflare.com

   							- get a CNAME, no AAAA records, and a SOA in authority section for ***ghost.io*** NOT cloudflare.com; query was successful. Effectively a hint that there is no AAAA record (which there isn't).

   						NOTE ON SOA

							- SOA server isn't necessarily ANY of the NS servers..., eg dig SOA cloudflare.com gives a server name for which the IP address is NONE of the NS servers.
							- Interestingly, for cloudflare, using sara.ns.cloudflare.com as a nameserver (IP of 173.245.58.144)  with   dig @173.245.58.144 +tries=1 +rrcomments +nofail +qr +multiline NS cloudflare.com
								gives a result which includes a LOT of additional data.


						DNSSEC Trust Anchor(s)

							RFC 7958

							FILES https://www.iana.org/dnssec/files

							TOOL https://github.com/iana-org/get-trust-anchor

							Trust Anchor XML https://data.iana.org/root-anchors/root-anchors.xml
							<?xml version="1.0" encoding="UTF-8"?>
							<TrustAnchor id="380DC50D-484E-40D0-A3AE-68F2B18F61C7" source="http://data.iana.org/root-anchors/root-anchors.xml">
							<Zone>.</Zone>
							<KeyDigest id="Kjqmt7v" validFrom="2010-07-15T00:00:00+00:00" validUntil="2019-01-11T00:00:00+00:00">
							<KeyTag>19036</KeyTag>
							<Algorithm>8</Algorithm>
							<DigestType>2</DigestType>
							<Digest>49AAC11D7B6F6446702E54A1607371607A1A41855200FD2CE1CDDE32F24E8FB5</Digest>
							</KeyDigest>
							<KeyDigest id="Klajeyz" validFrom="2017-02-02T00:00:00+00:00">
							<KeyTag>20326</KeyTag>
							<Algorithm>8</Algorithm>
							<DigestType>2</DigestType>
							<Digest>E06D44B80B8F1D39A95C0B0D7C65D08458E880409BBC683457104237C7F8EC8D</Digest>
							</KeyDigest>
							</TrustAnchor>

							There are two `DS` records for the above:-
								zone: .

					*/

					// TODO: A, AAAA

					DataType::CNAME_lower | DataType::DNAME_lower => (),

					_ => return Err(ResourceRecordTypeIsNotValidInAnswerSection(resource_record_type))
				},

				_ => return Err(ResourceRecordTypeIsNotValidInAnswerSection(resource_record_type)),
			}
		}

		self.dispatch_resource_record_type(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, parsed_labels, resource_record_visitor, response_parsing_state, true, false, resource_record_type)
	}

	/// Returns `Ok(end_of_resource_data_pointer)` unless there is an error.
	#[inline(always)]
	pub(crate) fn parse_authority_section_resource_record_in_response<'a>(&'a self, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, response_parsing_state: &mut ResponseParsingState) -> Result<usize, DnsProtocolError>
	{
		let (parsed_name_iterator, end_of_name_pointer, resource_record_type) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_labels)?;

		x;
		// TODO: NSEC and RRSIG can also occur
		// eg dig +tries=1 +rrcomments +nofail +qr +multiline +dnssec A ns8.cloudflare.com
		// eg dig +dnssec DS microsoft.com

		if likely!(DataType::SOA.is(resource_record_type))
		{
			self.handle_soa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state)
		}
		else
		{
			Err(ResourceRecordTypeIsNotValidInAuthoritySection(resource_record_type))
		}
	}

	/// Returns `Ok(end_of_resource_data_pointer)` unless there is an error.
	#[inline(always)]
	pub(crate) fn parse_additional_data_section_resource_record_in_response<'a>(&'a self, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, response_parsing_state: &mut ResponseParsingState) -> Result<usize, DnsProtocolError>
	{
		let (parsed_name_iterator, end_of_name_pointer, resource_record_type) = self.validate_minimum_record_size_and_parse_name_and_resource_record_type(end_of_message_pointer, parsed_labels)?;

		self.dispatch_resource_record_type(end_of_name_pointer, end_of_message_pointer, parsed_name_iterator, parsed_labels, resource_record_visitor, response_parsing_state, false, true, resource_record_type)
	}

	/// Compression of names within `RDATA` is a mess.
	///
	/// RFC 3597, Section 4, Paragraph 2 restricts the records to which name (label) compression can be applied to be those defined in RFC 1035 which implicitly contain a name, hence:-
	///
	/// * `CNAME`
	/// * `MB`
	/// * `MD`
	/// * `MF`
	/// * `MG`
	/// * `MINFO`
	/// * `MR`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	///
	/// Of these, many are obsolete, leaving the list as:-
	///
	/// * `CNAME`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	///
	/// Additionally:-
	///
	/// * RFC 2163 permits compression to `PX` records;
	/// * RFC 2535 permits compression in `SIG` and `NXT` records;
	/// * RFC 3597 permits compression in `RP`, `AFSDB`, `RT` and `NAPTR` records;
	/// * RFC 3597 prohibits compression in `PX`, `SIG` and `NXT` records;
	/// * RFC 2782 prohibits compression in `SRV` records but the original RFC 2052 mandated it;
	/// * RFC 3597 prohibits compression for all future record types;
	/// * RFC 6672 prohibits compression for `DNAME`, but historically, there was confusion in the original RFC 2672 about whether it was permitted.
	///
	/// Of the records listed in the clause above, all are obsolete apart from `NAPTR`, `SRV` and `DNAME`.
	///
	/// Observations:-
	///
	/// * Given the history of `SRV`, it seems prudent to permit compression.
	/// * Given the similarity of `DNAME` to `CNAME`, and the historic confusion, it seems prudent to permit compression;
	///
	/// This gives a list of
	///
	/// * `CNAME`
	/// * `MX`
	/// * `NS`
	/// * `PTR`
	/// * `SOA`
	/// * `NAPTR`
	/// * `SRV`
	/// * `DNAME`
	#[inline(always)]
	fn dispatch_resource_record_type<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, parsed_labels: &mut ParsedLabels<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, response_parsing_state: &mut ResponseParsingState, soa_is_permitted: bool, opt_is_permitted: bool, (type_upper, type_lower): (u8, u8)) -> Result<usize, DnsProtocolError>
	{
		match type_upper
		{
			0x00 => match type_lower
			{
				DataType::SIG0_lower => self.handle_sig0(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::A_lower => self.handle_a(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NS_lower => self.handle_ns(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::MD_lower => self.handle_very_obsolete_record_type(DataType::MD),

				DataType::MF_lower => self.handle_very_obsolete_record_type(DataType::MF),

				DataType::CNAME_lower => self.handle_cname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::SOA_lower => if likely!(soa_is_permitted)
				{
					self.handle_soa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, response_parsing_state)
				}
				else
				{
					Err(StartOfAuthorityResourceRecordTypeIsNotPermittedInThisSection)
				},

				DataType::MB_lower => self.handle_very_obsolete_record_type(DataType::MB),

				DataType::MG_lower => self.handle_very_obsolete_record_type(DataType::MG),

				DataType::MR_lower => self.handle_very_obsolete_record_type(DataType::MR),

				DataType::NULL_lower => self.handle_very_obsolete_record_type(DataType::NULL),

				DataType::WKS_lower => self.handle_very_obsolete_record_type(DataType::WKS),

				DataType::PTR_lower => self.handle_ptr(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::HINFO_lower => self.handle_hinfo(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::MINFO_lower => self.handle_very_obsolete_record_type(DataType::MINFO),

				DataType::MX_lower => self.handle_mx(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::TXT_lower => self.handle_txt(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::RP_lower => self.handle_obsolete_or_very_obscure_record_type("Used in some rare circumstances; some legacy records may remain"),

				DataType::AFSDB_lower => self.handle_obsolete_or_very_obscure_record_type("Replaced by use of SRV records; some legacy records may remain"),

				DataType::X25_lower => self.handle_very_obsolete_record_type(DataType::X25),

				DataType::ISDN_lower => self.handle_very_obsolete_record_type(DataType::ISDN),

				DataType::RT_lower => self.handle_very_obsolete_record_type(DataType::RT),

				DataType::NSAP_lower => self.handle_very_obsolete_record_type(DataType::NSAP),

				DataType::NSAP_PTR_lower => self.handle_very_obsolete_record_type(DataType::NSAP_PTR),

				DataType::SIG_lower => self.handle_obsolete_or_very_obscure_record_type("Not used now SIG(0) is available; some legacy records may remain"),

				DataType::KEY_lower => self.handle_obsolete_or_very_obscure_record_type("Replaced by IPSECKEY and various DNSSEC records; some legacy records may remain"),

				DataType::PX_lower => self.handle_very_obsolete_record_type(DataType::PX),

				DataType::GPOS_lower => self.handle_very_obsolete_record_type(DataType::GPOS),

				DataType::AAAA_lower => self.handle_aaaa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::LOC_lower => self.handle_loc(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NXT_lower => self.handle_very_obsolete_record_type(DataType::NXT),

				DataType::EID_lower => self.handle_very_obsolete_record_type(DataType::EID),

				DataType::NIMLOC_lower => self.handle_very_obsolete_record_type(DataType::NIMLOC),

				DataType::SRV_lower => self.handle_srv(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::ATMA_lower => self.handle_very_obsolete_record_type(DataType::ATMA),

				DataType::NAPTR_lower => self.handle_naptr(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::KX_lower => self.handle_kx(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::CERT_lower => self.handle_cert(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::A6_lower => self.handle_very_obsolete_record_type(DataType::A6),

				DataType::DNAME_lower => self.handle_dname(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels),

				DataType::SINK_lower => self.handle_very_obsolete_record_type(DataType::SINK),

				MetaType::OPT_lower => if likely!(opt_is_permitted)
				{
					self.handle_opt(end_of_name_pointer, end_of_message_pointer, response_parsing_state)
				}
				else
				{
					Err(ExtendedDnsOptResourceRecordTypeIsNotPermittedInThisSection)
				},

				DataType::APL_lower => self.handle_obsolete_or_very_obscure_record_type("Some legacy records may remain"),

				DataType::DS_lower => self.handle_ds(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::SSHFP_lower => self.handle_sshfp(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::IPSECKEY_lower => self.handle_ipseckey(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NSEC_lower => self.handle_nsec(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::RRSIG_lower => self.handle_rrsig(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::DNSKEY_lower => self.handle_dnskey(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::DHCID_lower => self.handle_dhcid(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NSEC3_lower => self.handle_nsec3(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NSEC3PARAM_lower => XXXX,

				DataType::TLSA_lower => self.handle_tlsa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::SMIMEA_lower => self.handle_smimea(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				54 => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, 0x00, 54),

				DataType::HIP_lower => self.handle_hip(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::NINFO_lower => self.handle_obsolete_or_very_obscure_record_type("No RFC or RFC draft and probably not deployed"),

				DataType::RKEY_lower => self.handle_obsolete_or_very_obscure_record_type("No RFC or RFC draft and probably not deployed"),

				DataType::TALINK_lower => self.handle_obsolete_or_very_obscure_record_type("No RFC or RFC draft and probably not deployed"),

				DataType::CDS_lower => XXXX,

				DataType::CDNSKEY_lower => XXXX,

				DataType::OPENPGPKEY_lower => self.handle_openpgpkey(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::CSYNC_lower => XXXX,

				DataType::ZONEMD_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				64 ... 98 => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, 0x00, type_lower),

				DataType::SPF_lower => self.handle_obsolete_or_very_obscure_record_type("RFC 7208 deprecated this record type; some legacy records may remain"),

				DataType::UINFO_lower => self.handle_very_obsolete_record_type(DataType::UINFO),

				DataType::UID_lower => self.handle_very_obsolete_record_type(DataType::UID),

				DataType::GID_lower => self.handle_very_obsolete_record_type(DataType::GID),

				DataType::UNSPEC_lower => self.handle_very_obsolete_record_type(DataType::UNSPEC),

				DataType::NID_lower => self.handle_nid(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::L32_lower => self.handle_l32(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::L64_lower => self.handle_l64(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::LP_lower => self.handle_lp(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::EUI48_lower => self.handle_eui48(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::EUI64_lower => self.handle_eui64(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				110 ... 127 => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, 0x00, type_lower),

				128 ... 248 => Err(UnknownQueryTypeOrMetaType(0x00, type_lower)),

				MetaType::TKEY_lower => self.handle_obsolete_meta_type("Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				MetaType::TSIG_lower => self.handle_obsolete_meta_type("Only really useful for updates, which, frankly, are probably better done out-of-band than using DNS; regardless, when using DNS over TLS a client certificate is much more useful"),

				QueryType::IXFR_lower => Err(QueryTypeIXFRShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::AXFR_lower => Err(QueryTypeAXFRShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::MAILB_lower => Err(QueryTypeMAILBShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::MAILA_lower => Err(QueryTypeMAILAShouldNotOccurOutsideOfAQuestionSectionEntry),

				QueryType::Asterisk_lower => Err(QueryTypeAsteriskShouldNotOccurOutsideOfAQuestionSectionEntry),
			},

			0x01 => match type_lower
			{
				DataType::URI_lower => self.handle_uri(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::CAA_lower => self.handle_caa(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor),

				DataType::DOA_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

				DataType::AMTRELAY_lower => self.handle_unsupported(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),
				
				_ => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, 0x01, type_lower),
			},

			0x02 ... 0x7F => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, type_upper, type_lower),

			0x80 => match type_lower
			{
				DataType::TA_lower => self.handle_obsolete_or_very_obscure_record_type("DNSSEC Trust Anchors were never widely deployed; some legacy records may remain"),

				DataType::DLV_lower => self.handle_obsolete_or_very_obscure_record_type("DNSSEC Lookaside Validation is not longer supported now that all root nameservers support DNSSEC; some legacy records may remain"),

				_ => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, 0x80, type_lower),
			},

			0x81 ... 0xEF => self.handle_unassigned(end_of_name_pointer, end_of_message_pointer, resource_record_name, resource_record_visitor, parsed_labels, resource_record_type),

			_ => Err(ReservedRecordType(type_upper, type_lower))
		}
	}

	/// Record types that died a very long time ago.
	#[inline(always)]
	fn handle_very_obsolete_record_type<'a>(&'a self, data_type: DataType) -> Result<usize, DnsProtocolError>
	{
		Err(VeryObsoleteResourceRecordType(data_type))
	}

	/// Record types that died, never became popular or widespread or never proceeded even to a RFC draft.
	#[inline(always)]
	fn handle_obsolete_or_very_obscure_record_type<'a>(&'a self, _reason: &'static str) -> Result<usize, DnsProtocolError>
	{
		let (_time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		Ok(resource_data.end_pointer())
	}

	/// Meta types, that, with the coming of DNS over TLS, are obsolete.
	#[inline(always)]
	fn handle_obsolete_meta_type<'a>(&'a self, _reason: &'static str) -> Result<usize, DnsProtocolError>
	{
		let (_time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		Ok(resource_data.end_pointer())
	}

	/// Data types that are draft RFCs or similar and may need to be supported by clients of this library.
	#[inline(always)]
	fn handle_unsupported<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>, type_upper: u8, type_lower: u8) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		resource_record_visitor.unsupported(resource_record_name, time_to_live, resource_data, parsed_labels, DataType([type_upper, type_lower]))?;
		Ok(resource_data.end_pointer())
	}

	/// Data types that aren't officially registered with IANA.
	#[inline(always)]
	fn handle_unassigned<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>, type_upper: u8, type_lower: u8) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		resource_record_visitor.unassigned(resource_record_name, time_to_live, resource_data, parsed_labels, DataType([type_upper, type_lower]))?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_a<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_internet_protocol_address_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.A(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_ns<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels)?;
		resource_record_visitor.NS(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_cname<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels)?;
		resource_record_visitor.CNAME(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_soa<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>, response_parsing_state: &mut ResponseParsingState) -> Result<usize, DnsProtocolError>
	{
		if unlikely!(!response_parsing_state.have_yet_to_see_a_soa_resource_record)
		{
			return Err(MoreThanOneStatementOfAuthorityResourceRecord)
		}

		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let start_of_resource_data = resource_data.pointer();

		let (primary_name_server, end_of_mname_pointer) = parsed_labels.parse_name_in_slice(resource_data)?;
		let (responsible_person_email_address, end_of_rname_pointer) = parsed_labels.parse_name_in_slice(&resource_data[(end_of_mname_pointer - start_of_resource_data) .. ])?;

		let end_of_resource_data = start_of_resource_data + resource_data.len();
		if likely!((end_of_resource_data - end_of_rname_pointer) == size_of::<StartOfAuthorityFooter>())
		{
			let start_of_authority = StartOfAuthority
			{
				primary_name_server,
				responsible_person_email_address,
				footer: unsafe { & * (end_of_rname_pointer as *const StartOfAuthorityFooter) },
			};

			resource_record_visitor.SOA(resource_record_name, time_to_live, start_of_authority)?;

			response_parsing_state.have_yet_to_see_a_soa_resource_record = false;

			Ok(resource_data.end_pointer())
		}
		else
		{
			Err(StartOfAuthorityIsIncorrectSizeAfterParsingMNAMEAndRNAME)
		}
	}

	#[inline(always)]
	fn handle_ptr<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels)?;
		resource_record_visitor.PTR(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_hinfo<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let length = resource_data.len();

		const MinimumCpuSize: usize = Self::MinimumCharacterStringSize;
		const MinimumOsSize: usize = Self::MinimumCharacterStringSize;

		if unlikely!(length < MinimumCpuSize + MinimumOsSize)
		{
			return Err(ResourceDataForTypeHINFOHasTooShortALength(length))
		}

		let character_strings_iterator = CharacterStringsIterator::new(resource_data);

		let cpu = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveCpuDataOverflow(length))?;

		let os = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveOsDataOverflow(length))?;

		if likely!(character_strings_iterator.is_empty())
		{
			resource_record_visitor.HINFO(resource_record_name, time_to_live, record)?;
			Ok(resource_data.end_pointer())
		}
		else
		{
			Err(ResourceDataForTypeHINFOWouldHaveUnusuedDataRemaining)
		}
	}

	#[inline(always)]
	fn handle_mx<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const PreferenceSize: usize = 2;
		const MinimumMailServerNameSize: usize = Self::MinimumNameSize;

		let length = resource_data.len();

		if unlikely!(length < PreferenceSize + MinimumMailServerNameSize)
		{
			return Err(ResourceDataForTypeMXHasTooShortALength(length))
		}

		let record = MailExchange
		{
			preference: resource_data.u16(0),
			mail_server_name: parsed_labels.parse_name_in_slice_with_nothing_left(&resource_data[PreferenceSize .. ])?,
		};

		resource_record_visitor.MX(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_txt<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let text_strings_iterator = CharacterStringsIterator::new(resource_data)?;

		resource_record_visitor.TXT(resource_record_name, time_to_live, text_strings_iterator)?;

		if likely!(text_strings_iterator.is_empty())
		{
			Ok(resource_data.end_pointer())
		}
		else
		{
			Err(ResourceDataForTypeTXTWouldHaveUnusuedDataRemaining)
		}
	}

	#[inline(always)]
	fn handle_aaaa<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_internet_protocol_address_only(end_of_name_pointer, end_of_message_pointer)?;
		resource_record_visitor.AAAA(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_loc<'a>(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let length = resource_data.len();
		if unlikely!(length != size_of::<Location>())
		{
			return Err(ResourceDataForTypeLOCHasAnIncorrectLength(length))
		}

		let location = resource_data.cast::<Location>(0);

		let version = location.version()?;
		debug_assert_eq!(version, LocationVersion::Version0, "Why are we supporting a version other than 0 of LOC records?");

		resource_record_visitor.LOC(resource_record_name, time_to_live, location)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_srv<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const PrioritySize: usize = 2;
		const WeightSize: usize = 2;
		const PortSize: usize = 2;
		const MinimumTargetNameSize: usize = Self::MinimumNameSize;

		let length = resource_data.len();
		if unlikely!(length < PrioritySize + WeightSize + PortSize + MinimumTargetNameSize)
		{
			return Err(ResourceDataForTypeSRVHasAnIncorrectLength(length))
		}

		let record = Service
		{
			priority: resource_data.u16(0),
			weight: resource_data.u16(PrioritySize),
			port: resource_data.u16(PrioritySize + WeightSize),
			target: parsed_labels.parse_name_in_slice_with_nothing_left(&resource_data[(PrioritySize + WeightSize + PortSize) .. ])?,
		};

		resource_record_visitor.SRV(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_naptr<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const OrderSize: usize = 2;
		const PreferenceSize: usize = 2;
		const MinimumFlagsSize: usize = Self::MinimumCharacterStringSize;
		const MinimumServicesSize: usize = Self::MinimumCharacterStringSize;
		const MinimumRegularExpressionSize: usize = Self::MinimumCharacterStringSize;
		const MinimumDomainNameSize: usize = Self::MinimumNameSize;

		let length = resource_data.len();
		if unlikely!(length < OrderSize + PreferenceSize + MinimumFlagsSize + MinimumServicesSize + MinimumRegularExpressionSize + MinimumDomainNameSize)
		{
			return Err(ResourceDataForTypeNAPTRHasAnIncorrectLength(length))
		}

		let order = resource_data.u16(0);
		let preference = resource_data.u16(OrderSize);

		let character_strings_iterator = CharacterStringsIterator::new(&resource_data[(OrderSize + PreferenceSize) .. ]);

		let flags = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveCpuDataOverflow(length))?;

		let services = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveOsDataOverflow(length))?;

		let regular_expression = character_strings_iterator.next().ok_or(ResourceDataForTypeHINFOWouldHaveOsDataOverflow(length))?;

		let remaining_resource_data = character_strings_iterator.remaining_resource_data();
		let start_of_name_pointer = remaining_resource_data.pointer();
		let resource_data_end_pointer = start_of_name_pointer + remaining_resource_data.len();

		let header = NamingAuthorityPointerHeader
		{
			order,
			preference,
			flags,
			services,
		};

		if regular_expression.is_empty()
		{
			let (domain_name, end_of_name_pointer) = ParsedNameIterator::parse_without_compression(start_of_name_pointer, resource_data_end_pointer);
			if unlikely!(end_of_name_pointer != resource_data_end_pointer)
			{
				return Err(ResourceDataForTypeNAPTRHasDataLeftOver)
			}

			let record = NamingAuthorityPointerWithDomainName
			{
				header,
				domain_name
			};

			resource_record_visitor.NAPTR_domain_name(resource_record_name, time_to_live, record)?;
		}
		else
		{
			let end_of_name_pointer = start_of_name_pointer + 1;

			if unlikely!(end_of_name_pointer != end_of_resource_data)
			{
				return Err(ResourceDataForTypeNAPTRHasDataLeftOver)
			}

			let domain_name_byte = unsafe { * (start_of_name_pointer as *const u8) };
			if unlikely!(domain_name_byte != 0)
			{
				return Err(ResourceDataForTypeNAPTRHasBothARegularExpressionAndADomainName)
			}

			let record = NamingAuthorityPointerWithRegularExpression
			{
				header,
				regular_expression
			};

			resource_record_visitor.NAPTR_regular_expression(resource_record_name, time_to_live, record)?;
		}
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_kx<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let length = resource_data.len();

		const PreferenceSize: usize = 2;
		const MinimumKeyExchangeServerNameSize: usize = Self::MinimumNameSize;

		if unlikely!(length < PreferenceSize + MinimumKeyExchangeServerNameSize)
		{
			return Err(ResourceDataForTypeKXHasTooShortALength(length))
		}

		let record = KeyExchange
		{
			preference: resource_data.u16(0),
			mail_server_name: parsed_labels.parse_name_in_slice_with_nothing_left(&resource_data[PreferenceSize .. ])?,
		};

		resource_record_visitor.KX(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_cert<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::CertificateResourceRecordIgnoredBecauseReason::*;
		use self::CertificateType::*;

		const CertificateTypeSize: usize = 2;
		const KeyTagSize: usize = 2;
		const AlgorithmSize: usize = 1;
		const MinimumCertificateOrCertificateRevocationListSize: usize = 0;
		const CertificateDataOffset: usize = CertificateTypeSize + KeyTagSize + AlgorithmSize;

		let length = resource_data.len();

		if unlikely!(length < CertificateTypeSize + KeyTagSize + AlgorithmSize + MinimumCertificateOrCertificateRevocationListSize)
		{
			return Err(ResourceDataForTypeCERTHasTooShortALength(length))
		}

		let certificate_type_value_upper = resource_data.u8(0);
		let certificate_type_value_lower = resource_data.u8(1);
		let certificate_type = match certificate_type_value_upper
		{
			0x00 => match certificate_type_value_lower
			{
				0 => return Err(ResourceDataForTypeCERTUsesAReservedCertificateTypeValue(0)),

				1 => X509ASPerPkixCertificate(&resource_data[CertificateDataOffset .. ]),

				2 => SpkiCertificate(&resource_data[CertificateDataOffset .. ]),

				3 => OpenPgpPacket(&resource_data[CertificateDataOffset .. ]),

				4 => UrlOfAX509DataObject(&resource_data[CertificateDataOffset .. ]),

				5 => UrlOfASpkiCertificate(&resource_data[CertificateDataOffset .. ]),

				6 => FingerprintAndUrlOfAnOpenPgpPacket(&resource_data[CertificateDataOffset .. ]),

				7 => AttributeCertificate(&resource_data[CertificateDataOffset .. ]),

				8 => UrlOfAnAttributeCertificate(&resource_data[CertificateDataOffset .. ]),

				9 ... 252 =>
				{
					resource_record_visitor.CERT_ignored(resource_record_name, CertificateTypeUnassigned(certificate_type_value_lower as u16));
					return Ok(resource_data_end_pointer)
				}

				253 =>
				{
					resource_record_visitor.CERT_ignored(resource_record_name, CertificateTypeUriPrivate(certificate_type_value_lower as u16));
					return Ok(resource_data_end_pointer)
				}

				254 =>
				{
					resource_record_visitor.CERT_ignored(resource_record_name, CertificateTypeOidPrivate(certificate_type_value_lower as u16));
					return Ok(resource_data_end_pointer)
				}

				255 => return Err(ResourceDataForTypeCERTUsesAReservedCertificateTypeValue(255)),
			},

			0x01 ... 0xFE =>
			{
				resource_record_visitor.CERT_ignored(resource_record_name, CertificateTypeUnassigned((certificate_type_value_upper as u16) << 8 | (certificate_type_value_lower as u16)));
				return Ok(resource_data_end_pointer)
			}

			0xFF => if unlikely!(certificate_type_value_lower == 0xFF)
			{
				return Err(ResourceDataForTypeCERTUsesAReservedCertificateTypeValue(0xFF << 8 | (certificate_type_value_lower as u16)))
			}
			else
			{
				return Err(ResourceDataForTypeCERTUsesAnExperimentalCertificateTypeValue(0xFF << 8 | (certificate_type_value_lower as u16)))
			},
		};

		let security_algorithm_type = resource_data.u8(CertificateTypeSize + KeyTagSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, false, false)?
		{
			Left(security_algorithm) => security_algorithm,

			Right(security_algorithm_rejected_because_reason) =>
			{
				resource_record_visitor.CERT_ignored(resource_record_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = Certificate
		{
			key_tag: resource_data.value::<KeyTag>(CertificateTypeSize),
			security_algorithm,
			certificate_type,
		};

		resource_record_visitor.CERT(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_opt(&self, end_of_name_pointer: usize, end_of_message_pointer: usize, response_parsing_state: &mut ResponseParsingState) -> Result<usize, DnsProtocolError>
	{
		if unlikely!(!response_parsing_state.have_already_seen_an_edns_opt_resource_record)
		{
			return Err(MoreThanOneExtendedDnsOptResourceRecord)
		}

		let start_of_name_pointer = self.name() as *const Name as usize;
		if unlikely!(end_of_name_pointer - start_of_name_pointer != 1)
		{
			return Err(ExtendedDnsOptRecordNameTooLong)
		}

		let name_length_or_offset = unsafe { * (self.name() as *const Name as *const u8) };
		if unlikely!(name_length_or_offset != 0x00)
		{
			return Err(ExtendedDnsOptRecordNameNotRoot)
		}

		let requestors_udp_payload_size = self.requestors_udp_payload_size(end_of_name_pointer);
		if unlikely!(requestors_udp_payload_size < 512)
		{
			return Err(ExtendedDnsOptRecordUdpPayloadSizeIsLessThan512Bytes(512))
		}

		let extended_response_code_and_flags = self.extended_response_code_and_flags(end_of_name_pointer);

		let upper_8_bits = extended_response_code_and_flags.extended_response_code_upper_8_bits();
		// TODO: Any value that isn't zero is effectively an error.
		x;


		let version = extended_response_code_and_flags.version()?;
		debug_assert_eq!(version, ExtendedDnsVersion::Version0, "Why do we support EDNS versions other than 0?");

		let dnssec_ok = extended_response_code_and_flags.dnssec_ok();

		extended_response_code_and_flags.z()?;

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		// 2-byte option code, 2-byte option length
		let mut current_pointer = resource_data.pointer();
		let end_of_options_pointer = current_pointer + resource_data.len();
		while current_pointer != end_of_options_pointer
		{
			if unlikely!(current_pointer + 4 > end_of_options_pointer)
			{
				return Err(ExtendedDnsOptionTooShort)
			}

			let option_code = u16::from_be_bytes(unsafe { * (current_pointer as *const [u8; 2]) });

			#[inline(always)]
			fn validate_raw_option_data<'a>(current_pointer: usize) -> Result<&'a [u8], DnsProtocolError>
			{
				let option_length = u16::from_be_bytes(unsafe { * ((current_pointer + 2) as *const [u8; 2]) }) as usize;
				let option_data_pointer = current_pointer + 4;
				if unlikely!(option_data_pointer + option_length > end_of_options_pointer)
				{
					Err(ExtendedDnsOptionDataOverflows)
				}
				else
				{
					Ok(unsafe { from_raw_parts(option_data_pointer as *const u8, option_length) })
				}
			}

			const DAU: u16 = 5;
			const DHU: u16 = 6;
			const N3U: u16 = 7;

			match option_code
			{
				0 | 65001 ... 65535 => return Err(ExtendedDnsOptionCodeWasReserved(option_code)),

				// RFC 6975, Section 6, Paragraph 4: "Authoritative servers MUST NOT set the DAU, DHU, and/or N3U option(s) on any responses.
				// These values are only set in queries".
				DAU => return Err(ExtendedDnsOptionDAUMustOnlyBeSetInARequest),

				// RFC 6975, Section 6, Paragraph 4: "Authoritative servers MUST NOT set the DAU, DHU, and/or N3U option(s) on any responses.
				// These values are only set in queries".
				DHU => return Err(ExtendedDnsOptionDHUMustOnlyBeSetInARequest),

				// RFC 6975, Section 6, Paragraph 4: "Authoritative servers MUST NOT set the DAU, DHU, and/or N3U option(s) on any responses.
				// These values are only set in queries".
				N3U => return Err(ExtendedDnsOptionN3UMustOnlyBeSetInARequest),

				_ =>
				{
					let option_data = validate_raw_option_data(current_pointer)?;
				}
			}
		}

		response_parsing_state.have_yet_to_see_an_edns_opt_resource_record = false;
	}

	#[inline(always)]
	fn handle_dname<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor, parsed_labels: &mut ParsedLabels<'a>) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, record, end_of_resource_data_pointer) = self.parse_name_only(end_of_name_pointer, end_of_message_pointer, parsed_labels)?;
		resource_record_visitor.DNAME(resource_record_name, time_to_live, record)?;
		Ok(end_of_resource_data_pointer)
	}

	#[inline(always)]
	fn handle_ds<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::DelegationSignerResourceRecordIgnoredBecauseReason::*;
		use self::DnsSecDigest::*;
		use self::SecurityAlgorithmRejectedBecauseReason::*;

		const KeyTagSize: usize = 2;
		const SecurityAlgorithmTypeSize: usize = 1;
		const DigestTypeSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < KeyTagSize + SecurityAlgorithmTypeSize + DigestTypeSize + MinimumDigestSize)
		{
			return Err(ResourceDataForTypeDSHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let security_algorithm_type = resource_data.u8(KeyTagSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, false, false)?
		{
			Left(security_algorithm) => security_algorithm,
			Right(security_algorithm_rejected_because_reason) =>
			{
				resource_record_visitor.DS_ignored(resource_record_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
				return Ok(resource_data_end_pointer)
			}
		};

		const DigestOffset: usize = KeyTagSize + SecurityAlgorithmTypeSize + DigestTypeSize;

		let digest_type = resource_data.u8(3);
		let digest = match digest_type
		{
			0 => return Err(DigestAlgorithmTypeIsReservedByRfc3658),

			1 =>
			{
				resource_record_visitor.DS_ignored(resource_record_name, DigestAlgorithmRejected(Sha1IsBroken));
				return Ok(resource_data_end_pointer)
			}

			2 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 256, Sha2_256, ResourceDataForTypeDSHasADigestLengthThatIsIncorrectForTheDigestType),

			3 =>
			{
				resource_record_visitor.DS_ignored(resource_record_name, DigestAlgorithmRejected(Gost94MayBeBroken));
				return Ok(resource_data_end_pointer)
			}

			4 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 384, Sha2_384, ResourceDataForTypeDSHasADigestLengthThatIsIncorrectForTheDigestType),

			_ =>
			{
				resource_record_visitor.DS_ignored(resource_record_name, DigestAlgorithmRejected(Unassigned(digest_type)));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = DelegationSigner
		{
			key_tag: resource_data.value::<KeyTag>(0),
			security_algorithm,
			digest,
		};

		resource_record_visitor.DS(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_sshfp<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::SshFingerprintDigest::*;
		use self::SshFingerprintResourceRecordIgnoredBecauseReason::*;

		const PublicKeyAlgorithmSize: usize = 1;
		const DigestAlgorithmSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < PublicKeyAlgorithmSize + DigestAlgorithmSize + MinimumDigestSize)
		{
			return Err(ResourceDataForTypeSSHFPHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let raw_public_key_algorithm = resource_data.u8(0);
		let public_key_algorithm: SshPublicKeyAlgorithm = match raw_public_key_algorithm
		{
			0 => return Err(ResourceDataForTypeSSHFPHasAReservedPublicKeyAlgorithm),

			1 ... 4 => unsafe { transmute(raw_public_key_algorithm) },

			_ =>
			{
				resource_record_visitor.SSHFP_ignored(resource_record_name, PublicKeyAlgorithmUnassigned(raw_public_key_algorithm));
				return Ok(resource_data_end_pointer)
			}
		};

		const DigestOffset: usize = PublicKeyAlgorithmSize + DigestAlgorithmSize;

		let raw_digest_algorithm = resource_data.u8(1);
		let public_key_digest = match raw_digest_algorithm
		{
			0 => return Err(ResourceDataForTypeSSHFPHasAReservedDigestAlgorithm),

			1 =>
			{
				resource_record_visitor.SSHFP_ignored(resource_record_name, DigestAlgorithmSha1IsBroken);
				return Ok(resource_data_end_pointer)
			}

			2 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 256, Sha2_256, ResourceDataForTypeSSHFPAHasADigestLengthThatIsIncorrectForTheMatchingType),

			_ =>
			{
				resource_record_visitor.SSHFP_ignored(resource_record_name, DigestAlgorithmUnassigned(raw_digest_algorithm));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = PublicKeyFingerprint
		{
			public_key_algorithm,
			public_key_digest,
		};

		resource_record_visitor.SSHFP(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_ipseckey<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::Gateway::*;
		use self::IpsecKeyResourceRecordIgnoredBecauseReason::*;

		const PrecedenceSize: usize = 1;
		const GatewayTypeSize: usize = 1;
		const PublicKeyAlgorithmSize: usize = 1;
		const MinimumGatewaySize: usize = 0;
		const MinimumPublicKeySize: usize = 0;

		const GatewayFieldStartsAtOffset: usize = PrecedenceSize + GatewayTypeSize + PublicKeyAlgorithmSize;

		let length = resource_data.len();

		if unlikely!(length < PrecedenceSize + GatewayTypeSize + PublicKeyAlgorithmSize + MinimumGatewaySize + MinimumPublicKeySize)
		{
			return Err(ResourceDataForTypeIPSECKEYHasTooShortALength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let gateway_type = resource_data.u8(PrecedenceSize);
		let (public_key_starts_at_offset, gateway) = match gateway_type
		{
			0 => (GatewayFieldStartsAtOffset, None),

			1 =>
			{
				if unlikely!(length < GatewayFieldStartsAtOffset + size_of::<IpAddrV4>())
				{
					return Err(ResourceDataForTypeIPSECKEYHasTooShortALengthForAnInternetProtocolVersion4Gateway(length))
				}
				let gateway = resource_data.cast::<IpAddrV4>(GatewayFieldStartsAtOffset);

				(GatewayFieldStartsAtOffset + size_of::<IpAddrV4>(), Some(InternetProtocolVersion4(gateway)))
			}

			2 =>
			{
				if unlikely!(length < GatewayFieldStartsAtOffset + size_of::<IpAddrV6>())
				{
					return Err(ResourceDataForTypeIPSECKEYHasTooShortALengthForAnInternetProtocolVersion6Gateway(length))
				}
				let gateway = resource_data.cast::<IpAddrV6>(GatewayFieldStartsAtOffset);

				(GatewayFieldStartsAtOffset + size_of::<IpAddrV6>(), Some(InternetProtocolVersion6(gateway)))
			}

			3 =>
			{
				if unlikely!(length < GatewayFieldStartsAtOffset + 1)
				{
					return Err(ResourceDataForTypeIPSECKEYHasTooShortALengthForDomainNameGateway(length))
				}

				let resource_data_starts_at_pointer = resource_data.pointer();
				let start_of_name_pointer = resource_data_starts_at_pointer + GatewayFieldStartsAtOffset;
				let (domain_name, end_of_domain_name_pointer) = ParsedNameIterator::parse_without_compression(start_of_name_pointer, start_of_name_pointer + length - GatewayFieldStartsAtOffset)?;

				(end_of_domain_name_pointer - resource_data_starts_at_pointer, Some(DomainName(domain_name)))
			}

			_ =>
			{
				resource_record_visitor.IPSECKEY_ignored(GatewayTypeUnassigned(gateway_type));
				return Ok(resource_data_end_pointer)
			}
		};

		let public_key_algorithm_type = resource_data.u8(PrecedenceSize + GatewayTypeSize);
		let public_key_length = length - public_key_starts_at_offset;
		let public_key = ipsec_like_public_key!(public_key_algorithm_type, resource_data, public_key_starts_at_offset, public_key_length, resource_data_end_pointer, { resource_record_visitor.IPSECKEY_ignored(PublicKeyAlgorithmDSAIsProbablyBroken) }, { resource_record_visitor.IPSECKEY_ignored(PublicKeyAlgorithmUnassigned(public_key_algorithm_type)) })?;

		let record = IpsecPublicKey
		{
			precedence: resource_data.u8(0),
			gateway,
			public_key,
		};

		resource_record_visitor.IPSECKEY(name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_nsec<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const MinimumNextSecureNameSize: usize = Self::MinimumNameSize;

		let length = resource_data.len();
		if unlikely!(length < MinimumNextSecureNameSize + TypeBitmaps::MinimumTypeBitmapsSize)
		{
			return Err(ResourceDataForTypeNSECHasAnIncorrectLength(length))
		}

		let resource_data_pointer = resource_data.pointer();
		let resource_data_end_pointer = resource_data.end_pointer();

		let (next_domain_name, end_of_next_domain_name_pointer) = ParsedNameIterator::parse_without_compression(resource_data_pointer, resource_data_end_pointer)?;

		let record = NextSecure
		{
			next_domain_name,
			type_bitmaps: TypeBitmaps::parse_type_bitmaps(&resource_data[(end_of_next_domain_name_pointer - resource_data_pointer) .. ])?,
		};

		resource_record_visitor.NSEC(name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_rrsig<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::DnsKeyPurpose::*;
		use self::ResourceRecordSetSignatureResourceRecordIgnoredBecauseReason::*;

		const TypeCoveredSize: usize = 2;
		const AlgorithmSize: usize = 1;
		const LabelsSize: usize = 1;
		const OriginalTimeToLiveSize: usize = 4;
		const SignatureExpirationSize: usize = 4;
		const SignatureInceptionSize: usize = 4;
		const KeyTagSize: usize = 2;
		const MinimumSignersNameSize: usize = Self::MinimumNameSize;
		const MinimumSignatureSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize + MinimumSignersNameSize + MinimumSignatureSize)
		{
			return Err(ResourceDataForTypeRRSIGHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let labels = resource_data.u8(TypeCoveredSize + AlgorithmSize);
		if unlikely!(labels > 126)
		{
			return Err(ResourceDataForTypeRRSIGHasMoreThan126Labels(labels))
		}

		let security_algorithm_type = resource_data.u8(TypeCoveredSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, false, false)?
		{
			Left(security_algorithm) => security_algorithm,
			Right(security_algorithm_rejected_because_reason) =>
			{
				resource_record_visitor.RRSIG_ignored(resource_record_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
				return Ok(resource_data_end_pointer)
			}
		};

		let signature_expiration_timestamp = resource_data.value::<SignatureTimestamp>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize);
		let signature_inception_timestamp = resource_data.value::<SignatureTimestamp>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize);
		match signature_expiration.difference(signature_inception)
		{
			None =>
			{
				resource_record_visitor.RRSIG_ignored(resource_record_name, DifferenceInSignatureExpirationAndInceptionIsTooGreatForWrappingSerialNumberMathematics { signature_inception_timestamp, signature_expiration_timestamp });
				return Ok(resource_data_end_pointer)
			}

			Some(signature_expiration_seconds, signature_inception_seconds, difference) => if unlikely!(difference <= 0)
			{
				resource_record_visitor.RRSIG_ignored(resource_record_name, DifferenceInSignatureInceptionAndExpirationWasNegativeOrZero { signature_inception_timestamp, signature_expiration_timestamp });
				return Ok(resource_data_end_pointer)
			}
			else
			{
				let now = get_time();

				const PeriodLengthInSeconds: i64 = ::std::u32::MAX as i64;

				// TODO: Increment this to 1 after Sunday, February 7, 2106 6:28:15 AM GMT.
				const ElapsedWrapAroundPoints: i64 = 0;
				const LastWrapAroundPoint: i64 = PeriodLengthInSeconds * ElapsedWrapAroundPoints;
				const NextWrapAroundPoint: i64 = LastWrapAroundPoint + PeriodLengthInSeconds;

				let signature_inception_timespec = if unlikely!(signature_inception_seconds > signature_expiration_seconds)
				{
					Timespec::new(NextWrapAroundPoint + signature_inception_seconds as i64, 0)
				}
				else
				{
					Timespec::new(LastWrapAroundPoint + signature_inception_seconds as i64, 0)
				};
				if unlikely!(signature_inception_timespec > now)
				{
					resource_record_visitor.RRSIG_ignored(resource_record_name, InceptionIsInTheFuture { signature_inception_timestamp, signature_expiration_timestamp });
					return Ok(resource_data_end_pointer)
				}

				let signature_expiration_timespec = Timespec::new(LastWrapAroundPoint + signature_expiration_seconds as i64);
				if unlikely!(signature_expiration_timespec <= now)
				{
					resource_record_visitor.RRSIG_ignored(resource_record_name, Expired { signature_inception_timestamp, signature_expiration_timestamp });
					return Ok(resource_data_end_pointer)
				}
			},
		}

		let remaining_data = &resource_data[(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize) .. ];
		let remaining_data_pointer = remaining_data.pointer();

		let (signers_name, end_of_name_pointer) = ParsedNameIterator::parse_without_compression(remaining_data.pointer(), resource_data_end_pointer)?;

		let signature_offset = TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize + KeyTagSize + (end_of_name_pointer - remaining_data_pointer);
		let signature = &resource_data[signature_offset .. ];

		let record = ResourceRecordSetSignature
		{
			type_covered: resource_data.value::<DataType>(0),
			security_algorithm,
			labels,
			original_time_to_live: resource_data.value::<TimeToLiveInSeconds>(TypeCoveredSize + AlgorithmSize + LabelsSize),
			key_tag: resource_data.value::<KeyTag>(TypeCoveredSize + AlgorithmSize + LabelsSize + OriginalTimeToLiveSize + SignatureExpirationSize + SignatureInceptionSize),
			signers_name,
			signature,
			rrsig_rdata_excluding_signature_field: &resource_data[ .. signature_offset],
		};

		resource_record_visitor.RRSIG(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_dnskey<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::DnsKeyPurpose::*;
		use self::DnsKeyResourceRecordIgnoredBecauseReason::*;

		const FlagsSize: usize = 2;
		const ProtocolSize: usize = 1;
		const AlgorithmSize: usize = 1;
		const MinimumPublicKeySize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < FlagsSize + ProtocolSize)
		{
			return Err(ResourceDataForTypeDNSKEYHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let protocol = resource_data.u8(FlagsSize);
		if unlikely!(protocol != 3)
		{
			resource_record_visitor.DNSKEY_ignored(resource_record_name, ProtocolWasNot3(protocol));
			return Ok(resource_data_end_pointer)
		}

		if unlikely!(length < FlagsSize + ProtocolSize + AlgorithmSize + MinimumPublicKeySize)
		{
			return Err(ResourceDataForTypeDNSKEYHasAnIncorrectLength(length))
		}

		let flags = resource_data.u16_network_endian(0);

		const ZONE: u16 = 7;
		#[cfg(target_endian = "big")] const IsZoneKeyFlag: u16 = 1 << (15 - ZONE);
		#[cfg(target_endian = "little")] const IsZoneKeyFlag: u16 = 1 << ((15 - ZONE) - 8);

		const REVOKE: u16 = 8;
		#[cfg(target_endian = "big")] const RevokedFlag: u16 = 1 << (15 - REVOKE);
		#[cfg(target_endian = "little")] const RevokedFlag: u16 = 1 << ((15 - REVOKE) + 8);

		const SEP: u16 = 15;
		#[cfg(target_endian = "big")] const SecureEntryPointFlag: u16 = 1 << (15 - SEP);
		#[cfg(target_endian = "little")] const SecureEntryPointFlag: u16 = 1 << ((15 - SEP) + 8);

		const KnownFlags: u16 = IsZoneKeyFlag | IsZoneKeyFlag | SecureEntryPointFlag;

		if unlikely!(flags & !KnownFlags != 0)
		{
			resource_record_visitor.DNSKEY_ignored(resource_record_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let is_revoked = flags & RevokedFlagBit != 0;
		if unlikely!(is_revoked)
		{
			resource_record_visitor.DNSKEY_ignored(resource_record_name, Revoked);
			return Ok(resource_data_end_pointer)
		}

		let is_zone_key = flags & IsZoneKeyFlagBit != 0;
		let is_secure_entry_point = flags & SecureEntryPointFlagBit != 0;

		let purpose = if unlikely!(is_zone_key)
		{
			ZoneSigningKey { is_secure_entry_point }
		}
		else
		{
			if unlikely!(is_secure_entry_point)
			{
				resource_record_visitor.DNSKEY_ignored(resource_record_name, SecureEntryPointFlagSetButNotZoneKeyFlag);
				return Ok(resource_data_end_pointer)
			}
			KeySigningKey
		};

		let security_algorithm_type = resource_data.u16(FlagsSize + ProtocolSize);
		let security_algorithm = match SecurityAlgorithm::parse_security_algorithm(security_algorithm_type, false, false)?
		{
			Left(security_algorithm) => security_algorithm,
			Right(security_algorithm_rejected_because_reason) =>
			{
				resource_record_visitor.DNSKEY_ignored(resource_record_name, SecurityAlgorithmRejected(security_algorithm_rejected_because_reason));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = DnsKey
		{
			computed_key_tag: resource_data.key_tag(),
			purpose,
			security_algorithm,
			public_key: &resource_data[(FlagsSize + ProtocolSize + AlgorithmSize) .. ],
		};

		resource_record_visitor.DNSKEY(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_dhcid<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::DhcidDigest::*;
		use self::DhcidResourceRecordIgnoredBecauseReason::*;

		const IdentifierTypeCodeSize: usize = 2;
		const DigestTypeCodeSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < IdentifierTypeCodeSize + DigestTypeCodeSize + MinimumDigestSize)
		{
			return Err(ResourceDataForTypeDHCIDHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let identifier_type_code = resource_data.u16(0);
		let identifier_type: IdentifierType = match identifier_type_code
		{
			0x0000 ... 0x0002 => unsafe { transmute(identifier_type_code) },

			0x0003 ... 0xFFFE =>
			{
				resource_record_visitor.DHCID_ignored(resource_record_name, IdentifierTypeUnassigned(identifier_type_code));
				return Ok(resource_data_end_pointer)
			}

			_ => return Err(ResourceDataForTypeDHCIDHasAReservedIdentifierTypeCode)
		};

		const DigestOffset: usize = IdentifierTypeCodeSize + DigestTypeCodeSize;
		let digest_type_code = resource_data.u8(IdentifierTypeCodeSize);
		let digest = match raw_digest_type_code
		{
			0 => return Err(ResourceDataForTypeDHCIDHasAReservedDigestTypeCode),

			1 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 256, Sha2_256, ResourceDataForTypeDHCIDHasADigestLengthThatIsIncorrectForTheMatchingType),

			_ =>
			{
				resource_record_visitor.DHCID_ignored(resource_record_name, DigestAlgorithmUnassigned(digest_type_code));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = Dhcid
		{
			identifier_type,
			digest,
		};

		resource_record_visitor.DHCID(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	fn handle_nsec3<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::NextSecureVersion3ResourceRecordIgnoredBecauseReason::*;

		const HashAlgorithmSize: usize = 1;
		const FlagsSize: usize = 1;
		const IterationsSize: usize = 2;
		const SaltLengthSize: usize = 1;
		const MinimumSaltSize: usize = 0;
		const HashLengthSize: usize = 1;
		const MinimumHashSize: usize = 0;
		const SaltStartOffset: usize = HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize;

		let length = resource_data.len();
		if unlikely!(length < HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize + MinimumSaltSize + HashLengthSize + MinimumHashSize + TypeBitmaps::MinimumTypeBitmapsSize)
		{
			return Err(ResourceDataForTypeNSEC3HasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let salt_length = resource_data.u8_as_usize(HashAlgorithmSize + FlagsSize + IterationsSize);
		let salt_end_offset = HashAlgorithmSize + FlagsSize + IterationsSize + SaltLengthSize + salt_length;

		if unlikely!(salt_end_offset > length)
		{
			return Err(ResourceDataForTypeNSEC3HasAnOverflowingSaltLength(salt_length))
		}
		let salt = &resource_data[SaltStartOffset .. salt_end_offset];

		let hash_algorithm_number = resource_data.u8(0);
		let (next_hashed_owner_name, hash_end_offset) = match hash_algorithm_number
		{
			0 => return Err(ResourceDataForTypeNSEC3HasAReservedHashAlgorithm),

			1 =>
			{
				let hash_length = resource_data.u8_as_usize(salt_end_offset);

				const DigestSizeInBits: usize = 160;
				const DigestSize: usize = DigestSizeInBits / DigestSizeInBits;

				if unlikely!(hash_length != DigestSize)
				{
					return Err(ResourceDataForTypeNSEC3HasAnIncorrectHashLengthForASha1Hash(length))
				}

				let hash_start_offset = salt_end_offset + HashLengthSize;

				let hash_end_offset = hash_start_offset + hash_length;

				if unlikely!(hash_end_offset > length)
				{
					return Err(ResourceDataForTypeNSEC3HasAnOverflowingHashLength(hash_length))
				}

				let hash = NextSecureVersion3Hash::Sha_1(resource_data.cast::<[u8; DigestSize]>(hash_start_offset));

				(hash, hash_end_offset)
			}

			_ =>
			{
				resource_record_visitor.NSEC3_ignored(UnassignedHashAlgorithm(hash_algorithm_number));
				Ok(resource_data_end_pointer)
			}
		};

		let flags = resource_data.u8(HashAlgorithmSize);

		const OptOut: u8 = 7;
		const OptOutFlag: u8 = 1 << (7 - OptOut);
		const KnownFlags: u8 = OptOutFlag;

		if unlikely!(flags & !KnownFlags != 0)
		{
			resource_record_visitor.NSEC3_ignored(resource_record_name, UnassignedFlags(flags));
			return Ok(resource_data_end_pointer)
		}

		let record = NextSecureVersion3
		{
			opt_out: flags & OptOutFlag != 0,

			iterations: resource_data.u16(HashAlgorithmSize + FlagsSize),

			salt,

			next_hashed_owner_name,

			type_bitmaps: TypeBitmaps::parse_type_bitmaps(&resource_data[hash_end_offset .. ])?,
		};

		resource_record_visitor.NSEC3(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_tlsa<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (resource_data_end_pointer, either) = self.handle_tlsa_or_smimea(end_of_name_pointer, end_of_message_pointer)?;

		match either
		{
			Left((time_to_live, record)) => resource_record_visitor.TLSA(resource_record_name, time_to_live, record)?,

			Right(resource_record_ignored_because_reason) => resource_record_visitor.TLSA_ignored(resource_record_name, resource_record_ignored_because_reason),
		}

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_smimea<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (resource_data_end_pointer, either) = self.handle_tlsa_or_smimea(end_of_name_pointer, end_of_message_pointer)?;

		match either
		{
			Left((time_to_live, record)) => resource_record_visitor.SMIMEA(resource_record_name, time_to_live, record)?,

			Right(resource_record_ignored_because_reason) => resource_record_visitor.SMIMEA_ignored(resource_record_name, resource_record_ignored_because_reason),
		}

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_hip<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::HostIdentityProtocolResourceRecordIgnoredBecauseReason::*;

		const HostIdentityTagLengthSize: usize = 1;
		const PublicKeyAlgorithmTypeSize: usize = 1;
		const PublicKeyLengthSize: usize = 2;
		const MinimumHostIdentityTagLength: usize = 0;
		const MinimumPublicKeyLength: usize = 0;
		const MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne: usize = Self::MinimumNameSize;
		const HostIdentityTagOffset: usize = HostIdentityTagLengthSize + PublicKeyAlgorithmTypeSize + PublicKeyLengthSize;

		let length = resource_data.len();
		if unlikely!(length < HostIdentityTagOffset + MinimumHostIdentityTagLength + MinimumPublicKeyLength + MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne)
		{
			return Err(ResourceDataForTypeHIPHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let host_identity_tag_length = resource_data.u8_as_usize(0);
		if unlikely!(length < HostIdentityTagOffset + host_identity_tag_length + MinimumPublicKeyLength + MinimumNumberOfRendezvousServersIsOneSoMinimumNameSizeIsOne)
		{
			return Err(ResourceDataForTypeHIPHasAnIncorrectLength(length))
		}

		let public_key_algorithm_type = resource_data.u8(HostIdentityTagLengthSize);
		let public_key_starts_at_offset = HostIdentityTagOffset + host_identity_tag_length;
		let public_key_length = resource_data.u16_as_usize(HostIdentityTagLengthSize + PublicKeyAlgorithmTypeSize);
		let public_key = ipsec_like_public_key!(public_key_algorithm_type, resource_data, public_key_starts_at_offset, public_key_length, resource_data_end_pointer, { resource_record_visitor.HIP_ignored(PublicKeyAlgorithmDSAIsProbablyBroken) }, { resource_record_visitor.HIP_ignored(PublicKeyAlgorithmUnassigned(public_key_algorithm_type)) })?;

		let start_of_name_pointer = resource_data.pointer() + HostIdentityTagOffset + host_identity_tag_length + public_key_length;
		let (first_rendezvous_server_domain_name, true_end_of_name_pointer) = ParsedNameIterator::parse_without_compression(start_of_name_pointer, resource_data_end_pointer)?;

		let remaining_rendezvous_servers_length = resource_data_end_pointer - true_end_of_name_pointer;
		let remaining_rendezvous_server_domain_names = unsafe { from_raw_parts(true_end_of_name_pointer as *const u8, remaining_rendezvous_servers_length) };

		let record = HostIdentityProtocol
		{
			host_identity_tag: &resource_data[HostIdentityTagOffset .. public_key_starts_at_offset],

			public_key,

			first_rendezvous_server_domain_name,

			remaining_rendezvous_server_domain_names,
		};

		resource_record_visitor.HIP(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_openpgpkey<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		resource_record_visitor.OPENPGPKEY(resource_record_name, time_to_live, resource_data)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_nid<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const PreferenceSize: usize = 2;
		const NodeIdentifierSize: usize = 8;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + NodeIdentifierSize)
		{
			return Err(ResourceDataForTypeNIDHasAnIncorrectLength(length))
		}

		let record = NodeIdentifier
		{
			preference: resource_data.u16(0),
			node_identifier: resource_data.u64(PreferenceSize),
		};

		resource_record_visitor.NID(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_l32<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const PreferenceSize: usize = 2;
		const LocatorSize: usize = 4;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + LocatorSize)
		{
			return Err(ResourceDataForTypeL32HasAnIncorrectLength(length))
		}

		let record = Locator32
		{
			preference: resource_data.u16(0),
			locator: resource_data.cast::<Ipv4Addr>(PreferenceSize),
		};

		resource_record_visitor.L32(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_l64<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const PreferenceSize: usize = 2;
		const LocatorSize: usize = 8;

		let length = resource_data.len();
		if unlikely!(length != PreferenceSize + LocatorSize)
		{
			return Err(ResourceDataForTypeL64HasAnIncorrectLength(length))
		}

		let record = Locator64
		{
			preference: resource_data.u16(0),
			locator: resource_data.value::<[u8; LocatorSize]>(PreferenceSize),
		};

		resource_record_visitor.L64(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_lp<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const PreferenceSize: usize = 2;
		const MinimumNameSize: usize = Self::MinimumNameSize;

		let length = resource_data.len();
		if unlikely!(length < PreferenceSize + MinimumNameSize)
		{
			return Err(ResourceDataForTypeLPHasTooShortALength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let domain_name_data = &resource_data[PreferenceSize .. ];
		let (domain_name, end_of_name_pointer) = ParsedNameIterator::parse_without_compression(domain_name_data.pointer(), resource_data_end_pointer)?;
		if unlikely!(end_of_name_pointer != resource_data_end_pointer)
		{
			return Err(ResourceDataForTypeLPHasDataLeftOver)
		}

		let record = LocatorPointer
		{
			preference: resource_data.u16(0),
			domain_name,
		};

		resource_record_visitor.LP(resource_record_name, time_to_live, record)?;
		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_eui48<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const Eui48Size: usize = 48 / Self::BitsInAByte;

		let length = resource_data.len();
		if unlikely!(length != Eui48Size)
		{
			return Err(ResourceDataForTypeEUI48HasAnIncorrectLength(length))
		}

		let record = resource_data.cast::<[u8; Eui48Size]>(0);

		resource_record_visitor.EUI48(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_eui64<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const Eui64Size: usize = 64 / Self::BitsInAByte;

		let length = resource_data.len();
		if unlikely!(length != Eui64Size)
		{
			return Err(ResourceDataForTypeEUI64HasAnIncorrectLength(length))
		}

		let record = resource_data.cast::<[u8; Eui64Size]>(0);

		resource_record_visitor.EUI64(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_uri<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		const PrioritySize: usize = 2;
		const WeightSize: usize = 2;
		const MinimumTargetSize: usize = 1;

		let length = resource_data.len();
		if unlikely!(length < PrioritySize + WeightSize + MinimumTargetSize)
		{
			return Err(ResourceDataForTypeURIHasAnIncorrectLength(length))
		}

		let record = Uri
		{
			priority: resource_data.u16(0),
			weight: resource_data.u16(PrioritySize),
			target_uri: &resource_data[(PrioritySize + WeightSize) .. ],
		};

		resource_record_visitor.URI(resource_record_name, time_to_live, record)?;
		Ok(resource_data.end_pointer())
	}

	#[inline(always)]
	fn handle_caa<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, resource_record_name: ParsedNameIterator<'a>, resource_record_visitor: &mut impl ResourceRecordVisitor) -> Result<usize, DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::CertificateAuthorityAuthorizationPropertyTag::*;
		use self::CertificateAuthorityAuthorizationResourceRecordIgnoredBecauseReason::*;

		const FlagsSize: usize = 1;
		const TagLengthSize: usize = 1;
		const MinimumTagSize: usize = 1;
		const MinimumValueSize: usize = 0;
		const PropertyTagOffset: usize = FlagsSize + TagLengthSize;

		let length = resource_data.len();
		if unlikely!(length < FlagsSize + TagLengthSize + MinimumTagSize + MinimumValueSize)
		{
			return Err(ResourceDataForTypeCAAHasAnIncorrectLength(length))
		}

		let tag_length = resource_data.u8(FlagsSize);

		if unlikely!(tag_length == 0)
		{
			return Err(ResourceDataForTypeCAAHasAZeroTagLength)
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		if unlikely!(tag_length > 15)
		{
			resource_record_visitor.CAA_ignored(resource_record_name, TagLengthExceeded15(tag_length));
			return Ok(resource_data_end_pointer)
		}

		let property_value_offset = PropertyTagOffset + tag_length;

		if unlikely!(property_value_offset > length)
		{
			return Err(ResourceDataForTypeCAAHasAnIncorrectLength(length))
		}

		let flag_bits = resource_data.u8(0);

		// // See <https://www.iana.org/assignments/pkix-parameters/pkix-parameters.xhtml>; note that bit 0 is MSB, ie bits are numbered from left-to-right.
		const IssuerCriticalFlagBit: u8 = 0b1000_0000;
		const ReservedFlagBits: u8 = !IssuerCriticalFlagBit;

		if unlikely!(flag_bits & ReservedFlagBits != 0)
		{
			resource_record_visitor.CAA_ignored(resource_record_name, UseOfUnassignedFlagBits);
			return Ok(resource_data_end_pointer)
		}

		static KnownTags: Map<&'static str, Option<CertificateAuthorityAuthorizationPropertyTag>> = phf_map!
		{
    		b"issue" => Some(AuthorizationEntryByDomain),
    		b"issuewild" => Some(AuthorizationEntryByWildcardDomain),
    		b"iodef" => Some(ReportIncidentByIodefReport),
    		b"contactemail" => Some(AuthorizedEMailContactForDomainValidation),
    		b"auth" => None,
    		b"path" => None,
    		b"policy" => None,
		};

		let property_tag_bytes = &resource_data[PropertyTagOffset .. property_value_offset];
		let property_tag = match KnownTags.get(property_tag_bytes)
		{
			Some(Some(property_tag)) => property_tag,

			Some(None) =>
			{
				resource_record_visitor.CAA_ignored(resource_record_name, TagReservedByRfcErrata3547(property_tag_bytes));
				return Ok(resource_data_end_pointer)
			}

			None =>
			{
				resource_record_visitor.CAA_ignored(resource_record_name, TagUnassigned(property_tag_bytes));
				return Ok(resource_data_end_pointer)
			}
		};

		let record = CertificationAuthorityAuthorization
		{
			is_issuer_critical: flags & 0b0000_0001 != 0,
			property_tag,
			property_value: &resource_data[property_value_offset .. ],
		};

		resource_record_visitor.CAA(resource_record_name, time_to_live, record)?;

		Ok(resource_data_end_pointer)
	}

	#[inline(always)]
	fn handle_tlsa_or_smimea<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<(usize, Either<(TimeToLiveInSeconds, DnsBasedAuthenticationOfNamedEntities<'a>), DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason>), DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		use self::DnsBasedAuthenticationOfNamedEntitiesResourceRecordIgnoredBecauseReason::*;

		const CertificateUsageSize: usize = 1;
		const SelectorSize: usize = 1;
		const MatchingTypeSize: usize = 1;
		const MinimumDigestSize: usize = 0;

		let length = resource_data.len();
		if unlikely!(length < CertificateUsageSize + SelectorSize + MatchingTypeSize + MinimumDigestSize)
		{
			return Err(ResourceDataForTypeTLSAOrSMIMEAHasAnIncorrectLength(length))
		}

		let resource_data_end_pointer = resource_data.end_pointer();

		let raw_certificate_usage = resource_data.u8(0);
		let certificate_usage: CertificateUsage = match raw_certificate_usage
		{
			0 ... 3 => unsafe { transmute(raw_certificate_usage) },

			4 ... 254 => return Ok((resource_data_end_pointer, Right(UnassignedCertificateUsage(raw_certificate_usage)))),

			255 => return Ok((resource_data_end_pointer, Right(PrivateCertificateUsage))),
		};

		let raw_selector = resource_data.u8(CertificateUsageSize);
		let selector: Selector = match raw_selector
		{
			0 ... 1 => unsafe { transmute(raw_selector) },

			2 ... 254 => return Ok((resource_data_end_pointer, Right(UnassignedSelector(raw_selector)))),

			255 => return Ok((resource_data_end_pointer, Right(PrivateSelector))),
		};

		const DigestOffset: usize = CertificateUsageSize + SelectorSize + MatchingTypeSize;
		use self::MatchingType::*;
		let raw_matching_type = resource_data.u8(CertificateUsageSize + SelectorSize);
		let matching_type = match raw_matching_type
		{
			0 => NoHashUsed(&resource_data[DigestOffset .. ]),

			1 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 256, Sha2_256, ResourceDataForTypeTLSAOrSMIMEAHasADigestLengthThatIsIncorrectForTheMatchingType),

			2 => guard_hash_digest_if_final_field!(resource_data, DigestOffset, 512, Sha2_512, ResourceDataForTypeTLSAOrSMIMEAHasADigestLengthThatIsIncorrectForTheMatchingType),

			2 ... 254 => return Ok((resource_data_end_pointer, Right(UnassignedMatchingType(raw_matching_type)))),

			255 => return Ok((resource_data_end_pointer, Right(PrivateMatchingType))),
		};

		Ok
		(
			(
				resource_data_end_pointer,
				Left
				(
					(
						time_to_live,
						DnsBasedAuthenticationOfNamedEntities
						{
							certificate_usage,
							selector,
							matching_type,
						}
					)
				)
			)
		)
	}


	#[inline(always)]
	fn validate_minimum_record_size_and_parse_name_and_resource_record_type<'a>(&'a self, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>) -> Result<(ParsedNameIterator<'a>, usize, (u8, u8)), DnsProtocolError>
	{
		let start_of_resource_record_pointer = self as *const Self as usize;

		if unlikely!(start_of_resource_record_pointer + Self::MinimumSize > end_of_message_pointer)
		{
			return Err(ResourceRecordIsShorterThanMinimumSize)
		}

		let (parsed_name_iterator, end_of_name_pointer) = parsed_labels.parse_name(start_of_resource_record_pointer, end_of_message_pointer)?;
		debug_assert!(end_of_name_pointer < end_of_message_pointer, "end_of_name_pointer exceeds end_of_message_pointer");

		if unlikely!(end_of_name_pointer + ResourceRecordFooter::MinimumSize > end_of_message_pointer)
		{
			return Err(ResourceRecordIsShorterThanMinimumSizeAfterParsingName)
		}

		let resource_record_type = self.resource_record_type(end_of_name_pointer);
		let resource_record_type_bytes = &resource_record_type.0;

		let type_upper = resource_record_type_bytes.u8(0);
		let type_lower = resource_record_type_bytes.u8(1);

		Ok((parsed_name_iterator, end_of_name_pointer, (type_upper, type_lower)))
	}

	#[inline(always)]
	fn parse_internet_protocol_address_only<'a, Address: 'a + Sized>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<(TimeToLiveInSeconds, &'a Address, usize), DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let length = resource_data.len();
		if unlikely!(length != size_of::<Address>())
		{
			Err(ResourceDataForTypeAOrAAAAHasAnIncorrectLength(length))
		}
		else
		{
			let address = resource_data.cast::<Address>(0);
			Ok((time_to_live, address, resource_data.end_pointer()))
		}
	}

	#[inline(always)]
	fn parse_name_only<'a>(&'a self, end_of_name_pointer: usize, end_of_message_pointer: usize, parsed_labels: &mut ParsedLabels<'a>) -> Result<(TimeToLiveInSeconds, ParsedNameIterator<'a>, usize), DnsProtocolError>
	{
		let (time_to_live, resource_data) = self.validate_class_is_internet_and_get_time_to_live_and_resource_data(end_of_name_pointer, end_of_message_pointer)?;

		let record = parsed_labels.parse_name_in_slice_with_nothing_left(resource_data)?;
		Ok((time_to_live, record, resource_data.end_pointer()))
	}

	#[inline(always)]
	fn validate_class_is_internet_and_get_time_to_live_and_resource_data(&self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<(TimeToLiveInSeconds, &[u8]), DnsProtocolError>
	{
		let class = self.resource_record_class(end_of_name_pointer)?;
		debug_assert_eq!(class, ResourceRecordClass::Internet, "Why do we support classes other than Internet?");

		let resource_data = self.safely_access_resource_data(end_of_name_pointer, end_of_message_pointer)?;
		let time_to_live = self.time_to_live(end_of_name_pointer);

		Ok((time_to_live, resource_data))
	}

	#[inline(always)]
	fn safely_access_resource_data(&self, end_of_name_pointer: usize, end_of_message_pointer: usize) -> Result<&[u8], DnsProtocolError>
	{
		let resource_data_length = self.resource_data_length(end_of_name_pointer) as usize;
		if unlikely!(end_of_name_pointer + resource_data_length > end_of_message_pointer)
		{
			Err(ResourceDataLengthOverflows)
		}
		else
		{
			Ok(unsafe { from_raw_parts(self.resource_data(end_of_name_pointer) as *const ResourceData as *const u8, resource_data_length) })
		}
	}

	/// `NAME` field.
	///
	/// For an `OPT` record, this should always just be `0x00`.
	#[inline(always)]
	fn name(&self) -> &Name
	{
		unsafe { & * (self as *const Self as *const Name) }
	}

	/// `NAME` field.
	///
	/// For an `OPT` record, this should always just be `0x00`.
	#[inline(always)]
	fn name_mutable(&mut self) -> &mut Name
	{
		unsafe { &mut * (self as *mut Self as *mut Name) }
	}

	/// `TYPE` field.
	#[inline(always)]
	fn resource_record_type(&self, end_of_name_pointer: usize) -> DataType
	{
		self.footer(end_of_name_pointer).resource_record_type()
	}

	/// `TYPE` field.
	#[inline(always)]
	fn set_resource_record_type(&mut self, end_of_name_pointer: usize, resource_record_type: DataType)
	{
		self.footer_mutable(end_of_name_pointer).set_resource_record_type(resource_record_type)
	}

	/// `CLASS` field.
	#[inline(always)]
	fn resource_record_class(&self, end_of_name_pointer: usize) -> Result<ResourceRecordClass, DnsProtocolError>
	{
		self.footer(end_of_name_pointer).resource_record_class()
	}

	/// `CLASS` field.
	#[inline(always)]
	fn set_resource_record_class(&mut self, end_of_name_pointer: usize, resource_record_class: ResourceRecordClass)
	{
		self.footer_mutable(end_of_name_pointer).set_resource_record_class(resource_record_class)
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	#[inline(always)]
	fn requestors_udp_payload_size(&self, end_of_name_pointer: usize) -> u16
	{
		self.footer(end_of_name_pointer).requestors_udp_payload_size()
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	///
	/// `size` is typically a MTU, like 1280; realistically, as we use TCP, we should probably set this to 4Kb or some such.
	#[inline(always)]
	fn set_requestors_udp_payload_size(&mut self, end_of_name_pointer: usize, size: u16)
	{
		self.footer_mutable(end_of_name_pointer).set_requestors_udp_payload_size(size)
	}

	/// `TTL` field.
	#[inline(always)]
	fn time_to_live(&self, end_of_name_pointer: usize) -> TimeToLiveInSeconds
	{
		self.footer(end_of_name_pointer).time_to_live()
	}

	/// `TTL` field.
	#[inline(always)]
	fn set_time_to_live(&mut self, end_of_name_pointer: usize, time_to_live: TimeToLiveInSeconds)
	{
		self.footer_mutable(end_of_name_pointer).set_time_to_live(time_to_live)
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	#[inline(always)]
	fn extended_response_code_and_flags(&self, end_of_name_pointer: usize) -> ExtendedResponseCodeAndFlags
	{
		self.footer(end_of_name_pointer).extended_response_code_and_flags()
	}

	/// `EDNS(0)` field.
	///
	/// RFC 6891.
	#[inline(always)]
	fn set_extended_response_code_and_flags(&mut self, end_of_name_pointer: usize, value: ExtendedResponseCodeAndFlags)
	{
		self.footer_mutable(end_of_name_pointer).set_extended_response_code_and_flags(value)
	}

	/// `RDLEN` field.
	#[inline(always)]
	fn resource_data_length(&self, end_of_name_pointer: usize) -> u16
	{
		self.footer(end_of_name_pointer).resource_data_length()
	}

	/// `RDLEN` field.
	#[inline(always)]
	fn set_resource_data_length(&mut self, end_of_name_pointer: usize, length: u16)
	{
		self.footer_mutable(end_of_name_pointer).set_resource_data_length(length)
	}

	/// `RDATA` field.
	#[inline(always)]
	fn resource_data(&self, end_of_name_pointer: usize) -> &ResourceData
	{
		self.footer(end_of_name_pointer).resource_data()
	}

	/// `RDATA` field.
	#[inline(always)]
	pub fn resource_data_mutable(&mut self, end_of_name_pointer: usize) -> &mut ResourceData
	{
		self.footer_mutable(end_of_name_pointer).resource_data_mutable()
	}

	#[inline(always)]
	fn footer(&self, end_of_name_pointer: usize) -> &ResourceRecordFooter
	{
		unsafe { & * (end_of_name_pointer as *const ResourceRecordFooter) }
	}

	#[inline(always)]
	fn footer_mutable(&self, end_of_name_pointer: usize) -> &mut ResourceRecordFooter
	{
		unsafe { &mut * (end_of_name_pointer as *mut ResourceRecordFooter) }
	}
}

