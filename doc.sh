#
# Generates WinSafe docs locally.
# If -r flag is passed, moves the generated files to $GH_PAGES and pushes it to the remote.
#

set -e
T0=$(date +%s%N) # start time

GREN='\033[0;32m'
BLUE='\033[0;34m'
PURP='\033[0;35m'
NC='\033[0m'

LOCAL_DOC=../_target/doc     # generated docs folder
GH_PAGES=../_winsafe-gh-pages # gh-pages repo folder

# Prints the time passed as the first argument $1.
print_duration () {
	MIN=$(( ($1 - ($1 % (60 * 1000))) / (1000 * 60) ))
	SEC=$(( ($1 - ($MIN * 1000 * 60) - ($1 % 1000)) / 1000 ))
	MS=$(( $1 % 1000 ))

	if (($MIN > 0)); then
		printf "${PURP}Duration${NC} %02d:%02d.%03d min\n" $MIN $SEC $MS
	else
		printf "${PURP}Duration${NC} %d.%03d sec\n" $SEC $MS
	fi
}

echo -e "${BLUE}Cleaning local doc folder...${NC}"
if [ -d $LOCAL_DOC ]; then
	cd $LOCAL_DOC
	rm -rf ./*
	cd -
fi

# Since doc_cfg is broken, we're using nightly-2025-09-27 because it's the last one supporting doc_auto_cfg.
# In lib.rs we'll insert doc_auto_cfg line, run rust doc, then remove the line.
# https://users.rust-lang.org/t/doc-auto-cfg-is-gone-what-am-i-supposed-to-do/135070
echo -e "${BLUE}Generating docs locally...${NC}"
sed -b -i '3i#![cfg_attr(docsrs, feature(doc_auto_cfg))]' src/lib.rs # insert line, 1-based
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly-2025-09-27 doc --all-features
sed -b -i '3d' src/lib.rs # remove line, 1-based

# Check the existence of -r flag, if absent, we stop here.
if [[ ! " $@ " =~ " -r " ]]; then
	echo -e "${BLUE}To push the docs to gh-pages, pass -r flag.${NC}"
	TF=$((($(date +%s%N) - $T0)/1000000)) # end time
	print_duration $TF
	exit 0
fi

echo -e "${BLUE}Removing previous gh-pages HTML files...${NC}"
cd $GH_PAGES
rm -rf ./*
cd -

echo -e "${BLUE}Moving generated HTML files to gh-pages...${NC}"
mv $LOCAL_DOC/* $GH_PAGES/.

echo -e "${BLUE}Performing git add...${NC}"
cd $GH_PAGES
git add .

echo -e "${BLUE}Committing changes...${NC}"
DEPLOY_TIME=$(date '+%Y-%m-%d %H:%M:%S')
DEPLOY_MSG="Cargo doc auto deployment $DEPLOY_TIME."
git commit -m "$DEPLOY_MSG" || {
	echo -e "${GREN}No changes since last publish.${NC}"
	TF=$((($(date +%s%N) - $T0)/1000000)) # end time
	print_duration $TF
	exit 0
}

echo -e "${BLUE}Squashing...${NC}"
git reset $(git commit-tree "HEAD^{tree}" -m "$DEPLOY_MSG") # squash into 1 commit

echo -e "${BLUE}Cleaning up...${NC}"
git fetch origin --prune
git -c gc.reflogExpire=0 -c gc.reflogExpireUnreachable=0 -c gc.rerereresolved=0 \
	-c gc.rerereunresolved=0 -c gc.pruneExpire=now gc # https://stackoverflow.com/a/14729486/6923555

echo -e "${BLUE}Pushing changes to remote...${NC}"
git push origin +gh-pages

echo -e "${BLUE}Updating local repo...${NC}"
cd -
git pull

echo -e "${GREN}$DEPLOY_MSG${NC}"
TF=$((($(date +%s%N) - $T0)/1000000)) # end time
print_duration $TF
