.PHONY: all python rust quite

all: python rust

python:
	pytest .

rust:
	cargo t

quiet:
	cargo t -q
	pytest . -q
