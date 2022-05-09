build:
  just clean
  cargo build
  cargo bootimage
  cargo rustc -- --emit=asm
  cargo rustc -- --emit=llvm-ir

qemu-x86_64:
  qemu-system-x86_64 -curses -drive format=raw,file=target/x86_64-ipuos/debug/bootimage-ipuos.bin -m 512

qemu-x86_64-m2048:
  qemu-system-x86_64 -curses -drive format=raw,file=target/x86_64-ipuos/debug/bootimage-ipuos.bin -m 2048

clean:
  fd -H 'ipuos' target/x86_64-ipuos/debug | xargs -I{} \rm -rf {}

write-usb:
  dd if=target/x86_64-ipuos/debug/bootimage-ipuos.bin of=/dev/sdX && sync
