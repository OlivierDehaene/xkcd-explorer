allow_k8s_contexts("rancher-desktop")

load("ext://helm_remote", "helm_remote")
load("ext://namespace", "namespace_inject")


def current_namespace():
    context_namespace = str(
        local("kubectl config view --minify --output 'jsonpath={..namespace}'")
    )
    if not context_namespace:
        context_namespace = "default"
    return context_namespace


def rancher():
    current_context = str(local("kubectl config current-context"))
    return current_context == "rancher-desktop"


def nerdctl_build(
    ref,
    context,
    dockerfile=None,
    target=None,
    only=None,
    ignore=None,
    extra_flags=None,
    **kwargs
):
    """Use nerdctl to build images for Tilt.
    Args:
      ref: The name of the image to build. Must match the image
        name in the Kubernetes resources you're deploying.
      context: The build context of the image to build. Expressed as a file path.
      dockerfile: The name of the Dockerfile
      target: build target
      only: build only if files are modified
      ignore: Changes to the given files or directories do not trigger rebuilds.
        Does not affect the build context.
      extra_flags: Extra flags to pass to nerdctl build. Expressed as an argv-style array.
      **kwargs: will be passed to the underlying `custom_build` call
    """
    deps = only or [context]
    extra_flags = extra_flags or []
    extra_flags_str = " ".join([shlex.quote(f) for f in extra_flags])
    if dockerfile:
        extra_flags_str += " -f %s" % dockerfile
    if target:
        extra_flags_str += " --target %s" % target

    custom_build(
        ref=ref,
        command="nerdctl build -t $EXPECTED_REF %s %s --namespace k8s.io\n"
        % (extra_flags_str, shlex.quote(context)),
        command_bat="nerdctl build -t %%EXPECTED_REF%% %s %s --namespace k8s.io\n"
        % (extra_flags_str, shlex.quote(context)),
        ignore=ignore,
        deps=deps,
        skips_local_docker=True,
        disable_push=True,
        **kwargs
    )


config.define_bool("local")
config.define_bool("small")
config.define_bool("test-integration")
config.define_string("image-tag")

cfg = config.parse()
local_exec = cfg.get("local", False)
small_dimensions = cfg.get("small", False)
run_integration_tests = cfg.get("test-integration", False)
image_tag = cfg.get("image-tag", "latest")
namespace = current_namespace()

if rancher():
    build_fn = nerdctl_build
else:
    build_fn = docker_build

# Gitlab Registry secret
k8s_yaml(
    local(
        "kubectl create secret docker-registry mati-ops-registry"
        + " --docker-server=https://registry.ops.mati.io"
        + " --docker-username=%s" % os.getenv("GITLAB_TOKEN_NAME")
        + " --docker-password=%s" % os.getenv("GITLAB_TOKEN")
        + " --docker-email=vision@mati.io"
        + " --dry-run='client' -o yaml"
        + " --namespace %s" % namespace
    )
)

# Vald resources
dimension = 128 if small_dimensions else 1024
helm_remote(
    "vald",
    repo_name="vald",
    namespace=namespace,
    repo_url="https://vald.vdaas.org/charts",
    values="server/vald/values.yaml",
    set=["agent.ngt.dimension=%s" % str(dimension)],
)

k8s_resource(
    "vald-agent-ngt",
    port_forwards=8081,
    labels="vald",
    trigger_mode=TRIGGER_MODE_MANUAL,
)

# RabbitMQ
helm_remote(
    "rabbitmq",
    repo_name="bitnami",
    repo_url="https://charts.bitnami.com/bitnami",
    namespace=namespace,
    set=[
        "persistence.enabled=false",
        "auth.username=guest",
        "auth.password=guest",
        "image.repository=rabbitmq",
        "image.tag=3.8.6-management-alpine",
    ],
)

k8s_resource(
    "rabbitmq",
    port_forwards=5672,
    labels="rabbitmq",
    trigger_mode=TRIGGER_MODE_MANUAL,
)


