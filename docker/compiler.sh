#!/usr/bin/env sh

case $TARGETPLATFORM in

    "linux/amd64")
  echo "gcc"
	;;
    "linux/arm64")
	echo "gcc-aarch64-linux-gnu"
	;;
    "linux/arm/v7")
	echo "gcc-arm-linux-gnueabihf"
	;;
    "linux/i386")
  echo "i386-unknown-linux-gnu"
  ;;
    "darwin/amd64")
  echo "x86_64-apple-darwin"
  ;;
esac