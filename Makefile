.PHONY: upload
upload:
	rsync -var book/ humberto@ccom.uprrp.edu:public_html/pages/teaching/compilers2021/book/
