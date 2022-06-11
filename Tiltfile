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
)

demo_port_forward = port_forward(local_port=8080, container_port=80, name="demo")

k8s_resource(
    "xkcd-explorer-gradio-demo",
    labels="xkcd-explorer",
    port_forwards=demo_port_forward,
    resource_deps=["xkcd-explorer"],
)
