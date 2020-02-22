
# compiles all the GLSL files in ./data

SOURCE_DIR=data
BUILD_DIR=data

SOURCES=$(shell find $(SOURCE_DIR) -name '*.comp')
OBJECTS=$(patsubst $(SOURCE_DIR)/%.comp, $(BUILD_DIR)/%.comp.spv, $(SOURCES))

all: $(OBJECTS)
clean:
	rm -f $(BUILD_DIR)/*.spv

$(BUILD_DIR)/%.comp.spv: $(SOURCE_DIR)/%.comp
	glslc -O \
	-c $< \
	--target-env=vulkan1.1 \
	--target-spv=spv1.3 \
	-o $@
