all: deps chips

CHIPS := at90usb1286 atmega1280 atmega1284p atmega128a atmega128rfa1 atmega16 atmega164pa atmega168 atmega16u2 atmega2560 atmega8 atmega8u2 atmega324pa atmega328p atmega328pb atmega32a atmega32u2 atmega32u4 atmega3208 atmega3209 atmega4808 atmega4809 atmega48p atmega64 atmega644 atmega88p attiny13a attiny202 attiny2313 attiny2313a attiny26 attiny402 attiny404 attiny44a attiny84 attiny85 attiny88 attiny816 attiny828 attiny841 attiny84a attiny861 attiny167 attiny1614 avr64du32 avr64du28

RUSTUP_TOOLCHAIN ?= nightly

PATCHES := $(foreach chip, $(CHIPS), $(wildcard patch/$(chip).yaml))
DEPS := $(foreach patch, $(PATCHES), $(patsubst patch/%.yaml, .deps/%.d, $(patch)))

.PHONY: chips deps $(CHIPS) vector all clean
chips: $(CHIPS)
deps: $(DEPS)
vector: macros/src/vector.rs

$(foreach chip, $(CHIPS), $(eval $(chip): src/devices/$(chip)/mod.rs))

.SECONDARY:
svd/%.svd: vendor/%.atdf
	@mkdir -p svd
	@echo -e "\tATDF2SVD\t$*"
	@atdf2svd $< $@ 2>/dev/null

svd/%.svd.patched: svd/%.svd .deps/%.d patch/%.yaml
	@if [ -f patch/$*.yaml ] ; then \
		echo -e "\tPATCH\t\t$*"; \
		svd patch patch/$*.yaml; \
		test -e $@; \
	else \
		echo -e "\t  - No patches found for $*"; \
		cp $< $@; \
	fi

src/devices/%/mod.full.rs: svd/%.svd.patched
	@mkdir -p $(@D)
	@echo -e "\tSVD2RUST\t$*"
	@cd $(@D); svd2rust --generic_mod --make_mod --target none -i $(realpath $<)
	@mv $(@D)/mod.rs $@
	@mv $(@D)/generic.rs $(@D)/../../generic.rs

src/devices/%/mod.rs: src/devices/%/mod.full.rs
	@echo -e "\tFORM\t\t$*"
	@RUST_LOG=WARN form -i $< -o $(@D) >/dev/null
	@rm $<
	@mv $(@D)/lib.rs $@
	@RUSTUP_TOOLCHAIN=$(RUSTUP_TOOLCHAIN) rustfmt $@
	@# Remove the `extern crate` lines
	@sed -i'' -e "/^extern crate/d" $@
	@# Remove DEVICE_PERIPHERALS declaration and replace it with a reference
	@# to the global version
	@patch --no-backup-if-mismatch --quiet $@ patch/modrs.patch
	@# Fixup the take() implementation
	@sed -i'' -e '/#\[cfg(feature = "critical-section")]/d' $@
	@sed -i'' -e 's/critical_section::with/crate::interrupt::free/' $@
	@echo -e "\tGEN-VECTOR\t>macros/src/vector.rs"
	@./gen-intr-lut.sh svd/*.patched >macros/src/vector.rs

macros/src/vector.rs: svd/*.patched
	@echo -e "\tGEN-VECTOR\t>macros/src/vector.rs"
	@./gen-intr-lut.sh $^ >$@

clean:
	@echo -e "\tCLEAN\t\t./svd/"
	@rm -rf svd/
	@echo -e "\tCLEAN\t\t./src/devices"
	@rm -rf src/devices/at*
	@echo -e "\tCLEAN\t\t./src/generic.rs"
	@rm -f src/generic.rs
	@echo -e "\tCLEAN\t\t./.deps/"
	@rm -rf .deps/
	@echo -e "\tCLEAN\t\t./macros/src/vector.rs"
	@rm -rf macros/src/vector.rs

# Patch dependencies
patch/%.yaml: .deps/%.d
.deps/%.d: patch/%.yaml
	@mkdir -p .deps
	@echo -e "\tMAKEDEPS\t$*"
	@svd makedeps $< $@

-include $(DEPS)
