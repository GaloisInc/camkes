#
# Copyright 2017, Data61
# Commonwealth Scientific and Industrial Research Organisation (CSIRO)
# ABN 41 687 119 230.
#
# This software may be distributed and modified according to the terms of
# the BSD 2-Clause license. Note that NO WARRANTY is provided.
# See "LICENSE_BSD2.txt" for details.
#
# @TAG(DATA61_BSD)
#
#

CURRENT_DIR := $(dir $(abspath $(lastword ${MAKEFILE_LIST})))

include ${RUMPRUN_BASE_DIR}/platform/sel4/rumprunlibs.mk

CAMKES_FLAGS += --cpp-flag=-I${RUMPRUN_BASE_DIR}/platform/sel4/camkes/ 

RUST_SOURCE_IPSTACK := ${CURRENT_DIR}ipstack

rumprun_ipstack_rumpbin := rust_ipstack

rust_ipstack: $(RUST_SOURCE_IPSTACK)/src/main.rs
	cd $(RUST_SOURCE_IPSTACK) &&	cargo build --target=x86_64-rumprun-netbsd --verbose
	cp $(RUST_SOURCE_IPSTACK)/target/x86_64-rumprun-netbsd/debug/ipstack $(BUILD_DIR)/$@