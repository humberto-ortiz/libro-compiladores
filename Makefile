.PHONY: default
default: build

.PHONY: build
build:
	mdbook build

.PHONY: upload
upload:
	rsync -var book/ humberto@ccom.uprrp.edu:public_html/pages/teaching/compilers2021/book/
