#!/bin/bash
set -e

BASE_URL=https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/redist/
VERSION=$1
PLATFORM=$2
TARGET_PATH=$3
CHECKSUMS=$(dirname $0)/checksums.txt

case $PLATFORM in
  Linux)
    MQC_REDIST_ARCHIVE=${VERSION}-IBM-MQC-Redist-LinuxX64.tar.gz
    ;;
  Windows)
    MQC_REDIST_ARCHIVE=${VERSION}-IBM-MQC-Redist-Win64.zip
    ;;
  *)
    >&2 echo "Unsupported platform $PLATFORM"
    exit 1
    ;;
esac

grep $MQC_REDIST_ARCHIVE $CHECKSUMS || (>&2 echo "Unknown archive $MQC_REDIST_ARCHIVE"; exit 1)

curl -o /tmp/$MQC_REDIST_ARCHIVE --retry 10 --retry-connrefused --location --silent --show-error --fail ${BASE_URL}${MQC_REDIST_ARCHIVE}
grep $MQC_REDIST_ARCHIVE $CHECKSUMS | (cd /tmp; sha256sum --check --status)
mkdir -p $TARGET_PATH
case $PLATFORM in
  Linux)
    tar -zxf /tmp/$MQC_REDIST_ARCHIVE -C $TARGET_PATH
    ;;
  Windows)
    unzip -q -o /tmp/$MQC_REDIST_ARCHIVE -d $TARGET_PATH
    ;;
esac

rm /tmp/$MQC_REDIST_ARCHIVE
