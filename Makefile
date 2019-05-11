all: chips

CHIPS := atmega328p atmega32u4 attiny85

.PHONY: chips
chips: $(foreach chip, $(CHIPS), src/devices/$(chip)/mod.rs)

svd/%.bare.svd: vendor/%.atdf
	mkdir -p svd
	atdf2svd $< $@

svd/%.patched.svd: svd/%.bare.svd
	if [ -f patch/$*.yaml ] ; then \
		python3 patch/svdpatch.py patch/$*.yaml $< $@; \
	else \
		echo "No patches found for $*"; \
		cp $< $@; \
	fi

src/devices/%/mod.full.rs: svd/%.patched.svd
	mkdir -p $(@D)
	cd $(@D); svd2rust --target none -i $(realpath $<)
	mv $(@D)/lib.rs $@

src/devices/%/mod.rs: src/devices/%/mod.full.rs
	form -i $< -o $(@D)
	rm $<
	mv $(@D)/lib.rs $@
	rustfmt $@
	@# Remove the `extern crate` lines
	sed -i "1,7d" $@
	@# Make DEVICE_PERIPHERALS visible crate-wide
	sed -i 's/\(static mut DEVICE_PERIPHERALS\)/pub(crate) \0/' $@

clean:
	rm -rf svd
	rm -rf src/devices/at*
