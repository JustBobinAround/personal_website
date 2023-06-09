run: markdown
	trunk serve --release --public-url personal_website

build: markdown
	trunk build --release --public-url personal_website

install:
	rustup target add wasm32-unknown-unknow
	cargo install --locked trunk

markdown:
	sh ./build_scripts/build_markdown_lists.sh

deploy:
	git push -u origin main
	git subtree push --prefix dist origin gh-pages

documents: ./doc_src/personal_website.tex
	pdflatex -output-directory=./doc_out/ ./doc_src/personal_website.tex
	pdflatex -output-directory=./doc_out/ ./doc_src/personal_website.tex
	mv -f ./doc_out/personal_website.aux ./doc_logs/
	mv -f ./doc_out/personal_website.log ./doc_logs/
	mv -f ./doc_out/personal_website.toc ./doc_logs/
