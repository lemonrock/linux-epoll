// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


use super::*;


/// Horrible hack to make public a static from rustls.
pub static TLS13_CHACHA20_POLY1305_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[0];

/// Horrible hack to make public a static from rustls.
pub static TLS13_AES_256_GCM_SHA384: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[1];

/// Horrible hack to make public a static from rustls.
pub static TLS13_AES_128_GCM_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[2];

/// Horrible hack to make public a static from rustls.
pub static TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[3];

/// Horrible hack to make public a static from rustls.
pub static TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[4];

/// Horrible hack to make public a static from rustls.
pub static TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[5];

/// Horrible hack to make public a static from rustls.
pub static TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[6];

/// Horrible hack to make public a static from rustls.
pub static TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[7];

/// Horrible hack to make public a static from rustls.
pub static TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256: &'static SupportedCipherSuite = &ALL_CIPHERSUITES[8];
