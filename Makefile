install: ## Install Repository dependencies
	brew install tilt-dev/tap/ctlptl kind kubectl helm derailed/k9s/k9s
	curl -fsSL https://raw.githubusercontent.com/tilt-dev/tilt/master/scripts/install.sh | bash

cluster-create: ## Create a Kind Kubernetes cluster
	ctlptl create cluster kind --registry=ctlptl-registry

cluster-delete: ## Delete a Kind Kubernetes cluster
	kind delete cluster
	docker stop ctlptl-registry && docker rm ctlptl-registry

k9s: ## Launch k9s
	kubectl config use-context kind-kind && k9s

up: ## Run `tilt up` using local Kind cluster
	kubectl config use-context kind-kind && tilt up

up-local: ## Run `tilt up` using local resources
	tilt up -- --local

down: ## Destroy tilt resources from local Kind cluster
	kubectl config use-context kind-kind && tilt down