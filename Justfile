run release="false" day="all" input=".input":
    cargo run {{if release == "false" { "" } else { "--release" } }} -- {{input}} {{day}}

make-test day part test input=".input":
    mkdir -p "{{input}}/{{day}}/{{part}}/tests/{{test}}/"
    read -d $'\004' -p $'In:\n' x ; echo "$x" > "{{input}}/{{day}}/{{part}}/tests/{{test}}/in.txt"
    read -d $'\004' -p $'Out:\n' y ; echo "$y" > "{{input}}/{{day}}/{{part}}/tests/{{test}}/out.txt"

make-input day input=".input":
    mkdir -p "{{input}}/{{day}}/A/"
    mkdir -p "{{input}}/{{day}}/B/"
    read -d $'\004' -p $'In:\n' x ; echo "$x" > "{{input}}/{{day}}/A/in.txt"
    ln "{{input}}/{{day}}/A/in.txt" "{{input}}/{{day}}/B/in.txt"

