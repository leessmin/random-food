# random-food

今天吃什么？一个用 Rust 编写的随机食物生成器。

菜谱数据来源与 [HowToCook](https://github.com/Anduin2017/HowToCook)，随机推荐今天要做的美食。

## 功能

- 随机生成推荐菜品，附带菜谱链接
- 命令行界面，支持自定义随机食物数量和分类筛选

## 安装

### 前置要求

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)

## 示例

### 随机生成一个食物

```bash
random-food
```

### 生成多个食物

```bash
random-food -c 5
```

### 指定分类

```bash
random-food -o 素菜 荤菜 -c 3
```

### 查看所有食物分类

```bash
random-food list
```

### 查看帮助

```bash
random-food -h
```

## 命令行选项

| 选项 | 说明 |
|------|------|
| `-h, --help` | 显示帮助信息 |
| `-V, --version` | 显示版本信息 |
| `-o, --option` | 指定菜单分类列表（如：素菜 荤菜），默认全部分类 |
| `-c, --count` | 输出食物数量，默认为 1 |
| `list` | 查看所有食物分类及数量 |

## 输出示例

```
宫保鸡丁 -> https://xxx/宫保鸡丁/宫保鸡丁.md
```

点击链接即可查看完整菜谱。

## 许可证

[GPL-3.0](LICENSE)

## 致谢

菜谱数据来源于 [Anduin2017/HowToCook](https://github.com/Anduin2017/HowToCook) 项目，感谢他们的贡献。
