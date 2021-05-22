#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

tcpserver -DRH "0.0.0.0" "${CHALLENGES_AOC_2018_PORT:-8083}" "${SCRIPT_DIR}/server_internal.sh"