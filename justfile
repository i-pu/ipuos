build:
  cargo build
  cargo bootimage

qemu-x86_64:
  qemu-system-x86_64 -curses -drive format=raw,file=target/x86_64-ipuos/debug/bootimage-ipuos.bin -m 512

write-usb:
  dd if=target/x86_64-ipuos/debug/bootimage-ipuos.bin of=/dev/sdX && sync
