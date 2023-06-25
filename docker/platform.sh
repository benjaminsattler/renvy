#!/usr/bin/env sh

case $TARGETPLATFORM in

    "linux/amd64")
	echo "x86_64-unknown-linux-gnu"
	;;
    "linux/arm64")
	echo "aarch64-unknown-linux-gnu"
	;;
    "linux/arm/v7")
	echo "armv7-unknown-linux-gnueabihf"
	;;
    "linux/i386")
  echo "i386-unknown-linux-gnu"
  ;;
    "darwin/amd64")
  echo "x86_64-apple-darwin"
  ;;
esac