#[cfg(all(feature = "kext", feature = "dext"))]
compile_error!(
    "features kext and dext are mutually exclusive, consider using --no-default-features"
);

#[cfg(not(any(feature = "kext", feature = "dext")))]
compile_error!("select at one feature from kext and dext");

fn main() {
    let mut build = cc::Build::new();

    build
        .file("c_src/driverkit.cpp")
        .cpp(true)
        .std("c++2a")
        .flag("-w")
        .shared_flag(true)
        .flag("-fPIC");

    #[cfg(feature = "dext")]
    {
        build
            .include("c_src/Karabiner-DriverKit-VirtualHIDDevice/include/pqrs/karabiner/driverkit");
        build.include("c_src/Karabiner-DriverKit-VirtualHIDDevice/src/Client/vendor/include");
    }

    #[cfg(feature = "kext")]
    {
        build.flag("-D");
        build.flag("USE_KEXT");
        build.include("c_src/Karabiner-VirtualHIDDevice/dist/include");
    }

    build.compile("driverkit");

    println!("cargo:rerun-if-changed=c_src/c_src/driverkit.hpp");
    println!("cargo:rerun-if-changed=c_src/c_src/driverkit.cpp");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}
