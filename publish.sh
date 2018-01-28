#!/bin/bash

set -e

publish() {
    cargo publish --manifest-path code-gen/Cargo.toml
    cargo publish --manifest-path helper/Cargo.toml
}

read -r -p "Are you sure? " response
case "$response" in
    [yY][eE][sS])
        publish
        ;;
    *)
        echo "ok then"
        ;;
esac
