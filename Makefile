# @Makefile by tde-los- (http://github.com/NineSuper)
UNAME_S := $(shell uname -s)

NAME = Zappy

ifeq ($(UNAME_S),Linux)		# Linux
    # LFLAGS =
	INSTALL_DEP = apt install cargo -y
endif

ifeq ($(UNAME_S),Darwin)	# macOS
    # LFLAGS =
	INSTALL_DEP =
endif

BIN_DIR		= bin

SERVER_BIN 	= $(BIN_DIR)/server
SERVER_CC 	= cargo build
SERVER_FLAG = --release
SERVER_DIR 	= server
SERVER_SRC 	= $(SERVER_DIR)/src
SERVER_DEBUG = RUSTFLAGS="-Awarnings" # enlève les warnings mais pas les erreurs

# TODO
CLIENT_BIN 	= $(BIN_DIR)/client
CLIENT_CC  	= cargo build
CLIENT_FLAG = --release
CLIENT_DIR 	= client
CLIENT_SRC 	= $(CLIENT_DIR)/src
SERVER_DEBUG = RUSTFLAGS="-Awarnings" # enlève les warnings mais pas les erreurs

# TODO
GFX_BIN 	= $(BIN_DIR)/gfx
GFX_CC  	=
GFX_FLAG 	=
GFX_DIR 	= gfx
GFX_SRC 	= $(GFX_DIR)/src

OBJ_DIR = obj
$(OBJ_DIR):
	@mkdir -p $(OBJ_DIR)

OBJ =

# ------------------------------ Couleurs ------------------------------

BOLD			=		\033[1m

B_RED			=		\033[1;31m
B_GREEN   		=		\033[1;32m
B_YELLOW     	=       \033[1;33m
B_BLUE    		=		\033[1;34m
B_PURPLE     	=       \033[1;35m
B_CYAN       	=       \033[1;36m
B_WHITE   		=		\033[1;37m

NO_COLOR        =       \033[0m

# ------------------------------ Messages ------------------------------

HEADER_COMP     =       echo "\n🤖 $(B_BLUE)$(NAME)$(NO_COLOR)$(BOLD) by $(B_RED)hbaduel $(B_WHITE)& $(B_PURPLE)mafrendo $(B_WHITE)& $(B_CYAN)lmas $(B_WHITE)& $(B_GREEN)tde-los-\n"

COMP_START      =       printf "\n🚧 $(B_YELLOW)Make: $(NO_COLOR)$(BOLD)Debut de compilation...\r$(NO_COLOR)"

EXE_READY       =       echo "\n📦 $(B_BLUE)$(NAME)$(NO_COLOR) $(BOLD)est prêt à être exécuté !$(NO_COLOR)\n"

CLEANED         =       echo "\n💧 $(B_CYAN)Clean: $(NO_COLOR)$(BOLD)Suppression des fichiers objets$(NO_COLOR)\n"

FCLEANED        =       echo "\n🧼 $(B_CYAN)Fclean: $(NO_COLOR)$(BOLD)Suppression des fichiers (binaire + objets + libs)$(NO_COLOR)\n"

SERV_START		=		printf "\n🪩  $(B_YELLOW)Server: $(NO_COLOR)$(BOLD)Debut de compilation...\r$(NO_COLOR)\n\n"

SERV_READY		=		echo "\n🪩  $(B_YELLOW)Server: $(NO_COLOR)$(BOLD)Compilation terminée ✅$(NO_COLOR)\n"

CLIENT_START	=		echo "\n🎮 $(B_RED)Client: $(NO_COLOR)$(BOLD)Debut de compilation...\r$(NO_COLOR)\n"

CLIENT_READY	=		echo "\n🎮 $(B_RED)Client: $(NO_COLOR)$(BOLD)Compilation terminée ✅$(NO_COLOR)\n"

GFX_START		=		echo "\n🐉 $(B_PURPLE)GFX: $(NO_COLOR)$(BOLD)Debut de compilation...\r$(NO_COLOR)\n"

GFX_READY		=		echo "\n🐉 $(B_PURPLE)GFX: $(NO_COLOR)$(BOLD)Compilation terminée ✅$(NO_COLOR)\n"

define print_progress
	@COUNT=0; \
	for FILE in $(1); do \
		COUNT=$$(($$COUNT + 1)); \
		PERCENT=$$((100 * $$COUNT / $(2))); \
		printf "$(3) %-33.33s [%3d%%]\r" $$FILE $$PERCENT; \
		sleep 0.05; \
	done; \
	printf "\n"
endef

# ------------------------------ Regles ------------------------------
MAKEFLAGS += --silent

.DEFAULT_GOAL := all

# TODO
TOTAL_FILES_SERV := $(words $(SERVER_SRC))
TOTAL_FILES_CLIENT := $(words $(CLIENT_SRC))
TOTAL_FILES_GFX := $(words $(GFX_SRC))

COMPILED_FILES_SERV := 0
COMPILED_FILES_CLIENT := 0
COMPILED_FILES_GFX := 0

all:
	@$(MAKE) comp_start
	@$(MAKE) server
	@$(MAKE) client
	@$(MAKE) gfx
	@$(EXE_READY)

install:
	$(INSTALL_DEP)

server:
	@$(SERV_START)
	@cd $(SERVER_DIR) && $(SERVER_CC) $(SERVER_FLAG)
	@cp $(SERVER_DIR)/target/release/$(notdir $(SERVER_DIR)) $(SERVER_BIN)
	@$(SERV_READY)

server_clean:
	rm -f $(SERVER_BIN)
	@cd $(SERVER_DIR) && cargo clean
	rm -f $(SERVER_DIR)/*.lock

client:
	@$(CLIENT_START)
	@cd $(CLIENT_DIR) && $(CLIENT_CC) $(CLIENT_FLAG)
	@cp $(CLIENT_DIR)/target/release/$(notdir $(CLIENT_DIR)) $(CLIENT_BIN)
	@$(CLIENT_READY)

client_clean:
	rm -f $(CLIENT_BIN)
	@cd $(CLIENT_DIR) && cargo clean
	rm -f $(CLIENT_DIR)/*.lock

gfx:
	@$(GFX_START)
#	@$(GFX_READY)

comp_start:
	@$(HEADER_COMP)

clean:
	$(CLEANED)
	@$(MAKE) server_clean
	@$(MAKE) client_clean
	rm -f $(GFX_BIN)

fclean: clean
	$(FCLEANED)

re: fclean all

.PHONY: all server client gfx comp_start clean fclean re server_clean client_clean
