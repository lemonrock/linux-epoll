// This file is part of linux-epoll. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT. No part of linux-epoll, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of linux-epoll. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-epoll/master/COPYRIGHT.


use super::*;


/*
> CONNECT server.example.com:80 HTTP/1.1
> Host: server.example.com:80

< 407 Proxy Authentication Required
< Proxy-Authenticate: XXXX
	Will reply <type> realm=<relam>
	See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Proxy-Authenticate

> Proxy-Authorization: Basic YWxhZGRpbjpvcGVuc2VzYW1l  (where YWxhZGRpbjpvcGVuc2VzYW1l is base64-encoding of username:password, in this case aladdin:opensesame)

`Basic` is one of several schemes: https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml
See also: https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication#Authentication_schemes


For HTTP CONNECT, see https://tools.ietf.org/html/rfc7231#section-4.3.6



Proxy-Authenticate examples:-

Proxy-Authenticate: Basic
Proxy-Authenticate: Basic realm="Access to the internal site"
Proxy-Authenticate: Newauth realm="apps", type=1, title="Login to \"apps\"", Basic realm="simple"

See https://tools.ietf.org/html/rfc7235#section-4.3 and https://tools.ietf.org/html/rfc7235#section-4.1 (WWW-Authenticate, which shares rules for parsing)

// Furthermore, the header field itself can occur multiple times

Bearer auth uses the "scope" parameter.


Additionally, a http_connect proxy needs to work over TLS, so that Basic auth parameters can be passed unencrypted.
Look at squid?

*/
