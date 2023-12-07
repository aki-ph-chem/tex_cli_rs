#!/usr/bin/bash

src="tex_rs"
pkgname="tex_cli_rs"
pkgver="1.2.2"
arch="x86_64"
tarball_name="${pkgname}-${pkgver}"
pkgname="${tarball_name}-1-${arch}"

flist=("${tarball_name}" "${tarball_name}.tar.gz" "${pkgname}.pkg.tar.zst" "src" "pkg")

for f in "${flist[@]}";do
    if [ -e "${f}" ];then
        rm -r "${f}"
    fi
done
