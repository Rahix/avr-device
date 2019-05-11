all: deps chips

CHIPS := atmega8 atmega328p atmega32u4 attiny85

PATCHES := $(foreach chip, $(CHIPS), $(wildcard patch/$(chip).yaml))
DEPS := $(foreach patch, $(PATCHES), $(patsubst patch/%.yaml, .deps/%.d, $(patch)))

.PHONY: chips deps
chips: $(foreach chip, $(CHIPS), src/devices/$(chip)/mod.rs)
deps: $(DEPS)

svd/%.bare.svd: vendor/%.atdf
	@mkdir -p svd
	@echo -e "\tATDF2SVD\t$*"
	@atdf2svd $< $@ 2>/dev/null

$(foreach patch, $(PATCHES), $(eval $(patsubst patch/%.yaml, svd/%.patched.svd, $(patch)): $(patch)))

svd/%.patched.svd: svd/%.bare.svd patch/svdpatch.py
	@if [ -f patch/$*.yaml ] ; then \
		echo -e "\tPATCH\t\t$*"; \
		python3 patch/svdpatch.py patch/$*.yaml $< $@; \
	else \
		echo -e "\t  - No patches found for $*"; \
		cp $< $@; \
	fi

src/devices/%/mod.full.rs: svd/%.patched.svd
	@mkdir -p $(@D)
	@echo -e "\tSVD2RUST\t$*"
	@cd $(@D); svd2rust --target none -i $(realpath $<)
	@mv $(@D)/lib.rs $@

src/devices/%/mod.rs: src/devices/%/mod.full.rs
	@echo -e "\tFORM\t\t$*"
	@RUST_LOG=WARN form -i $< -o $(@D) >/dev/null
	@rm $<
	@mv $(@D)/lib.rs $@
	@rustfmt $@
	@# Remove the `extern crate` lines
	@sed -i "1,7d" $@
	@# Make DEVICE_PERIPHERALS visible crate-wide
	@sed -i 's/\(static mut DEVICE_PERIPHERALS\)/pub(crate) \0/' $@

clean:
	@echo -e "\tCLEAN\t\t./svd/"
	@rm -rf svd/
	@echo -e "\tCLEAN\t\t./src/devices"
	@rm -rf src/devices/at*
	@echo -e "\tCLEAN\t\t./.deps/"
	@rm -rf .deps/

# Patch dependencies
.deps/%.d: patch/%.yaml
	@mkdir -p .deps
	@echo -e "\tMAKEDEPS\t$*"
	@python3 patch/makedeps.py $< >$@

-include $(DEPS)
