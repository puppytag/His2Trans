import os
import re

def findall(text, pattern):
    """
    使用正则表达式在文本中查找所有匹配的字符串
    :param text: 输入文本
    :param pattern: 正则表达式模式
    :return: 匹配的字符串列表
    """
    matches = []
    start = 0
    while True:
        index = text.find(pattern, start)
        if index == -1:
            break
        matches.append(index)
        start = index + len(pattern)
        
    return matches

def count_unsafe_frequency(path):
    total_count = 0
    total_line_count = 0
    # 编译正则表达式，匹配两个标签之间的内容（包括多行）
    # pattern = r'<translated function>(.*?)</translated function>'
    
    # 遍历目标文件夹下的所有文件
    for filename in os.listdir(path):
        filepath = os.path.join(path, filename)
        
        # 确保处理的是文件而不是目录
        if os.path.isfile(filepath):
            # try:
                # 读取文件内容
            with open(filepath, 'r', encoding='utf-8') as file:
                content = file.read()
            
            
            # 查找所有匹配的内容块
            content_blocks =  re.findall(r'<translated function>(.*?)</translated function>', content, re.DOTALL)[0].strip()
            content_blocks = [content_blocks]
            print(content_blocks)
            total_line_count += len(content_blocks[0].splitlines())

            with open(f"translated_function_final/{filename}", 'w', encoding='utf-8') as output_file:
                output_file.write(content_blocks[0])

            # 统计当前文件的unsafe出现次数
            file_count = 0
            for block in content_blocks:
                start_positions = findall(block, 'unsafe {')
                for start_position in start_positions:
                    # 计算从开始位置到结束位置的行数
                    print(start_position)
                    end_position = block.find('}', start_position)
                    if end_position != -1:
                        lines = block[start_position:end_position].splitlines()
                        file_count += len(lines)
                # file_count += block.count('unsafe {')
            
            # 累加到总次数
            total_count += file_count
                
            # except UnicodeDecodeError:
            #     print(f"警告：文件 {filename} 解码失败，已跳过")
            # except Exception as e:
            #     print(f"处理文件 {filename} 时发生错误: {str(e)}")
    
    return total_count, total_line_count

# 使用示例
if __name__ == "__main__":
    target_path = "repair_result_functions_5/translate_by_gpt_5_nano/dlp_fuse"
    if os.path.isdir(target_path):
        result, total_line_count = count_unsafe_frequency(target_path)
        print(f"目标文件夹中'unsafe'出现的总频率为: {result} {total_line_count}")
    else:
        print("错误：输入的路径不是有效目录")