#!/usr/bin/env python3

import subprocess

hay = """path/to/foo:54:Blue Harvest
path/to/bar:90:Something, Something, Something, Dark Side
path/to/baz:3:It's a Trap!
"""

child_env = {
    "MYVAR": hay,
}

subprocess.run(["./target/debug/cackle_test"], env=child_env)
