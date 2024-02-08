if [ ! -f bin/t5 ]; then
	cd t5
	cargo build --release
	cp target/release/t5 ../bin/
	cd -
else
	cd t5
	cargo clean
	cd -
fi
rm -rf data
bin/t5 .
wc data/translate/*
tar cvfj ~/data.tar.bz2 data
