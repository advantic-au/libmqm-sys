#!/bin/sh
set -e

MQC_REDIST_TAR=9.4.0.5-IBM-MQC-Redist-LinuxX64.tar.gz
MQC_REDIST_SHA256=5398c48756e80f1cbba86d17d75a55d18f9164bafa6b48eb7f501efc5243dfd9
TARGET=$1

mkdir -p $TARGET
cd $TARGET
curl --retry 10 --retry-connrefused --location --silent --show-error --fail -O https://public.dhe.ibm.com/ibmdl/export/pub/software/websphere/messaging/mqdev/redist/$MQC_REDIST_TAR
echo $MQC_REDIST_SHA256 $MQC_REDIST_TAR | sha256sum --check --status
tar zxf $MQC_REDIST_TAR
