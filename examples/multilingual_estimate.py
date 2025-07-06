import tiktoken

from skimtoken.multilingual_simple import estimate_tokens

# Initialize tiktoken encoder
encoder = tiktoken.get_encoding("o200k_base")


# Compare token estimation
def compare_tokens(lang_name: str, text: str) -> None:
    tik = len(encoder.encode(text))
    skim = estimate_tokens(text)
    diff = abs(skim - tik)
    accuracy = (1 - diff / tik) * 100 if tik > 0 else 0
    print(f"{lang_name:12} | Tik: {tik:3} | Skim: {skim:3} | Diff: {diff:3} ({accuracy:.1f}%)")


# All test data organized in dictionaries
test_data = {
    "English": "Token prediction is difficult for non-space languages",
    "Japanese": "スペースで区切られていない言語の場合トークン数を予測するのは難しいです",
    "Chinese": "对于没有空格分隔的语言，预测词元数量是困难的",
    "Thai": "การทำนายจำนวนโทเคนเป็นเรื่องยากสำหรับภาษาที่ไม่มีช่องว่าง",
}

for lang_name, text in test_data.items():
    compare_tokens(lang_name, text)
