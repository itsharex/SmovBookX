# SmovBook


文件整理流程 -> 获取文件元数据 -> 判断是否为单文件 判断标准为目录下是否存在其他视频文件 -> 更新文件名为name -> 更新目录下可能存在的文件和字幕文件为name -> 迁移文件夹位置 

当为单文件时 新建一个名为name的文件夹

在网络检索时 当取到这个视频确实有检索到的数据 就要对文件位置进行处理了


需要增加代理池功能，通过代理访问


FFmpeg concat 分离器

这种方法成功率很高，也是最好的，但是需要 FFmpeg 1.1 以上版本。先创建一个文本文件 filelist.txt：

file 'input1.mkv'

file 'input2.mkv'

file 'input3.mkv'

然后：

ffmpeg -f concat -i filelist.txt -c copy output.mkv

注意：使用 FFmpeg concat 分离器时，如果文件名有奇怪的字符，要在 filelist.txt 中转义


看来需要对大量错误的option写法做返回判断了。。。。

