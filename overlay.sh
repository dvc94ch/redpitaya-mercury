set -e

OVERLAYS=/sys/kernel/config/device-tree/overlays
MERCURY=/opt/redpitaya/fpga/mercury
BITSTREAM=$MERCURY/fpga.bit
DTBO=$MERCURY/fpga.dtbo

# Load bitstream
cat $BITSTREAM > /dev/xdevcfg
# Load dtbo
mkdir -p $OVERLAYS/mercury
cat $DTBO > $OVERLAYS/mercury/dtbo
