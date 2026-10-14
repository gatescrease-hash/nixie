#!/usr/bin/env bash
# ADR-0012: no literal epsilons. Fails on any non-zero float literal with
# magnitude below 1e-3 in Rust sources outside crates/nixie-units. Use a
# named tolerance from `nixie_units::tol` instead.
#
# Usage: scripts/lint-epsilon.sh [file.rs ...]
# With no arguments, checks every tracked or untracked-but-not-ignored .rs file.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

if [ "$#" -gt 0 ]; then
    files=("$@")
else
    mapfile -t files < <(git ls-files --cached --others --exclude-standard -- '*.rs')
fi

checked=()
for f in "${files[@]}"; do
    case "$f" in
        crates/nixie-units/*) ;;
        *) [ -f "$f" ] && checked+=("$f") ;;
    esac
done

[ "${#checked[@]}" -eq 0 ] && exit 0

perl -ne '
    BEGIN { $bad = 0 }
    my $line = $_;
    $line =~ s{//.*$}{};    # drop line comments, doc comments included
    while ($line =~ /(?<!\w)(\d[\d_]*(?:\.[\d_]*)?(?:[eE][+-]?[\d_]+)?)(?:_?f(?:32|64))?(?![\w])/g) {
        (my $lit = $1) =~ tr/_//d;
        my $v = $lit + 0;
        if ($v != 0 && abs($v) < 1e-3) {
            chomp(my $src = $_);
            print "$ARGV:$.: literal epsilon `$&`; use a named tolerance from nixie_units::tol (ADR-0012)\n    $src\n";
            $bad = 1;
        }
    }
    close ARGV if eof;    # reset $. per file
    END { exit $bad }
' "${checked[@]}"
