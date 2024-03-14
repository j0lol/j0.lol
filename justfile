watch:
	cargo watch -d 2 -x run

deploy:
	fly deploy

docker-build:
	docker build --tag 'j0lol-site' .

docker-run:
	docker run -it 'j0lol-site'
