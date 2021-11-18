#!/bin/bash
TAR_TARGET=/root/snapshots/snapshot_$(date +"%m-%d-%Y").tar.gz
TAR_SRC=/mnt/chains/nakamoto_mainnet/db

# Let's delete the oldest snapshot in here first.
# we only need to maintain 10 days at a time.
cd /root/snapshots
ls -1t | tail -n +11 | xargs rm
cd ~

# Stop subtensor and start the DB export
systemctl stop subtensor
cd $TAR_SRC
tar -zcvf $TAR_TARGET *
cd ~
systemctl start subtensor

# Add to ipfs
NEW_PIN=`ipfs add -Q -r /root/snapshots`
ipfs name publish /ipfs/$NEW_PIN

# Now let's build the new docker image
docker build -t subtensor . --platform linux/x86_64 --build-arg SNAPSHOT_DIR="snapshots" --build-arg SNAPSHOT_FILE="snapshot_11-18-2021.tar.gz"  -f subtensor/Dockerfile

# Tag this new image with the latest
docker tag subtensor opentensor/subtensor:latest

# now let's push this sum' bitch to dockerhub
docker push opentensor/subtensor:latest