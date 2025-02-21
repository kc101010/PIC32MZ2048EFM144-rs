svd2rust --target=mips -i PIC32MZ2048EFM144.svd

cp device.x build.rs pic32mz2048efm144_pac/
cp lib.rs pic32mz2048efm144_pac/src/lib.rs