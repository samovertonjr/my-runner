#!/bin/bash

printf "This is a simple script for installing the nix\n"
printf "package manager on ubuntu and most other linux\n"
printf "operating systems.\n"
printf "\nThis script uses the 'sudo' command and must be\n"
printf "executed by a priveleged user.\n"
printf "\nIf the privileged user is protected by a password,\n"
printf "there execution will require the user to enter a\n"
printf "password.\n"

mkdir -p "${HOME}/.config/nix"

echo "experimental-features = nix-command flakes" >> "${HOME}/.config/nix/nix.conf"

sudo mkdir /nix

sudo chown "${USER}":"${USER}" /nix

curl -L https://nixos.org/nix/install | sh

echo '# shellcheck disable=SC1091' >> "${HOME}/.bashrc"

# shellcheck disable=SC2016
echo 'source "${HOME}/.nix-profile/etc/profile.d/nix.sh"' >> "${HOME}/.bashrc"

# shellcheck disable=SC1091
source "${HOME}/.bashrc"
