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

# This fragment is expected to included by other tests.

# Take the level of parallelism from the first argument, if given.
set jobs [expr {[lindex $argv 0] ne "" ? [lindex $argv 0] : 1}];

spawn make silentoldconfig
check_exit

spawn make clean
check_exit

spawn make --jobs=$jobs
check_exit
