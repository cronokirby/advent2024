run release="false" day="all" input=".input":
    cargo run {{if release == "false" { "" } else { "--release" } }} -- {{input}} {{day}}
