

all: data/merge.comp.spv data/column.comp.spv data/relabel.comp.spv data/headless.comp.spv
clean:
	rm -f data/merge.comp.spv
	rm -f data/column.comp.spv
	rm -f data/relabel.comp.spv
	rm -f data/headless.comp.spv

data/column.comp.spv: data/column.comp
	glslc -O \
	-c \data/column.comp \
	--target-env=vulkan1.1 \
	--target-spv=spv1.3 \
	-o data/column.comp.spv

data/merge.comp.spv: data/merge.comp
	glslc -O \
	-c \data/merge.comp \
	--target-env=vulkan1.1 \
	--target-spv=spv1.3 \
	-o data/merge.comp.spv

data/relabel.comp.spv: data/relabel.comp
	glslc -O \
	-c \data/relabel.comp \
	--target-env=vulkan1.1 \
	--target-spv=spv1.3 \
	-o data/relabel.comp.spv

data/headless.comp.spv: data/headless.comp
	glslc -O \
	-c \data/headless.comp \
	--target-env=vulkan1.1 \
	--target-spv=spv1.3 \
	-o data/headless.comp.spv
