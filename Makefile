all: deps chips

CHIPS := atmega1280 atmega8 atmega328p atmega32u4 attiny85

PATCHES := $(foreach chip, $(CHIPS), $(wildcard patch/$(chip).yaml))
DEPS := $(foreach patch, $(PATCHES), $(patsubst patch/%.yaml, .deps/%.d, $(patch)))

.PHONY: chips deps $(CHIPS) vector
chips: $(CHIPS)
deps: $(DEPS)
vector: macros/src/vector.rs

$(foreach chip, $(CHIPS), $(eval $(chip): src/devices/$(chip)/mod.rs))

.SECONDARY:
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
	@cd $(@D); svd2rust --generic_mod --target none -i $(realpath $<)
	@mv $(@D)/lib.rs $@
	@mv $(@D)/generic.rs $(@D)/../../generic.rs

src/devices/%/mod.rs: src/devices/%/mod.full.rs
	@echo -e "\tFORM\t\t$*"
	@RUST_LOG=WARN form -i $< -o $(@D) >/dev/null
	@rm $<
	@mv $(@D)/lib.rs $@
	@RUSTUP_TOOLCHAIN=nightly rustfmt $@
	@# Remove the `extern crate` lines
	@sed -i'' -e "1,7d" $@
	@# Remove DEVICE_PERIPHERALS declaration and replace it with a reference
	@# to the global version
	@sed -i'' -e '/^\#\[no_mangle\]/,+1cuse crate::devices::DEVICE_PERIPHERALS;' $@
	@echo -e "\tGEN-VECTOR\t>macros/src/vector.rs"
	@./gen-intr-lut.sh src/devices/*/interrupt.rs >macros/src/vector.rs

macros/src/vector.rs: src/devices/*/interrupt.rs
	@echo -e "\tGEN-VECTOR\t>macros/src/vector.rs"
	@./gen-intr-lut.sh $^ >$@

clean:
	@echo -e "\tCLEAN\t\t./svd/"
	@rm -rf svd/
	@echo -e "\tCLEAN\t\t./src/devices"
	@rm -rf src/devices/at*
	@echo -e "\tCLEAN\t\t./.deps/"
	@rm -rf .deps/
	@echo -e "\tCLEAN\t\t./macros/src/vector.rs"
	@rm -rf macros/src/vector.rs

# Patch dependencies
.deps/%.d: patch/%.yaml
	@mkdir -p .deps
	@echo -e "\tMAKEDEPS\t$*"
	@python3 patch/makedeps.py $< >$@

-include $(DEPS)
