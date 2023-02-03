# Runs cargo check on each feature.

FEATS=(
	advapi
	comctl
	comdlg
	dshow
	gdi
	gui
	kernel
	ktm
	msimg
	ole
	oleaut
	shell
	user
	uxtheme
	version
)

set -e

for FEAT in "${FEATS[@]}" ; do
	echo "$FEAT..."
	cargo check --features "$FEAT"
done
