# Runs cargo check on each feature.

FEATS=(
	comctl
	dshow
	dwm
	dxgi
	gdi
	gui
	kernel
	mf
	ole
	oleaut
	shell
	taskschd
	user
	uxtheme
	version
)

set -e

print_elapsed () {
	MIN=$(( ($1 - ($1 % (60 * 1000))) / (1000 * 60) ))
	SEC=$(( ($TF - ($MIN * 1000 * 60) - ($1 % 1000)) / 1000 ))
	MS=$(( $1 % 1000 ))

	PURP='\033[0;35m'
	NC='\033[0m'
	if (($MIN > 0)); then
		printf "    ${PURP}Duration${NC} %02d:%02d.%03d min\n" $MIN $SEC $MS
	else
		printf "    ${PURP}Duration${NC} %d.%03d sec\n" $SEC $MS
	fi
}

for FEAT in "${FEATS[@]}" ; do
	echo "$FEAT..."
	T0=$(date +%s%N)

	cargo check --features "$FEAT"

	TF=$((($(date +%s%N) - $T0)/1000000))
	print_elapsed $TF
done
