day=$(expr $1 + 0)
day_padded=$(printf "%02d" "$day")

if [ -z "$SESSION_TOKEN" ]; then
    source "$PWD/.env" || { echo "No .env file found in the current directory!"; exit 1; }
fi

if [ -z "$YEAR" ]; then
    YEAR="$(date +%Y)"
fi

curl -LsH "Cookie: session=$SESSION_TOKEN" -o "day-$day_padded/input.txt" "https://adventofcode.com/${YEAR}/day/${day}/input"