YEAR="$(date +%Y)"

source "$PWD/.env" || { echo "No .env file found in the current directory!"; exit 1; }

[ -z "$SESSION_TOKEN" ] && { echo "SESSION_TOKEN is not set in .env file!"; exit 1; }

current_day=$(find . -maxdepth 1 -type d -name "day-*" | sort | tail -1 | cut -c 3-)
current_day=${current_day/*-}
next_day="$(expr $current_day + 1 )"
next_day_padded=$(printf "%02d" "$next_day")

if [ "$current_day" == "" ]; then
    insert_after="lib"
else
    insert_after="day-$current_day"
fi

awk -F, '/"'"$insert_after"'",/ { print; print "    \"day-'"$next_day_padded"'\","; next }1' Cargo.toml | tee .tmp
mv .tmp Cargo.toml

cargo new --bin --vcs=none "day-$next_day_padded"
cargo add -p "day-$next_day_padded" lib --path="lib"

cat > "day-$next_day_padded/src/main.rs" << EOF
use lib::*;

fn main() {
    let input: String = lib::read_input!();
}

#[cfg(test)]
mod test {
    use super::*;
}
EOF

curl -LsH "Cookie: session=$SESSION_TOKEN" -o "day-$next_day_padded/input.txt" "https://adventofcode.com/${YEAR}/day/${next_day}/input"

touch "day-$next_day_padded/challenge.txt"
touch "day-$next_day_padded/test_input.txt"
