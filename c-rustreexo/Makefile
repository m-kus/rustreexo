OBJECT_INSTALL_PATH=/usr/local/lib/
HEADERS_INSTALL_PATH=/usr/local/include/
LIB_NAME=librustreexo
LIB_FILE_NAME=libc_rustreexo.a
CC=gcc

all: build

build:
	@cargo build --release

build-debug:
	@cargo build

examples: build-debug
	@$(CC) -I $(PWD)/include/ -L $(PWD)/target/debug/ examples/simple_stump_example.c -l:LIB_FILE_NAME -o $(PWD)/examples/pollard

install: build
	sudo mkdir -p $(HEADERS_INSTALL_PATH)/$(LIB_NAME)/
	sudo cp $(PWD)/include/* $(HEADERS_INSTALL_PATH)/$(LIB_NAME)/
	sudo cp $(PWD)/target/release/$(LIB_FILE_NAME) $(OBJECT_INSTALL_PATH)/$(LIB_FILE_NAME)

uninstall:
	sudo rm -r $(HEADERS_INSTALL_PATH)/$(LIB_NAME)/
	sudo rm $(OBJECT_INSTALL_PATH)/$(LIB_FILE_NAME)
