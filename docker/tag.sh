#!/usr/bin/env sh

case "${1}" in

    "linux/amd64")
	echo "linux-amd64"
	;;
    "linux/arm64")
	echo "linux-arm64"
	;;
    "linux/arm/v7")
	echo "linux-armv7"
	;;
    "linux/i386")
  echo "linux-i386"
  ;;
    "darwin/amd64")
  echo "darwin-amd64"
  ;;
esac