run:
	trunk serve --release

build:
	trunk build --release

install:
	rustup target add wasm32-unknown-unknow
	cargo install --locked trunk


documents: ./doc_src/personal_website.tex
	pdflatex -output-directory=./doc_out/ ./doc_src/personal_website.tex
	pdflatex -output-directory=./doc_out/ ./doc_src/personal_website.tex
	mv -f ./doc_out/personal_website.aux ./doc_logs/
	mv -f ./doc_out/personal_website.log ./doc_logs/
	mv -f ./doc_out/personal_website.toc ./doc_logs/
