#!/usr/bin/env python3
"""临时脚本：修复 auto_repair_rust.py 中的缩进问题"""
with open('auto_repair_rust.py', 'r', encoding='utf-8') as f:
    content = f.read()

# 修复第 307-312 行区域的缩进
old_part1 = """            # 尝试读取签名文件
            translated_function_signature = ""
                signature_file = signature_dir / translated_file.name
                if signature_file.exists():
                    with open(signature_file, 'r', encoding='utf-8', errors='ignore') as f:
                    translated_function_signature = f.read().strip()"""

new_part1 = """            # 尝试读取签名文件
            translated_function_signature = ""
            signature_file = signature_dir / translated_file.name
            if signature_file.exists():
                with open(signature_file, 'r', encoding='utf-8', errors='ignore') as f:
                    translated_function_signature = f.read().strip()"""

# 修复 try-except 块的缩进
old_part2 = """                if not success:
                    # 如果新方法失败，尝试旧的简单方法
                    try:
                left = skeleton_result.split(translated_function_signature)[0]
                right = skeleton_result.split(translated_function_signature)[1]
                skeleton_result_lines = skeleton_result.splitlines()
                for line in skeleton_result_lines:
                    if translated_function_signature in line:
                        if line.strip().endswith(";"):
                            translated_result = left + translated_function + right[right.find(";") + 1:]
                        else:
                            translated_result = left + translated_function + right[right.find("}") + 1:]
                        break
            except:
                        # 最后的备选：在第一个函数位置插入
                        frist_function_position_start = get_frist_function_position_start(skeleton_result)
                        translated_result = list(skeleton_result)
                        translated_result.insert(frist_function_position_start, translated_function)
                        translated_result = "".join(translated_result)"""

new_part2 = """                if not success:
                    # 如果新方法失败，尝试旧的简单方法
                    try:
                        left = skeleton_result.split(translated_function_signature)[0]
                        right = skeleton_result.split(translated_function_signature)[1]
                        skeleton_result_lines = skeleton_result.splitlines()
                        for line in skeleton_result_lines:
                            if translated_function_signature in line:
                                if line.strip().endswith(";"):
                                    translated_result = left + translated_function + right[right.find(";") + 1:]
                                else:
                                    translated_result = left + translated_function + right[right.find("}") + 1:]
                                break
                    except:
                        # 最后的备选：在第一个函数位置插入
                        frist_function_position_start = get_frist_function_position_start(skeleton_result)
                        translated_result = list(skeleton_result)
                        translated_result.insert(frist_function_position_start, translated_function)
                        translated_result = "".join(translated_result)"""

if old_part1 in content:
    content = content.replace(old_part1, new_part1)
    print("Fixed part 1")
else:
    print("Part 1 not found or already fixed")

if old_part2 in content:
    content = content.replace(old_part2, new_part2)
    print("Fixed part 2")
else:
    print("Part 2 not found or already fixed")

with open('auto_repair_rust.py', 'w', encoding='utf-8') as f:
    f.write(content)

print("Done!")

