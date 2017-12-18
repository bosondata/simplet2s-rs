# -*- coding: utf-8 -*-
from __future__ import absolute_import
import sys

from ._native import ffi, lib


PY2 = sys.version_info[0] == 2

if PY2:
    text_type = unicode  # noqa
else:
    text_type = str


def decode_str(s):
    """Decodes a FfiStr"""
    try:
        if s.len == 0:
            return u''
        return ffi.unpack(s.data, s.len).decode('utf-8', 'replace')
    finally:
        lib.simplet2s_str_free(ffi.addressof(s))


def convert(text):
    if isinstance(text, text_type):
        text = text.encode('utf-8')
    r = lib.simplet2s_convert(text)
    return decode_str(r)
