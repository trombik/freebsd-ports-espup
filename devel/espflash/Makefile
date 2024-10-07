PORTNAME=	espflash
DISTVERSION=	3.1.1
DISTVERSIONPREFIX=	v
CATEGORIES=	devel
MASTER_SITES=

MAINTAINER=	y@trombik.org
COMMENT=	Serial flasher utility for Espressif SoCs and modules based on esptool.py

LICENSE=	MIT APACHE20
LICENSE_COMB=	dual
WWW=	https://github.com/esp-rs/espflash

BUILD_DEPENDS=	rust-nightly>0:lang/rust-nightly
LIB_DEPENDS=	libcurl.so:ftp/curl \
	libssh2.so:security/libssh2 \
	libgit2.so:devel/libgit2

USES=	cargo ssl
USE_GITHUB=	yes
GH_TUPLE=	esp-rs:${PORTNAME}:${DISTVERSIONPREFIX}${DISTVERSION}

# XXX disable CARGO_BUILDDEP because cargo.mk add lang/rust, not
# lang/rust-nightly, to BUILD_DEPENDS,
CARGO_BUILDDEP=	no
CARGO_INSTALL_PATH=	cargo-espflash espflash

.include <bsd.port.mk>
