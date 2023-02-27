.PHONY: prepare
prepare:
	@pip install -r load-balancing/requirements.txt

.PHONY: start svc-assets
start:
	@locust -f load-balancing/assets.py --host=http://localhost:8004

.PHONY: start svc-cargo
start:
	@locust -f load-balancing/cargo.py --host=http://localhost:8000
