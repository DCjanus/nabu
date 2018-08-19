#!/bin/sh
until psql -h db -U nabu -c '\q'; do
    echo 'Waiting for PostGreSQL'
    sleep 1
done
export SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
export SSL_CERT_DIR=/etc/ssl/certs
exec ./nabu