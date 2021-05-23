#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

HTTP_200="HTTP/1.1 200 OK"
HTTP_400="HTTP/1.1 400 Bad Request"
HTTP_404="HTTP/1.1 404 Not Found"
HTTP_LOCATION="Location:"

function response {
    printf "%s\n%s %s\n\n%s\n" "$1" "$HTTP_LOCATION" "$REQUEST" "$2"
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

export LENGTH=0
export START_BODY="0"
export BODY=""

while IFS= read -r INPUT || [[ -n "$INPUT" ]]; do
    TRIM_INPUT=$(echo "$INPUT" | tr -d '\r\n');
    if echo "$INPUT" | grep -qE '^(GET|POST) /'; then # if line starts with "GET / or POST /"
        METHOD=$(echo "$TRIM_INPUT" | cut -d ' ' -f1); # extract the method
        REQUEST=$(echo "$TRIM_INPUT" | cut -d ' ' -f2); # extract the request
    elif echo "$INPUT" | tr '[:upper:]' '[:lower:]' | grep -qE '^content-length:'; then
        LENGTH=$(($(echo "$TRIM_INPUT" | cut -d ':' -f2)+0)) # extract the content length
    elif [ "$TRIM_INPUT" = "" ]; then
        START_BODY="1";
        break;
    fi
done

echo -e "$(date '+%Y-%m-%dT%H:%M:%S.%3NZ') [AoC 2018] ${METHOD} ${REQUEST}" 1>&2

# Handle POST data
if [ "$START_BODY" = "1" ] && [ "$LENGTH" -gt 0 ]; then
    while IFS= read -r -n1 -d '' INPUT; do
        BODY="${BODY}${INPUT}"
        LENGTH=$((LENGTH-1))

        if [ "$LENGTH" -le 0 ]; then
            break;
        fi
    done
fi

# List scripts
if echo "$REQUEST" | grep -qE '^/list'; then
    response "$HTTP_200" "$(list)"
# Call scripts to solve problems
elif echo "$REQUEST" | grep -qE '^/solve'; then
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
        echo -e "Running $id" 1>&2
        response "$HTTP_200" "$(printf '%s' "$BODY" | bash "$SCRIPT_DIR"/"$id" || "Failed")"
    else
        response "$HTTP_400" "$id not found"
    fi
else
    response "$HTTP_404" "Resource $REQUEST NOT FOUND!"
fi

