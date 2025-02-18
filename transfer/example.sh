#!/bin/bash

export ALICE_AUTH="Authorization: Basic $(echo -n 'alice:wonderland' | base64)"
export BOB_AUTH="Authorization: Basic $(echo -n 'bob:sponge' | base64)"

curl -i -H "$ALICE_AUTH" 'http://localhost:8080/balance?currency=EUR'
curl -i -H "$BOB_AUTH" 'http://localhost:8080/balance?currency=EUR'

curl -i -XPOST -H "$ALICE_AUTH" 'http://localhost:8080/transfer?currency=EUR&recipient=bob&quantity=10'
