#!/bin/bash

TAR_SRC=/root/.local/share/node-subtensor/chains/nakamoto_mainnet/db

currenthour=$(date +%H)
if [[ "$currenthour" == "00" ]]; then
        echo "[+] It's midnight, creating nightly snapshot"

        SNAPSHOT_FILENAME=snapshot_$(date +"%m-%d-%Y_%H")-00-nightly
        TAR_TARGET=/root/snapshots_nightly/$SNAPSHOT_FILENAME.tar.gz
        SNAPSHOT_DIR="snapshots_nightly"

        # Let's delete the oldest snapshot in here first.
        # we only need to maintain 10 days at a time.
        cd /root/$SNAPSHOT_DIR
        ls -1t | tail -n +11 | xargs rm
        cd ~
else
        echo "[+] Creating hourly snapshot"
        SNAPSHOT_FILENAME=snapshot_$(date +"%m-%d-%Y_%H")-00
        TAR_TARGET=/root/snapshots_hourly/$SNAPSHOT_FILENAME.tar.gz
        SNAPSHOT_DIR="snapshots_hourly"

        # Let's delete the oldest snapshot in here first.
        # we only need to maintain 10 days at a time.
        cd /root/$SNAPSHOT_DIR
        ls -1t | tail -n +25 | xargs rm
        cd ~
fi

echo "[+] Removing previous docker images from previous build"
# Kill dangling docker images from previous builds
/usr/bin/docker system prune -a -f

echo "[+] Stopping Subtensor and starting database export"
# Stop subtensor and start the DB export
/usr/local/bin/pm2 describe subtensor > /dev/null
RUNNING=$?

# If subtensor is running, then start the export process.
# NOTE: Export won't happen if chain is down, because it would very likely be out of date.
if [ "${RUNNING}" -eq 0 ]; then

        echo "[+] Stopping subtensor PM2 job"
        # Stop subtensor chain so we can export the db
        /usr/local/bin/pm2 stop subtensor
        cd $TAR_SRC
        tar -zcvf $TAR_TARGET *
        cd ~

        # Build docker image
        echo "[+] Building Docker image from directory ${SNAPSHOT_DIR} and snapshot file ${SNAPSHOT_FILENAME}"
        /usr/bin/docker build -t subtensor . --platform linux/x86_64 --build-arg SNAPSHOT_DIR=$SNAPSHOT_DIR --build-arg SNAPSHOT_FILE=$SNAPSHOT_FILENAME  -f /root/subtensor/Dockerfile

        # Tag new image with latest
        echo "[+] Tagging new image with latest tag"
        /usr/bin/docker tag subtensor opentensorfdn/subtensor:latest
        /usr/bin/docker tag subtensor opentensorfdn/subtensor:$SNAPSHOT_FILENAME

        # now let's push this sum' bitch to dockerhub
        echo "[+] Pushing Docker image to DockerHub"
        /usr/bin/docker push opentensorfdn/subtensor:latest
        /usr/bin/docker push opentensorfdn/subtensor:$SNAPSHOT_FILENAME

        # Start the chain again
        echo "[+] Restarting Subtensor chain"
        /usr/local/bin/pm2 start subtensor --watch
fi
echo "\n"