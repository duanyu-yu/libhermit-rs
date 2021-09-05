#!/bin/zsh

sudo qemu-system-x86_64 -M microvm,x-option-roms=off,pit=off,pic=off,rtc=off \
        -global virtio-mmio.force-legacy=on \
        -nodefaults \
        -no-user-config \
        -display none \
        -smp 1 \
        -m 64M \
        -serial stdio \
        -kernel ~/RWTH/Bachelorarbeit/loader/target/x86_64-unknown-hermit-loader/debug/rusty-loader \
        -initrd ~/RWTH/Bachelorarbeit/rusty-hermit/target/x86_64-unknown-hermit/debug/hello_world \
        -enable-kvm \
        -cpu host \
        -netdev tap,id=tap10,script=no,downscript=no,vhost=off  \
        -device virtio-net-device,netdev=tap10 \
        
        # -netdev tap,id=net0,ifname=tap10 \
        # -device virtio-net-device,netdev=net0 \
        
   
