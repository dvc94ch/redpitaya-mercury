IP=192.168.2.2
ARCH=armv7-unknown-linux-gnueabihf
BUILD=debug
EXAMPLES=
BIN=test_interrupts #la_tcp_server #print_hwid
TARGET=./target/$ARCH/$BUILD/examples/$BIN

scp $TARGET $IP:/opt/bluepitaya
