fn main() {
    // Shadow esp-hal's rodata.x with our own version that places the ESP-IDF app
    // descriptor at the start of the DROM segment. espflash v4+ and the ESP-IDF
    // bootloader v5.2.3+ require this descriptor. esp-hal 0.23.x doesn't include
    // it (1.0+ does). Our OUT_DIR comes first in the linker search path, so this
    // rodata.x is found before esp-hal's when linkall.x does INCLUDE "rodata.x".
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(
        std::path::Path::new(&out_dir).join("rodata.x"),
        r#"
SECTIONS {
  .rodata : ALIGN(4)
  {
    . = ALIGN (4);
    _rodata_start = ABSOLUTE(.);
    KEEP(*(.flash.appdesc))
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
    . = ALIGN(4);
    _rodata_end = ABSOLUTE(.);
  } > RODATA

  .rodata.wifi : ALIGN(4)
  {
    . = ALIGN(4);
    *( .rodata_wlog_*.* )
    . = ALIGN(4);
  } > RODATA
}
"#,
    )
    .unwrap();

    println!("cargo:rustc-link-search={out_dir}");
    println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
}