if local_exec:
    # Face retrieval resources
    local_resource(
        "face-retrieval-server",
        "cargo build",
        serve_cmd="target/debug/face-retrieval-server",
        serve_env={
            "AWS_ACCESS_KEY_ID": os.getenv("MEDIA_AWS_ACCESS_KEY_ID"),
            "AWS_SECRET_ACCESS_KEY": os.getenv("MEDIA_AWS_SECRET_ACCESS_KEY"),
            "PARAVISION_URL": os.getenv("PARAVISION_URL"),
            "VALD_URL": "http://localhost:8081",
            "PROMETHEUS_METRICS_ADDR": "0.0.0.0:9000",
            "RUST_LOG": "debug",
        },
        resource_deps=["vald-agent-ngt"],
        labels="face-retrieval",
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    local_resource(
        "face-retrieval-server-insert-consumer",
        "cargo build",
        serve_cmd="target/debug/face-retrieval-insert-consumer",
        serve_env={
            "AMQP_HOST": "localhost",
            "FACE_RETRIEVAL_URL": "http://localhost:50051",
            "PROMETHEUS_METRICS_ADDR": "0.0.0.0:9001",
            "RUST_LOG": "debug",
        },
        resource_deps=["vald-agent-ngt", "face-retrieval-server", "rabbitmq"],
        labels="face-retrieval",
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    local_resource(
        "face-retrieval-server-insert-router",
        "cargo build",
        serve_cmd="target/debug/face-retrieval-insert-routing",
        serve_env={
            "AMQP_HOST": "localhost",
            "PROMETHEUS_METRICS_ADDR": "0.0.0.0:9002",
            "RUST_LOG": "debug",
        },
        resource_deps=["vald-agent-ngt", "face-retrieval-server", "rabbitmq"],
        labels="face-retrieval",
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    local_resource(
        "face-retrieval-server-search-router",
        "cargo build",
        serve_cmd="target/debug/face-retrieval-search-routing",
        serve_env={
            "AMQP_HOST": "localhost",
            "FACE_RETRIEVAL_URL": "http://localhost:50051",
            "PROMETHEUS_METRICS_ADDR": "0.0.0.0:9003",
            "RUST_LOG": "debug",
        },
        resource_deps=["vald-agent-ngt", "face-retrieval-server", "rabbitmq"],
        labels="face-retrieval",
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    local_resource(
        "Insert data",
        "python data/main.py insert --data-path=data/subset.csv --server-url localhost:50051",
        resource_deps=["face-retrieval-server"],
        labels="data",
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    local_resource(
        "Remove data",
        "python data/main.py remove --data-path=data/subset.csv --server-url localhost:50051",
        resource_deps=["face-retrieval-server"],
        labels="data",
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    local_resource(
        "Streamlit Demo",
        "streamlit run demo/app.py",
        resource_deps=["face-retrieval-server"],
        labels="demo",
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )
else:
    # Face retrieval resources
    k8s_yaml(
        helm(
            "helm",
            namespace=namespace,
            set=[
                "image.tag=%s" % image_tag,
                "aws.accessKeyId=%s" % os.getenv("MEDIA_AWS_ACCESS_KEY_ID"),
                "aws.secretAccessKey=%s" % os.getenv("MEDIA_AWS_SECRET_ACCESS_KEY"),
                "server.paravisionUrl=%s" % os.getenv("PARAVISION_URL"),
                "server.valdUrl=http://vald-agent-ngt:8081",
                "amqp.host=rabbitmq",
                "rustLog=debug",
            ],
            name="face-retrieval-server",
        )
    )

    k8s_resource(
        "face-retrieval-server",
        labels="face-retrieval",
        resource_deps=["vald-agent-ngt"],
        port_forwards=["50051:50051", "9000:9000"],
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    k8s_resource(
        "face-retrieval-server-insert-consumer",
        labels="face-retrieval",
        resource_deps=["face-retrieval-server", "rabbitmq"],
        port_forwards="9001:9000",
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    k8s_resource(
        "face-retrieval-server-insert-router",
        labels="face-retrieval",
        resource_deps=["face-retrieval-server", "rabbitmq"],
        port_forwards="9002:9000",
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    k8s_resource(
        "face-retrieval-server-search-router",
        labels="face-retrieval",
        resource_deps=["face-retrieval-server", "rabbitmq"],
        port_forwards="9003:9000",
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    # Insert data
    k8s_yaml(namespace_inject(read_file("data/job-insert-local.yaml"), namespace))
    k8s_resource(
        "insert-data",
        labels="data",
        resource_deps=["face-retrieval-server"],
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    # Remove data
    k8s_yaml(namespace_inject(read_file("data/job-remove.yaml"), namespace))
    k8s_resource(
        "remove-data",
        labels="data",
        resource_deps=["face-retrieval-server"],
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

    # Streamlit Demo
    k8s_yaml(namespace_inject(read_file("demo/deployment-local.yaml"), namespace))
    k8s_resource(
        "face-retrieval-demo",
        labels="demo",
        resource_deps=["face-retrieval-server"],
        port_forwards=8501,
        auto_init=False,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

if local_exec or run_integration_tests:
    local_resource(
        "Integration Tests",
        "make test-integration",
        env={
            "VALD_URL": "http://localhost:8081",
            "RUST_LOG": "debug",
            "RUSTC_WRAPPER": os.getenv("RUSTC_WRAPPER", "")
        },
        resource_deps=[
            "face-retrieval-server",
            "face-retrieval-server-insert-router",
            "face-retrieval-server-search-router",
        ],
        labels="face-retrieval",
        auto_init=run_integration_tests,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )