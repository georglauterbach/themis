SHELL     := /bin/bash -eEu -o pipefail -c

SRCDIR    := src
OUTDIR    := build
DEPDIR    := $(OUTDIR)/dependencies
LIBRARY   := $(OUTDIR)/libffi.a

AR        ?= ar
CPP       ?= cpp
CC        ?= gcc
CXX       ?= g++
AS        ?= as

SRCFILES  := $(shell find $(SRCDIR) -regextype egrep -regex ".*\.(c|cc|S)")
OBJFILES  := $(addsuffix .o, $(patsubst $(SRCDIR)%, $(OUTDIR)%, $(basename $(SRCFILES))))
DEPFILES  := $(patsubst $(OUTDIR)%.o,$(DEPDIR)%.d,$(OBJFILES))

__CFLAGS  := -m64 -Wall -Wextra -pedantic -fPIC -I $(SRCDIR)
CFLAGS    := $(CFLAGS) $(__CFLAGS) -std=c17
CXXFLAGS  := $(CXXFLAGS) $(__CFLAGS) -std=c++17
DEPFLAGS   = -MT $@ -MMD -MP -MF $(DEPDIR)/$*.d

# COMMON TARGETS
# ---------------------------------------------------------

default: $(LIBRARY)

force: clean default

clean:
	@ $(RM) -r $(OUTDIR)

$(LIBRARY): $(OBJFILES)
	@ printf 'AR\t$@\n'
	@ $(AR) -crs $(LIBRARY) $(OBJFILES)

# CREATE OBJECT FILES
# ---------------------------------------------------------

# build all .o files from source .c files
$(OUTDIR)/%.o: $(SRCDIR)/%.c | $(OUTDIR) $(DEPDIR)
	@ printf 'CC\t$<\n'
	@ [[ -d $(@D) ]] || mkdir -p $(@D)
	@ [[ -d $(DEPDIR)/$(*D) ]] || mkdir -p $(DEPDIR)/$(*D)
	@ $(CC) $(DEPFLAGS) $(CFLAGS) $${_CFLAGS:-} -c -o $@ $<

# build all .o files from source .cc files
$(OUTDIR)/%.o: $(SRCDIR)/%.cc | $(OUTDIR) $(DEPDIR)
	@ printf 'CXX\t$<\n'
	@ [[ -d $(@D) ]] || mkdir -p $(@D)
	@ [[ -d $(DEPDIR)/$(*D) ]] || mkdir -p $(DEPDIR)/$(*D)
	@ $(CXX) $(DEPFLAGS) $(CXXFLAGS) $${_CXXFLAGS:-} -c -o $@ $<

# build all .o files from source .S files
$(OUTDIR)/%.o: $(SRCDIR)/%.S | $(OUTDIR)
	@ printf 'ASM\t$<\n'
	@ [[ -d $(@D) ]] || mkdir -p $(@D)
	@ $(CPP) -o $(OUTDIR)/$*.clean.S $<
	@ $(AS) -o $@ $(OUTDIR)/$*.clean.S

# MISCELLANEOUS
# ---------------------------------------------------------

$(DEPDIR) $(OUTDIR) $(QEMU_DIR) $(EFI_BOOT_DIR):
	@- mkdir -p $@

$(DEPFILES):
-include $(wildcard $(DEPFILES))
