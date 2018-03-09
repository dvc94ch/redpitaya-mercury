IP=192.168.1.10
ARCH=armv7-unknown-linux-gnueabihf
BUILD=debug
EXAMPLES=
TARGET=./target/$ARCH/$BUILD/examples/print_hwid

scp $TARGET $IP:/opt/bluepitaya
