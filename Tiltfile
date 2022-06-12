config.define_bool("local")

cfg = config.parse()
local_exec = cfg.get("local", False)

if local_exec:
    local_resource(
        "xkcd-explorer-torchserve",
        serve_cmd="cd torchserve && make start",
        labels="xkcd-explorer",
    )

    local_resource(
        "xkcd-explorer",
        "cd server && cargo build",
        serve_cmd="./server/target/debug/xkcd-explorer-server",
        resource_deps=["xkcd-explorer-torchserve"],
        labels="xkcd-explorer",
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    local_resource(
        "xkcd-explorer-gradio-demo",
        serve_cmd="xkcd-explorer-demo gradio",
        serve_env={
            "GRADIO_SERVER_PORT": "7860"
        },
        labels="xkcd-explorer",
        resource_deps=["xkcd-explorer"],
        links=["http://127.0.0.1:7860"],
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    local_resource(
        "xkcd-explorer-demo-data-job",
        "xkcd-explorer-demo import-data",
        labels="xkcd-explorer",
        resource_deps=["xkcd-explorer"],
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

else:
    k8s_yaml(
        helm(
            "helm",
            name="xkcd-explorer",
        )
    )

    docker_build(
        "docker.io/xkcd-explorer/server",
        context=".",
        dockerfile="Dockerfile",
        only=["./server/", "./proto"],
    )

    docker_build(
        "docker.io/xkcd-explorer/torchserve",
        context=".",
        dockerfile="Dockerfile.torchserve",
        only=["./torchserve/"],
    )

    docker_build(
        "docker.io/xkcd-explorer/demo",
        context=".",
        dockerfile="Dockerfile.demo",
        only=["./demo/", "./client"],
    )


    k8s_resource(
        "xkcd-explorer-torchserve", labels="xkcd-explorer", port_forwards=["7070:7070"]
    )

    k8s_resource(
        "xkcd-explorer",
        labels="xkcd-explorer",
        port_forwards=["50051:50051"],
        resource_deps=["xkcd-explorer-torchserve"],
    )

    k8s_resource(
        "xkcd-explorer-demo-data-job",
        labels="xkcd-explorer",
        resource_deps=["xkcd-explorer"],
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    demo_port_forward = port_forward(local_port=8080, container_port=80, name="demo")

    k8s_resource(
        "xkcd-explorer-gradio-demo",
        labels="xkcd-explorer",
        port_forwards=demo_port_forward,
        resource_deps=["xkcd-explorer"],
    )
