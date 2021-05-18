#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
OUTFILE="${SCRIPT_DIR}/out"

function list {
  days="[]"
  for filename in ${SCRIPT_DIR}/*.sh; do
    content=$(cat "$filename")
    title_line=$(grep -e "^# Title: .*$" "$filename")
    desc_line=$(grep -e "^# Description: .*$" "$filename")
    if [ ! -z "$title_line" ]; then
      title=$(sed 's/# Title: //g' <<< "${title_line}")
      description=""
      if [ ! -z "$desc_line" ]; then description=$(sed 's/# Description: //g' <<< "${desc_line}"); fi
      days=$(jq  '. |= . + [{id: "'${filename#"${SCRIPT_DIR}/"}'", title: "'"$title"'", description: "'"${description}"'"}]' <<< "${days}")
    fi
  done
  echo "${days}"
}


rm -f ${OUTFILE}
mkfifo ${OUTFILE}
trap "rm -f ${OUTFILE}" EXIT
while true
do
  cat ${OUTFILE} | nc -w1 -l ${CHALLENGES_AOC_2018_PORT:-8083} > >( # parse the netcat output, to build the answer redirected to the pipe "out".
    export REQUEST=
    while read line
    do
      line=$(echo "$line" | tr -d '[\r\n]')
      if echo "$line" | grep -qE '^(GET|POST) /' # if line starts with "GET / or POST /"
      then
        REQUEST=$(echo "$line" | cut -d ' ' -f2) # extract the request
      elif [ "x$line" = x ] # empty line / end of request
      then
        HTTP_200="HTTP/1.1 200 OK"
        HTTP_400="HTTP/1.1 400 Bad Request"
        HTTP_404="HTTP/1.1 404 Not Found"
        HTTP_LOCATION="Location:"

        function response {
            printf "%s\n%s %s\n\n%s\n" "$1" "$HTTP_LOCATION" $REQUEST "$2" > ${OUTFILE}
        }

        # List scripts
        if echo $REQUEST | grep -qE '^/list'
        then
            response "$HTTP_200" "$(list)"
        # Call scripts to solve problems
        elif echo $REQUEST | grep -qE '^/solve'
        then
            id=${REQUEST#"/solve/"}
            found=false
            for row in $(echo "$(list)" | jq -r '.[] | @base64'); do
                _jq() {
                  echo ${row} | base64 --decode | jq -r ${1}
                }

              if [ "$(_jq '.id')" == "${id}" ]; then
                found=true
              fi
            done

            if [ "$found" == "true" ]; then
              echo "Running $id"
              response "$HTTP_200" "$(bash $SCRIPT_DIR/$id)"
            else
              response "$HTTP_400" "$id not found"
            fi

        else
            response "$HTTP_404" "Resource $REQUEST NOT FOUND!"
        fi
      fi
    done
  )
done