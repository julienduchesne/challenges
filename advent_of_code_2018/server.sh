#!/usr/bin/env bash

# Constants
HTTP_200="HTTP/1.1 200 OK"
HTTP_400="HTTP/1.1 400 Bad Request"
HTTP_404="HTTP/1.1 404 Not Found"
HTTP_LOCATION="Location:"

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Create output file
OUTFILE="${SCRIPT_DIR}/out"
rm -f "${OUTFILE}"
mkfifo "${OUTFILE}"
trap 'rm -f ${OUTFILE}' EXIT

function response {
    printf "%s\n%s %s\n\n%s\n" "$1" "$HTTP_LOCATION" "$REQUEST" "$2" > "${OUTFILE}"
}

function list {
  days="[]"
  for filename in "${SCRIPT_DIR}"/*.sh; do
    title_line=$(grep -e "^# Title: .*$" "$filename")
    desc_line=$(grep -e "^# Description: .*$" "$filename")
    if [ -n "$title_line" ]; then
      title=${title_line//# Title: /}
      description=""
      if [ -n "$desc_line" ]; then description=${desc_line//# Description: /}; fi
      days=$(jq  '. |= . + [{id: "'"${filename#"${SCRIPT_DIR}/"}"'", title: "'"$title"'", description: "'"${description}"'"}]' <<< "${days}")
    fi
  done
  echo "${days}"
}

while true
do
  # shellcheck disable=SC2002
  cat "${OUTFILE}" | nc -w1 -l "${CHALLENGES_AOC_2018_PORT:-8083}" > >( # parse the netcat output, to build the answer redirected to the pipe "out".
    while IFS= read -r  INPUT || [[ -n "$INPUT" ]]; do
      echo $INPUT | cat -v
      INPUT=$(echo "$INPUT" | tr -d '\r\n')
      if echo "$INPUT" | grep -qE '^(GET|POST) /'; then # if line starts with "GET / or POST /"
        METHOD=$(echo "$INPUT" | cut -d ' ' -f1) # extract the method
        REQUEST=$(echo "$INPUT" | cut -d ' ' -f2) # extract the request
      elif [ "$INPUT" = "" ]; then break; # empty line == end of GET
      fi
    done

      
    # List scripts
    if echo "$REQUEST" | grep -qE '^/list'
    then
        response "$HTTP_200" "$(list)"
    # Call scripts to solve problems
    elif echo "$REQUEST" | grep -qE '^/solve'
    then
        id=${REQUEST#"/solve/"}
        found=false
        for row in $(list | jq -r '.[] | @base64'); do
            _jq() {
              echo "${row}" | base64 --decode | jq -r "${1}"
            }

          if [ "$(_jq '.id')" == "${id}" ]; then
            found=true
          fi
        done

        if [ "$found" == "true" ]; then
          echo "Running $id"
          response "$HTTP_200" "$(echo "-6, +3, +8, +5, -6" | bash "$SCRIPT_DIR"/"$id" || "Failed")"
        else
          response "$HTTP_400" "$id not found"
        fi
    else
        response "$HTTP_404" "Resource $REQUEST NOT FOUND!"
    fi
  )
done