test: ./atc_console ./bytecode_byter ./main.asc
	cargo r -r
	./bytecode_byter ./out.atc
	./atc_console ./out.atc > ./ftc_cons.atc_log

build: ./main.asc
	cargo r -r

debug: ./bytecode_byter ./main.asc
	cargo r -r
	./bytecode_byter ./out.atc