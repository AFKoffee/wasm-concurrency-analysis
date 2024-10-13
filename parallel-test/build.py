#!/usr/bin/env python3
import os
import subprocess

root_dir = os.path.dirname(__file__)

subprocess.run(["cargo", "build", "--release"], cwd=root_dir).check_returncode()
subprocess.run(["wasm-pack", "build", "--target", "no-modules"], cwd=root_dir).check_returncode()