# PCNT 解码旋转编码器

请注意，PCNT 只跟踪单个的 16 位值。我们使用中断来检测 LOW 和 HIGH 阈值并跟踪其占比，并提供 i64 值结果

注：示例来源于官方。