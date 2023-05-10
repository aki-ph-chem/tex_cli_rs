#!/usr/bin/bash

src="tex_rs"
pkgname="tex_cli_rs"
pkgver="1.0"
tarball_name="${pkgname}-${pkgver}"

if [ ! -e "${tarball_name}" ];then
    mkdir "${tarball_name}"
    cp -r "../${src}/src" "${tarball_name}/src"
    cp  "../${src}/Cargo.toml" "${tarball_name}/"
fi

if [ ! -e "${tarball_name}.tar.gz" ]; then
    tar -cvzf "${tarball_name}.tar.gz" "${tarball_name}"
fi

makepkg -g >> PKGBUILD
makepkg -s
