#!/usr/bin/python3

# 找出合适 KDE 的分数缩放倍率，使得逻辑分辨率是整数。

import sys

def main():
    if len(sys.argv) != 3:
        print(f'usage: {sys.argv[0]} width height', file=sys.stderr)
        exit(1)

    try:
        w = int(sys.argv[1])
        h = int(sys.argv[2])
    except ValueError:
        print(f'Width or height not number: width={sys.argv[1]}, height={sys.argv[2]}',
              file=sys.stderr)
        exit(2)

    scale = 200

    while scale >= 100:
        logical_w = w * 100 / scale
        logical_h = h * 100 / scale

        lw_str = str(logical_w)
        lh_str = str(logical_h)
        if lw_str.endswith('.0') and lh_str.endswith('.0'):
            print(f'scale = {scale}%, logical width = {logical_w}, logical height = {logical_h}')
        scale -= 1

if __name__ == '__main__':
    main()
