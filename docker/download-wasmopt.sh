#!/usr/bin/env bash

set -e

RELEASE=version_119
REPO_OWNER=WebAssembly
REPO=binaryen
EXECUTABLE=wasm-opt

# Define variables.
if [[ "$OSTYPE" == "linux-gnu" ]]; then
  TAR="${REPO}-${RELEASE}-x86_64-linux.tar.gz"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  TAR="${REPO}-${RELEASE}-arm64-macos.tar.gz"
else
  echo "Only \"MacOS\" and \"Linux\" are supported - not \"$OSTYPE\""
  exit 1;
fi

echo $TAR

TAG="$RELEASE"
EXECUTABLE_DIR=$(pwd)/.$EXECUTABLE

GH_API="https://api.github.com"
GH_REPO="$GH_API/repos/${REPO_OWNER}/${REPO}"
GH_TAGS="$GH_REPO/releases/tags/$TAG"

# create dir
mkdir -p $EXECUTABLE_DIR
cd $EXECUTABLE_DIR

# Download release
echo ""
echo "Downloading $EXECUTABLE release \"$TAG\""
echo ""

# Read asset tags.
response=$(curl -s $GH_TAGS)

# Get ID of the asset based on given name.
eval $(echo "$response" | grep -C3 "name.:.\+$TAR" | grep -w id | tr : = | tr -cd '[[:alnum:]]=')
[ "$id" ] || { echo "Error: Failed to get asset id, response: $response" | awk 'length($0)<100' >&2; exit 1; }

wget --no-verbose --content-disposition --no-cookie -q --header "Accept: application/octet-stream" "$GH_REPO/releases/assets/$id" --show-progress

# unpack
tar -xf $TAR
rm -f $TAR

EXECUTABLE_PATH="$EXECUTABLE_DIR/${REPO}-${RELEASE}/bin/$EXECUTABLE"

# make executable
chmod +x $EXECUTABLE_PATH

echo ""
# smoke test executable installation
if ! [ -f $EXECUTABLE_PATH ]; then
  echo "$EXECUTABLE setup failed"
  exit 1
fi

echo "$EXECUTABLE download successful at $EXECUTABLE_PATH"
echo ""