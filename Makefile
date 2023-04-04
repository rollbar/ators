docker-build:
	docker build -t ators .

docker-run:
	docker run --rm ators -i -o=/usr/src/iosAppSwift -l=0x0100360000 -- 0x0100369e4c

release-mac:
	strip target/release/ators
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/ators-mac.tar.gz ./ators

release-linux:
	strip target/release/ators
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/ators-linux.tar.gz ./ators
