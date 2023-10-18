SERVICE_NAME=api
VERSION=$(shell git rev-parse --short HEAD)
AWS_ACCOUNT=plz-fill-me-in
AWS_REGION=ap-southeast-2
REGISTRY=$(AWS_ACCOUNT).dkr.ecr.$(AWS_REGION).amazonaws.com/$(SERVICE_NAME)

.PHONY:
version:
	@echo "$(VERSION)"

.PHONY:
build:
	go build -o out/api cmd/api/main.go

.PHONY:
docker-build:
	docker build . -t $(SERVICE_NAME)

.PHONY:
docker-push:
	docker tag $(SERVICE_NAME) $(REGISTRY):$(VERSION) && \
	docker push $(REGISTRY):$(VERSION)

.PHONY:
ci-build-and-push: docker-build docker-push
	@echo "Done"
