run: ./atc_console ./main.asc
	cargo r -r
	./bytecode_byter ./out.atc
	./atc_console ./out.atc > ./ftc_cons.trace