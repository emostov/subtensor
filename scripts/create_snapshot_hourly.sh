#!/bin/bash

TAR_SRC=/mnt/chains/nakamoto_mainnet/db
SNAPSHOT_FILENAME=snapshot_$(date +"%m-%d-%Y_%H-%M-%S")
TAR_TARGET=/root/snapshots_hourly/$SNAPSHOT_FILENAME.tar.gz

# Let's delete the oldest snapshot in here first.
# we only need to maintain 10 days at a time.
cd /root/snapshots_hourly
ls -1t | tail -n +25 | xargs rm
cd ~

# Stop subtensor and start the DB export
systemctl stop subtensor
cd $TAR_SRC
tar -zcvf $TAR_TARGET *
cd ~
systemctl start subtensor

# Add to ipfs
NEW_PIN=`ipfs add -Q -r /root/snapshots_hourly`
ipfs name publish /ipfs/$NEW_PIN

# Now let's build the new docker image
docker build -t subtensor . --platform linux/x86_64 --build-arg SNAPSHOT_DIR="snapshots_hourly" --build-arg SNAPSHOT_FILE=$SNAPSHOT_FILENAME  -f subtensor/Dockerfile

# Tag this new image with the latest
docker tag subtensor opentensor/subtensor:latest
docker tag subtensor opentensor/subtensor:$SNAPSHOT_FILENAME

# now let's push this sum' bitch to dockerhub
docker push opentensor/subtensor:latest
docker push opentensor/subtensor:$SNAPSHOT_FILENAME