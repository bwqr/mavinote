export PATH=/usr/bin:$HOME/.cargo/bin:$PATH

RELFLAG=
if [[ "$CONFIGURATION" != "Debug" ]]; then
    RELFLAG=--release
fi

IS_SIMULATOR=0
if [ "${LLVM_TARGET_TRIPLE_SUFFIX-}" = "-simulator" ]; then
  IS_SIMULATOR=1
fi

for arch in $ARCHS; do
  case "$arch" in
    x86_64)
      if [ $IS_SIMULATOR -eq 0 ]; then
        echo "Building for x86_64, but not a simulator build. What's going on?" >&2
        exit 2
      fi

      # Intel iOS simulator
      env -i PATH=$PATH CARGO_TARGET_DIR=$CARGO_TARGET_DIR cargo build $RELFLAG --target x86_64-apple-ios --package ios
      ;;

    arm64)
      if [ $IS_SIMULATOR -eq 0 ]; then
        # Hardware iOS targets
        cargo build $RELFLAG --target aarch64-apple-ios --package ios
      else
        # M1 iOS simulator -- currently in Nightly only and requires to build `libstd`
        cargo build $RELFLAG --target aarch64-apple-ios-sim --package ios
      fi
  esac
done
