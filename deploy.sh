EXE=winsafe-stats.exe

echo "Compiling $EXE..."
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-msvc

echo "Replacing old $EXE..."
mv ./target/x86_64-pc-windows-msvc/release/$EXE /d/Stuff/apps/_audio\ tools/.

echo "Cleaning up..."
rm -rf ./target/release
rm -rf ./target/x86_64-pc-windows-msvc

echo "Done."
