#! /bin/bash

set -ex

rustc --version
cargo --version
cargo clippy --version

# Keep features in sync with run-cargo-test.sh
get_features() {
    crate=$1
    case "$crate" in
        gstreamer-audio|gstreamer-editing-services|gstreamer-gl|gstreamer-pbutils|gstreamer-rtp|gstreamer-rtsp|gstreamer-video|gstreamer)
            echo "--features=serde,v1_22"
            ;;
        gstreamer-validate)
            echo ""
            ;;
        *)
            echo "--features=v1_22"
            ;;
    esac
}

for crate in gstreamer* gstreamer-gl/{egl,wayland,x11}; do
    if [ -e "$crate/Cargo.toml" ]; then
        FEATURES=$(get_features "$crate")

        echo "Running clippy on $crate with $FEATURES"

        cargo clippy --locked --color=always --manifest-path "$crate/Cargo.toml" $FEATURES --all-targets -- $CLIPPY_LINTS
    fi
done

# And also run over all the examples/tutorials
cargo clippy --locked --color=always --manifest-path examples/Cargo.toml --all-targets --all-features -- $CLIPPY_LINTS
cargo clippy --locked --color=always --manifest-path tutorials/Cargo.toml --all-targets --all-features -- $CLIPPY_LINTS
