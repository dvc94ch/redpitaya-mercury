IP=192.168.1.10
ARCH=armv7-unknown-linux-gnueabihf
BUILD=debug
EXAMPLES=
BIN=la_tcp_server #print_hwid
TARGET=./target/$ARCH/$BUILD/examples/$BIN

scp $TARGET $IP:/opt/bluepitaya
