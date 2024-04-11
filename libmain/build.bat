@echo off
cargo ndk -t arm64-v8a -o ./jniLibs build --release
IF "%1" NEQ "auto" (
    pause
)