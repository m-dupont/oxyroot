

publish:
    git push gitlab master --tags
    git push github master --tags
    cargo publish --allow-dirty -p oxyroot
    cargo publish --allow-dirty -p oxyroot_derive
    cargo publish --allow-dirty -p oxyroot-dump
    cargo publish --allow-dirty -p oxyroot-ls