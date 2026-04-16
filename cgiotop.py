#!/usr/bin/env python3

# 将 systemd 统计的 cgroup io 排序打印。
# 需要 system.conf 和 user.conf [systemd-system.conf(5)] 里开启 DefaultIOAccounting=yes
import sys
import re
import subprocess

services = []
current = None

# 匹配 service 行
service_re = re.compile(r"^[^ ]\s+(\S+)\s+-\s+(.*)")
# 匹配 IO 行
io_re = re.compile(r"IO:\s+([\d\.]+)([KMG]?)B? read,\s+([\d\.]+)([KMG]?)B? written")

def to_bytes(value, unit):
    value = float(value)
    scale = {
        "": 1,
        "K": 1024,
        "M": 1024**2,
        "G": 1024**3,
    }
    return int(value * scale.get(unit, 1))


cmd = r"""systemctl -n0 --user --type=service status | rg -e 'IO:' -e '^[^ ] [^ ]+\.service'"""

proc = subprocess.run(cmd, stdout=subprocess.PIPE, shell=True, text=True)
proc.check_returncode()
for line in proc.stdout.splitlines():
    line = line.strip()

    m = service_re.match(line)
    if m:
        current = {
            "name": m.group(1),
            "desc": m.group(2),
            "written": 0,
        }
        services.append(current)
        continue

    m = io_re.search(line)
    if m and current:
        written_val, written_unit = m.group(3), m.group(4)
        current["written"] = to_bytes(written_val, written_unit)
        current['w_val'] = written_val
        current['w_unit'] = written_unit

# 排序（按写入量降序）
services.sort(key=lambda x: x["written"], reverse=True)

# 输出
for s in services:
    if s['written'] != 0:
        print(f"{s['w_val']:>12}{s['w_unit']:1}  {s['name']} ({s['desc']})")

# 最大的
if services:
    top = services[0]
    print("\nTop writer:")
    print(f"{top['name']} ({top['desc']}) -> {top['written']} bytes")
