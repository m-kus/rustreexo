OBJECT_INSTALL_PATH=/usr/local/lib
HEADERS_INSTALL_PATH=/usr/local/include
LIB_NAME=librustreexo
LIB_FILE_NAME=librustreexo.so
LICENSE_PATH=/usr/share/licenses
# This compiler is used for building C artifacts, like examples and tests
CC=gcc

all: build

clean:
	@cargo clean
	@rm -rf $(PWD)/examples/simple_test

build:
	@cargo build --release

build-debug:
	@cargo build

examples: build-debug
	@$(CC) -Wmissing-field-initializers -I $(PWD)/include/ -L $(PWD)/../target/debug/ examples/simple_stump_example.c -l:$(LIB_FILE_NAME) -lcrypto -g -o $(PWD)/examples/simple_test
	@$(CC) -Wmissing-field-initializers -I $(PWD)/include/ -L $(PWD)/../target/debug/ examples/proof_update.c -l:$(LIB_FILE_NAME) -lcrypto -g -o $(PWD)/examples/proof_update

install: build
	sudo mkdir -p $(HEADERS_INSTALL_PATH)/$(LIB_NAME)/
	sudo cp $(PWD)/include/* $(HEADERS_INSTALL_PATH)/$(LIB_NAME)/
	sudo cp $(PWD)/../target/release/$(LIB_FILE_NAME) $(OBJECT_INSTALL_PATH)/$(LIB_FILE_NAME)
	sudo cp $(PWD)/../target/release/$(LIB_FILE_NAME) $(OBJECT_INSTALL_PATH)/$(LIB_NAME).a
	sudo mkdir -p $(LICENSE_PATH)/$(LIB_NAME)/
	sudo cp -r $(PWD)/../LICENSE $(LICENSE_PATH)/$(LIB_NAME)/LICENSE

uninstall:
	sudo rm -r $(HEADERS_INSTALL_PATH)/$(LIB_NAME)/
	sudo rm $(OBJECT_INSTALL_PATH)/$(LIB_FILE_NAME)
