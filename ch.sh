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

for FEAT in "${FEATS[@]}" ; do
	echo "$FEAT..."
	cargo check --features "$FEAT"
done
