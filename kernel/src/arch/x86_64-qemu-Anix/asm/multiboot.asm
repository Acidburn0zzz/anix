; Copyright (C) 2018-2020 Nicolas Fouquet
;
; This program is free software: you can redistribute it and/or modify
; it under the terms of the GNU General Public License as published by
; the Free Software Foundation, either version 3 of the License, or
; (at your option) any later version.
;
; This program is distributed in the hope that it will be useful,
; but WITHOUT ANY WARRANTY; without even the implied warranty of
; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
; GNU General Public License for more details.
;
; You should have received a copy of the GNU General Public License
; along with this program.  If not, see https://www.gnu.org/licenses.

section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd header_end - header_start ; header length
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    align 8

video_tag:
    dw 5                            ; video tag id
    dw 0                            ; flags
    dd video_tag_end - video_tag    ; size
    dd 1024                         ; width
    dd 768                          ; height
    dd 32                           ; depth
video_tag_end:

    align 8

tag_end_start:
    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
tag_end_end:
header_end:
