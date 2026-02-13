
from transformers import AutoTokenizer, AutoModel
import torch
from torch.nn.functional import cosine_similarity

# Load the pre-trained Unixcoder model and tokenizer
model_name = "microsoft/unixcoder-base-nine"  # Replace with actual model name
tokenizer = AutoTokenizer.from_pretrained(model_name, local_files_only=True)
model = AutoModel.from_pretrained(model_name, local_files_only=True)

# import torch
# from unixcoder import UniXcoder
# from sklearn.metrics.pairwise import cosine_similarity

# device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
# model = UniXcoder("microsoft/unixcoder-base-nine")
# model.to(device)


# 定义计算代码相似度的函数
def get_code_embeddings(code: str):
    inputs = tokenizer(code, return_tensors="pt", truncation=True, padding=True, max_length=512)
    with torch.no_grad():
        embeddings = model(**inputs).last_hidden_state.mean(dim=1)  # 取平均值作为句子嵌入
    return embeddings

# 计算两个代码片段的相似度
def compute_similarity(code1: str, code2: str):
    embedding1 = get_code_embeddings(code1)
    embedding2 = get_code_embeddings(code2)

    # 计算余弦相似度
    cos_sim = torch.nn.functional.cosine_similarity(embedding1, embedding2)
    return cos_sim.item()

# 示例代码
# code1 = """def add(a, b): return a + b"""
# code2 = """def sum(a, b): return a + b"""


# similarity_score = compute_similarity(code1, code2)
# print(f"代码相似度得分: {similarity_score}")