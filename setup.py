# -*- coding: utf-8 -*-
from setuptools import setup, find_packages


def build_native(spec):
    # build an example rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./cabi'
    )

    spec.add_cffi_module(
        module_path='simplet2s._native',
        dylib=lambda: build.find_dylib('simplet2s', in_path='target/release'),
        header_filename=lambda: build.find_header('simplet2s.h', in_path='target'),
        rtld_flags=['NOW', 'NODELETE']
    )


setup(
    name='simplet2s',
    version='0.2.0',
    url='https://github.com/bosondata/simplet2s-rs',
    description='A simple traditional Chinese to simplified Chinese converter',
    packages=find_packages(),
    zip_safe=False,
    platforms='any',
    setup_requires=['milksnake'],
    install_requires=['milksnake'],
    tests_require=['pytest'],
    milksnake_tasks=[
        build_native
    ]
)
