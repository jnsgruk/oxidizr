#!/usr/bin/env bash

ensure_sudors_installed() {
    apt list sudo-rs | MATCH installed

    ls -la /usr/bin/sudo | MATCH "/usr/bin/sudo -> /usr/lib/cargo/bin/sudo"
    ls -la /usr/bin | MATCH ".sudo.oxidizr.bak"
    /usr/bin/sudo --version 2>&1 | MATCH "sudo-rs"

    ls -la /usr/bin/su | MATCH "/usr/bin/su -> /usr/lib/cargo/bin/su"
    ls -la /usr/bin | MATCH ".su.oxidizr.bak"
    /usr/bin/su --version 2>&1 | MATCH "su-rs"

    ls -la /usr/sbin/visudo | MATCH "/usr/sbin/visudo -> /usr/lib/cargo/bin/visudo"
    ls -la /usr/sbin | MATCH ".visudo.oxidizr.bak"
}

ensure_sudors_absent() {
    apt list sudo-rs | NOMATCH installed

    ls -la /usr/bin/sudo | NOMATCH "/usr/bin/sudo -> /usr/lib/cargo/bin/sudo"
    ls -la /usr/bin | NOMATCH ".sudo.oxidizr.bak"
    /usr/bin/sudo --version 2>&1 | NOMATCH "sudo-rs"

    ls -la /usr/bin/su | NOMATCH "/usr/bin/su -> /usr/lib/cargo/bin/su"
    ls -la /usr/bin | NOMATCH ".su.oxidizr.bak"
    /usr/bin/su --version 2>&1 | NOMATCH "su-rs"

    ls -la /usr/sbin/visudo | NOMATCH "/usr/sbin/visudo -> /usr/lib/cargo/bin/visudo"
    ls -la /usr/sbin | NOMATCH ".visudo.oxidizr.bak"
}
