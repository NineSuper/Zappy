# @Makefile by tde-los- (http://github.com/NineSuper)
UNAME_S := $(shell uname -s)

NAME = Zappy

CC =
FLAGS =

ifeq ($(UNAME_S),Linux)		# Linux
    LFLAGS =
endif

ifeq ($(UNAME_S),Darwin)	# macOS
    LFLAGS =
endif

# TODO
SERVER_DIR = server
SERVER_SRC = $(SERVER_DIR)/src

# TODO
CLIENT_DIR = client
CLIENT_SRC = $(CLIENT_DIR)/src

# TODO
GFX_DIR = gfx
GFX_SRC = $(GFX_DIR)/src

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

EXE_READY       =       echo "\n🐿️  $(B_BLUE)$(NAME)$(NO_COLOR) $(BOLD)est prêt à être exécuté !$(NO_COLOR)\n"

CLEANED         =       echo "\n💧 $(B_CYAN)Clean: $(NO_COLOR)$(BOLD)Suppression des fichiers objets$(NO_COLOR)\n"

FCLEANED        =       echo "\n🧼 $(B_CYAN)Fclean: $(NO_COLOR)$(BOLD)Suppression des fichiers (binaire + objets + libs)$(NO_COLOR)\n"

SERV_START		=		printf "\n💿 $(B_YELLOW)Server: $(NO_COLOR)$(BOLD)Debut de compilation...\r$(NO_COLOR)"

SERV_READY		=		echo "\n💿 $(B_YELLOW)Server: $(NO_COLOR)$(BOLD)Compilation terminée ✅$(NO_COLOR)\n"

CLIENT_START	=		printf "\n🎮 $(B_RED)Client: $(NO_COLOR)$(BOLD)Debut de compilation...\r$(NO_COLOR)"

CLIENT_READY	=		echo "\n🎮 $(B_RED)Client: $(NO_COLOR)$(BOLD)Compilation terminée ✅$(NO_COLOR)\n"

GFX_START		=		printf "\n🐉 $(B_PURPLE)GFX: $(NO_COLOR)$(BOLD)Debut de compilation...\r$(NO_COLOR)"

GFX_READY		=		echo "\n🐉 $(B_PURPLE)GFX: $(NO_COLOR)$(BOLD)Compilation terminée ✅$(NO_COLOR)\n"

# ------------------------------ Regles ------------------------------
MAKEFLAGS += --silent

.DEFAULT_GOAL := all

# TODO
# TOTAL_FILES_SERV := $(words $(SRC))
# TOTAL_FILES_CLIENT := $(words $(SRC))
# TOTAL_FILES_GFX := $(words $(SRC))

# COMPILED_FILES := 0

all:
	@$(MAKE) comp_start
	@$(MAKE) server
	@$(MAKE) client
	@$(MAKE) gfx
	@$(EXE_READY)

server:
	@$(SERV_START)
	$(SERV_READY)

client:
	@$(CLIENT_START)
	@$(CLIENT_READY)

gfx:
	@$(GFX_START)
	@$(GFX_READY)

comp_start:
	@$(HEADER_COMP)

clean:
	$(CLEANED)

fclean: clean
	$(FCLEANED)


re: fclean all

.PHONY: all server client gfx comp_start clean fclean re

# TODO
# @printf "🚧 $(BOLD_YELLOW)Make: $(NO_COLOR)$(BOLD)Compilation des fichiers :$(BOLD_CYAN) %-33.33s $(BOLD_YELLOW)[%3d%%] \r$(NO_COLOR)" $? $(shell expr \( $(COMPILED_FILES) \) \* 100 / $(TOTAL_FILES))
