IMAGES_FOLDER ?= $(INPUT_FOLDER)/images
INPUT_FOLDER ?= $(shell pwd)
OUTPUT_FOLDER ?= $(shell pwd)/dist

slides:
	@pandoc rust-intro.md \
	--from markdown+tex_math_single_backslash \
	--to revealjs \
	--output rust-intro.html \
	-V revealjs-url=template \
	-V theme=white \
	-V progress=true \
	-V slideNumber=true \
	-V history=true \
	--standalone --slide-level 2
