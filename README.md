# RWR GFL mod 武器数据提取工具

## 快速上手

下载[构建后代码](https://github.com/Kreedzt/rwr-gfl-weapon-parser/releases), 然后从命令行附带参数启动即可:

> 路径替换为自己的路径即可
> -i 后面参数为武器目录路径
> -t 后面参数为翻译文件路径

``` sh
.\weapon-parser.exe -i D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\weapons -t D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\languages\cn\translation_weapon.xml
```

执行成功后会在当前目录生成一个带时间的 `CSV` 文件(注意: CSV 文件在 Excel 中直接打开无法读取中文字符, 建议新建 Excel 导入 CSV 数据源打开, 编码选择 UTF-8)

## 开发

该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境

因为需要指定武器目录, 所以启动开发环境需要参数:

> 路径替换为自己的路径即可
> -i 后面参数为武器目录路径
> -t 后面参数为翻译文件路径

``` sh
cargo run -- -i D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\weapons -t D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\languages\cn\translation_weapon.xml
```

## 构建

该项目采用 Rust 语言编写，需要 [Rust](https://www.rust-lang.org/) 开发环境

编译需执行以下命令：
```bash
cargo build --release
```

编译后在根目录的 `target/release` 内生成二进制文件（exe）

编译后可用 [upx](https://github.com/upx/upx) 二次缩小体积

该文件通过终端赋予参数直接运行:

> 路径替换为自己的路径即可
> -i 后面参数为武器目录路径
> -t 后面参数为翻译文件路径

``` sh
.\weapon-parser.exe -i D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\weapons -t D:\SteamLibrary\steamapps\workshop\content\270150\2513537759\media\packages\Girls_FrontLine\languages\cn\translation_weapon.xml
```

执行成功后会在当前目录生成一个带时间的 `CSV` 文件(注意: CSV 文件在 Excel 中直接打开无法读取中文字符, 建议新建 Excel 导入 CSV 数据源打开, 编码选择 UTF-8)

## 其他项目
- [RWR GFL mod 护甲数据提取工具](https://github.com/Kreedzt/rwr-gfl-armor-parser)

## 协议

- [GPLv3](https://opensource.org/licenses/GPL-3.0)
