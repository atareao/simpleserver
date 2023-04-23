user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`

build:
    echo {{version}}
    echo {{name}}
    podman build -t {{user}}/{{name}}:{{version}} \
                 -t {{user}}/{{name}}:latest \
                 .

push:
    # docker push {{user}}/{{name}} --all-tags
    podman push {{user}}/{{name}}:{{version}}
    podman push {{user}}/{{name}}:latest

run:
    podman run --rm \
               --init \
               --name aopodcast \
               --volume $PWD/config.yml:/app/config.yml \
               --volume $PWD/data:/app/data \
               --volume $PWD/assets:/app/assets \
               --volume $PWD/templates:/app/templates \
               --volume $PWD/posts:/app/posts \
               --volume $PWD/pages:/app/pages \
               {{user}}/{{name}}
