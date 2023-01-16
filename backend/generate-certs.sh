#!/bin/sh

if [ -z ${1} ]; then
  SERVER="localhost"
else
  SERVER="${1}"
fi;

# Generates a new SSL key and certificate bound to either localhost or a specified server and removes it's password
openssl req -x509 -new-key rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/CN=${SERVER}"
openssl rsa -in key.pem -out nopass.pem
rm key.pem
mv nopass.pem key.pem
