set -e

EXE=stats-winsafe.exe
TARGET_DIR=/d/Stuff/Core/rs/_target

T0=$(date +%s%N)

echo "Compiling $EXE..."
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-msvc

echo "Replacing old $EXE..."
mv -f "$TARGET_DIR/x86_64-pc-windows-msvc/release/$EXE" "/d/Stuff/apps/_audio tools/."

echo "Cleaning up..."
rm -rf "$TARGET_DIR/release"
rm -rf "$TARGET_DIR/x86_64-pc-windows-msvc"

print_elapsed () {
	MIN=$(( ($1 - ($1 % (60 * 1000))) / (1000 * 60) ))
	SEC=$(( ($TF - ($MIN * 1000 * 60) - ($1 % 1000)) / 1000 ))
	MS=$(( $1 % 1000 ))

	PURP='\033[0;35m'
	NC='\033[0m'
	if (($MIN > 0)); then
		printf "${PURP}Duration${NC} %02d:%02d.%03d min\n" $MIN $SEC $MS
	else
		printf "${PURP}Duration${NC} %d.%03d sec\n" $SEC $MS
	fi
}
TF=$((($(date +%s%N) - $T0)/1000000))
print_elapsed $TF
