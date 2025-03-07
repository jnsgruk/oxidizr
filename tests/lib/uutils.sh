#!/usr/bin/env bash

ensure_coreutils_installed() {
    apt list rust-coreutils | MATCH installed
    while read p; do
        util_path="$(which $p)"
        util_dir="$(dirname "$util_path")"
        ls -la "$util_path"| MATCH "$util_path -> /usr/bin/coreutils"
        ls -la "$util_dir/.$p.oxidizr.bak" || echo "No backup file for $util_path"
        $util_path --help | NOMATCH "https://www.gnu.org/software/coreutils"
    done < ${SPREAD_PATH}/tests/lib/rust-coreutils-bins.txt
}

ensure_coreutils_absent() {
    apt list rust-coreutils | NOMATCH installed
    ls -la /usr/bin/date | NOMATCH "/usr/bin/date -> /usr/bin/coreutils"
    ls -la /usr/bin | NOMATCH ".date.oxidizr.bak"
    date --help | MATCH "GNU"
}

ensure_findutils_installed() {
    apt list rust-findutils | MATCH installed

    ls -la "/usr/bin/find"| MATCH "/usr/bin/find -> /usr/lib/cargo/bin/findutils/find"
    ls -la "/usr/bin/.find.oxidizr.bak" || echo "No backup file for 'find'"
    find --help | NOMATCH "https://www.gnu.org/software/coreutils"
    
    ls -la "/usr/bin/xargs"| MATCH "/usr/bin/xargs -> /usr/lib/cargo/bin/findutils/xargs"
    ls -la "/usr/bin/.xargs.oxidizr.bak" || echo "No backup file for 'xargs'"
    xargs --help | NOMATCH "https://www.gnu.org/software/coreutils"
}

ensure_findutils_absent() {
    apt list rust-findutils | NOMATCH installed
    ls -la /usr/bin/find | NOMATCH "/usr/bin/find -> /usr/lib/cargo/bin/findutils/find"
    ls -la /usr/bin | NOMATCH ".find.oxidizr.bak"
    find --help | MATCH "GNU"
    
    ls -la /usr/bin/xargs | NOMATCH "/usr/bin/xargs -> /usr/lib/cargo/bin/findutils/xargs"
    ls -la /usr/bin | NOMATCH ".xargs.oxidizr.bak"
    xargs --help | MATCH "GNU"
}

ensure_diffutils_installed() {
    if [[ "$(lsb_release -rs)" != "24.04" ]]; then
        apt list rust-diffutils | MATCH installed
        ls -la "/usr/bin/diff"| MATCH "/usr/bin/diff -> /usr/lib/cargo/bin/diffutils/diff"
        ls -la "/usr/bin/.diff.oxidizr.bak" || echo "No backup file for /usr/bin/diff"
        /usr/bin/diff --help | NOMATCH "https://www.gnu.org/software/coreutils"
    fi
}

ensure_diffutils_absent() {
    apt list rust-diffutils | NOMATCH installed
    ls -la /usr/bin/find | NOMATCH "/usr/bin/find -> /usr/lib/cargo/bin/diffutils/find"
    ls -la /usr/bin | NOMATCH ".find.oxidizr.bak"
    find --help | MATCH "GNU"
}
