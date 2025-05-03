default:
    just --list

test-all:
    cargo +stable vet --locked
    cargo +stable deny --all-features --locked check
    cargo +stable fmt
    cargo +stable build --locked
    cargo +stable clippy --locked
    cargo +stable test --locked

_dist:
    rm -rf dist
    mkdir -p dist

# Build and sign a reproducible archive of cargo vendor sources
_vendor: _dist
    rm -rf vendor/
    cargo vendor --locked
    echo SOURCE_DATE_EPOCH="$(env LC_ALL=C TZ=UTC0 git show --quiet --date='format-local:%Y-%m-%dT%H:%M:%SZ' --format="%cd" HEAD)"
    # See https://reproducible-builds.org/docs/archives/
    env LC_ALL=C TZ=UTC0 tar --numeric-owner --owner 0 --group 0 \
        --sort name --mode='go+u,go-w' --format=posix \
        --pax-option=exthdr.name=%d/PaxHeaders/%f \
        --pax-option=delete=atime,delete=ctime \
        --mtime="$(env LC_ALL=C TZ=UTC0 git show --quiet --date='format-local:%Y-%m-%dT%H:%M:%SZ' --format="%cd" HEAD)" \
        -c -f "dist/git-gone-$(git describe)-vendor.tar.zst" \
        --zstd vendor

# Build and sign a reproducible git archive bundle
_git-archive: _dist
    env LC_ALL=C TZ=UTC0 git archive --format tar \
        --prefix "git-gone-$(git describe)/" \
        --output "dist/git-gone-$(git describe).tar" HEAD
    zstd --rm "dist/git-gone-$(git describe).tar"

package: _git-archive _vendor
    curl https://codeberg.org/swsnr.keys > dist/key
    ssh-keygen -Y sign -f dist/key -n file "dist/git-gone-$(git describe)-vendor.tar.zst"
    ssh-keygen -Y sign -f dist/key -n file "dist/git-gone-$(git describe).tar.zst"
    rm -f dist/key

release *ARGS: test-all
    cargo release {{ARGS}}
    just package
    echo "Upload dist/ contents to codeberg release now"
