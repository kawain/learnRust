# Linuxの場合
# target/release/lib****.so
# をpythonと同階層に持ってきて
# ****.so
# にリネーム
#
# Windowsの場合
# \target\release\****.dll
# をpythonと同階層に持ってきて
# ****.pyd
# にリネーム
#
# インポート
import rpnpy

a = input("逆ポーランド入力：")
b = rpnpy.rpn(a)
print(b)
