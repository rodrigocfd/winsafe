# Generates the doc and deploys it to gh-pages branch,
# which must be in the $GHP directory.
# https://gist.github.com/rodrigocfd/3a0f3370817ec5c8c3d2ec6e516ae86b

T0=$(date +%s%N)

GHP=../gh-pages-winsafe # target gh-pages repo folder

echo '> Generating docs locally...'
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features

echo '> Removing previous HTML files...'
cd $GHP
git rm -r .

echo '> Moving generated HTML files...'
cd -
mv ./target/doc/* $GHP/.

echo '> Performing git add...'
cd -
git add .

echo '> Committing changes...'
dtt=$(date '+%Y-%m-%d %H:%M:%S')
git commit -m "Cargo doc auto deployment $dtt."

echo '> Pushing changes to remote...'
git push

echo '> Updating local repo...'
cd -
git pull

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
