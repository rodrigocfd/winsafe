# Runs cargo check on each feature.

FEATS=(
	comctl
	comdlg
	dshow
	dwm
	dxgi
	gdi
	gui
	kernel
	mf
	msimg
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
