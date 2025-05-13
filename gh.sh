# Generates the doc and deploys it to gh-pages branch,
# which must be in the $GHP directory.
# https://gist.github.com/rodrigocfd/3a0f3370817ec5c8c3d2ec6e516ae86b

set -e

BLUE='\033[0;34m'
PURP='\033[0;35m'
NC='\033[0m'

T0=$(date +%s%N)

GHP=../_winsafe-gh-pages # target gh-pages repo folder

echo -e "${BLUE}Generating docs locally...${NC}"
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features

echo -e "${BLUE}Removing previous HTML files...${NC}"
cd $GHP
git rm -r .

echo -e "${BLUE}Moving generated HTML files...${NC}"
cd -
mv ./target/doc/* $GHP/.

echo -e "${BLUE}Performing git add...${NC}"
cd -
git add .

echo -e "${BLUE}Committing changes...${NC}"
dtt=$(date '+%Y-%m-%d %H:%M:%S')
git commit -m "Cargo doc auto deployment $dtt."

echo -e "${BLUE}Pushing changes to remote...${NC}"
git push

echo -e "${BLUE}Updating local repo...${NC}"
cd -
git pull

print_elapsed () {
	MIN=$(( ($1 - ($1 % (60 * 1000))) / (1000 * 60) ))
	SEC=$(( ($TF - ($MIN * 1000 * 60) - ($1 % 1000)) / 1000 ))
	MS=$(( $1 % 1000 ))

	if (($MIN > 0)); then
		printf "${PURP}Duration${NC} %02d:%02d.%03d min\n" $MIN $SEC $MS
	else
		printf "${PURP}Duration${NC} %d.%03d sec\n" $SEC $MS
	fi
}

TF=$((($(date +%s%N) - $T0)/1000000))
print_elapsed $TF
