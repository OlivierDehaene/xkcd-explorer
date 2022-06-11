gen-client:
	mkdir xkcd_explorer/pb || true
	pip install grpcio-tools==1.44.0
	python -m grpc_tools.protoc -I../proto --python_out=xkcd_explorer/pb --grpc_python_out=xkcd_explorer/pb ../proto/xkcd_explorer.proto
	find xkcd_explorer/pb/ -type f -name "*.py" -print0 -exec sed -i -e 's/^\(import.*pb2\)/from . \1/g' {} \;
	touch xkcd_explorer/pb/__init__.py

install:
	make gen-client
	poetry install

install-m1:
	conda install grpcio==1.46.3 -y
	make gen-client
	poetry install