# -*- coding: utf-8 -*-
from __future__ import absolute_import
import sys

from ._native import ffi, lib


__version__ = u'0.1.5'

PY2 = sys.version_info[0] == 2

if PY2:
    text_type = unicode  # noqa
else:
    text_type = str


def convert(text):
    if isinstance(text, text_type):
        text = text.encode('utf-8')
    r = lib.simplet2s_convert(text)
    r = ffi.gc(r, lib.simplet2s_str_free)
    s = ffi.string(r).decode('utf-8', 'replace')
    return s
