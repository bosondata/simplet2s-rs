# -*- coding: utf-8 -*-
from __future__ import absolute_import
import sys

from ._native import ffi, lib


__version__ = u'0.2.0'

PY2 = sys.version_info[0] == 2

if PY2:
    text_type = unicode  # noqa
else:
    text_type = str


def convert(text):
    if isinstance(text, text_type):
        text = text.encode('utf-8')
    text_len = len(text)
    try:
        r = lib.simplet2s_convert(text, text_len)
        s = ffi.unpack(r, text_len).decode('utf-8', 'replace')
        return s
    finally:
        lib.simplet2s_str_free(r, text_len)
