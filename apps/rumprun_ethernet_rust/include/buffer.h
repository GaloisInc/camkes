/*
 * Copyright 2017, Data61
 * Commonwealth Scientific and Industrial Research Organisation (CSIRO)
 * ABN 41 687 119 230.
 *
 * This software may be distributed and modified according to the terms of
 * the BSD 2-Clause license. Note that NO WARRANTY is provided.
 * See "LICENSE_BSD2.txt" for details.
 *
 * @TAG(DATA61_BSD)
 */

#pragma once

#define ETHERNET_FRAME_BUFSIZE 1500
#define ETHERNET_FRAME_END_IDX (ETHERNET_FRAME_BUFSIZE - 1)
#define ETHERNET_FRAME_MAX_LEN (ETHERNET_FRAME_END_IDX - 1)

#define IP_FRAME_BUFSIZE 65536
#define IP_FRAME_END_IDX (IP_FRAME_BUFSIZE - 1)
#define IP_FRAME_MAX_LEN (IP_FRAME_END_IDX - 1)
