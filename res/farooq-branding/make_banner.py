#!/usr/bin/env python3
"""Farooq Remote 1.1.3: the logo at the top of the main window (flutter/assets/logo.png, shown at most 300 x 60 by
loadLogo() in flutter/lib/common.dart) = the owner's Installer-header-banner.jpg (1 Nov 2026, «for icons and logos»):
the white icon + «Farooq Remote» on his teal, cut to a banner strip with rounded corners, 3x of 60 px high.
Source: Y:\\FA-020-farooqremote\\1-working\\Remote-Branding\\Installer-header-banner.jpg (2048 x 2048, not in Git).
Usage : python3 res/farooq-branding/make_banner.py <folder with the jpg> .   (run from the repo root)
"""
import os
import sys
from PIL import Image, ImageDraw

src_dir, repo = sys.argv[1], sys.argv[2]
im = Image.open(os.path.join(src_dir, "Installer-header-banner.jpg")).convert("RGB")
x0, y0, x1, y1 = 202, 704, 1846, 1336          # the icon, the amber arc and the name (measured on the 2048 px file)
ch = y1 - y0
padx, pady = int(ch * 0.30), int(ch * 0.22)
strip = im.crop((x0 - padx, y0 - pady, x1 + padx, y1 + pady))
w, h = strip.size
mask = Image.new("L", (w * 4, h * 4), 0)
ImageDraw.Draw(mask).rounded_rectangle((0, 0, w * 4 - 1, h * 4 - 1), radius=int(h * 4 * 0.18), fill=255)
rgba = strip.convert("RGBA")
rgba.putalpha(mask.resize((w, h), Image.LANCZOS))
H = 180
rgba.resize((round(w * H / h), H), Image.LANCZOS).save(os.path.join(repo, "flutter/assets/logo.png"), optimize=True)
print("flutter/assets/logo.png written")
