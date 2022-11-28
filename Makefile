
IMAGE_NAME:=deltafusion

build:
	docker build -t ${IMAGE_NAME} .

run:
	docker run -it --rm -v $(shell pwd):/workspaces/rusting ${IMAGE_NAME}
