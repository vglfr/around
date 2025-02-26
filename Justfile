alias b := build

default: build

build:
	docker image build . --tag around
	# docker container run -ti --rm around /usr/local/bin/app
