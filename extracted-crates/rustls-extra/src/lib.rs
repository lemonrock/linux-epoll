// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]


//! #rustls-extra
//! 
//! rustls-extra contains useful abstractions to make it easier to work with the logic in rustls for epoll-based servers and clients.


extern crate ct_logs;
#[macro_use] extern crate likely;
extern crate indexmap;
extern crate rustls;
pub extern crate webpki;


use self::supported_cipher_suites::*;
pub use ::ct_logs::LOGS as GooglesKnownListOfCertificateTransparencyLogs;
pub use ::indexmap::IndexSet;
use ::rustls::ALL_CIPHERSUITES;
pub use ::rustls::AllowAnyAnonymousOrAuthenticatedClient;
pub use ::rustls::AllowAnyAuthenticatedClient;
pub use ::rustls::Certificate;
pub use ::rustls::ClientCertVerifier;
pub use ::rustls::ClientConfig;
pub use ::rustls::ClientSession;
pub use ::rustls::ClientSessionMemoryCache;
pub use ::rustls::NoClientAuth;
pub use ::rustls::NoClientSessionStorage;
pub use ::rustls::NoServerSessionStorage;
pub use ::rustls::PrivateKey;
pub use ::rustls::ProtocolVersion;
pub use ::rustls::RootCertStore;
pub use ::rustls::ServerConfig;
pub use ::rustls::ServerSession;
pub use ::rustls::ServerSessionMemoryCache;
pub use ::rustls::Session;
pub use ::rustls::SupportedCipherSuite;
pub use ::rustls::Ticketer;
pub use ::rustls::TLSError;
pub use ::rustls::WriteV;
pub use ::rustls::internal::pemfile::*;
use ::std::error;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fs::File;
use ::std::io;
use ::std::io::BufReader;
use ::std::io::Read;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::path::PathBuf;
use ::std::sync::Arc;
pub use ::webpki::DNSName;
pub use ::webpki::DNSNameRef;


/// Horrible hack to export references from rustls for supported cipher suites.
pub mod supported_cipher_suites;


include!("ApplicationLayerProtocolNegotiationProtocol.rs");
include!("ApplicationLayerProtocolNegotiationProtocols.rs");
include!("CertificateChainAndPrivateKey.rs");
include!("CertificateChainAndPrivateKeyError.rs");
include!("ClientAuthenticationConfiguration.rs");
include!("CommonTlsPostHandshakeInformation.rs");
include!("RootCertificateStoreLoadError.rs");
include!("ServerNameIndication.rs");
include!("SupportedTlsVersions.rs");
include!("TlsClientConfiguration.rs");
include!("TlsClientConfigurationError.rs");
include!("TlsCommonConfiguration.rs");
include!("TlsServerConfiguration.rs");
include!("TlsServerConfigurationError.rs");

