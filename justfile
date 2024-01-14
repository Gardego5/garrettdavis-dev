watch:
    export ALERTS_TO_EMAIL=me@garrettdavis.dev
    cargo lambda watch

build:
    nix build

css:
    tailwindcss \
        -o {{justfile_directory()}}/static/css/tailwind.css \
        -i base.css \
        -m

tf action: build css
    terraform -chdir={{justfile_directory()}}/infra {{action}}

