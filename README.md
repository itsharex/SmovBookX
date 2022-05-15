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

下阶段计划：

1.SmovItem 

2.SmovView

3.增加不存在的文件自动删除功能

4.增加回收站删除功能

5.合并视频文件提示

6.自动更新功能 codeing提供 下载软件 服务器提供返回文本

晚上更新一个切换关闭列表 切换为可用的 功能 0.1.6版本


打包方式更改为签名 

全局变量修改为 env-cmd

npm install -g env-cmd 安装


删除数据时需要同时删除检索队列的数据

周末写 设置界面

发现一个问题 就是 检索文件夹的时候 因为 整理文件夹不在检索范围内 所以会导致把已经检索的文件 判断为未检索 所以当我在新增检索文件夹时应该把 默认的整理文件夹给加进去






hfs功能

1. 配置文件可自定义 路径为Roaming\smovbook
  1) app运行时 直接缓存一个config 这个config模板可以直接从 rocket 做一个默认的 当文件不存在时 新增一个默认的config 然后存储到这个目录下 ？是否可以直接使用config 这个类 build 还是只能用 配置文件 是否会造成性能的损耗 
2. 状态返回
  1) 在运行时需要对文件服务器做一个多次返回状态 当成功时 应该直接给一个启动成功的状态返回
  2) 在运行失败时 直接返回错误 （已完成）
3. 对hfs文件服务器状态缓存
  1) 缓存到哪里 应该缓存到app 因为当用户使用时hfs文件服务器不一定已经启动了 ？当文件服务器未启动 我是否能得到一个config 如何能得到 我能否做到自由修改 和重启
  2) 服务器启动状态的缓存 ？如何获得hfs的启动状态 是用lazy去缓存一个状态还是说 我能直接得到这个线程的运行状态 或是是否存在 ？
4. hfs文件服务器重启
  1) rocket的关闭在默认有一个 Shutdown 的方法 或许可以通过http的方式关闭？ 是否可以通过 commond的方式调用
5. hfs文件服务器的性能问题
  1) 文件服务器是暂停比较好 还是每次新增一个线程比较好 ？如何删除这个线程 删除线程是否会导致当前正在使用的资源出现问题？ 


  当我请求后台启动服务器时 这个任务貌似会一直处于一个阻塞的状态 应为他应当返回一个结果

  文件服务器的返回 应该都是要做多次返回的类型！！ 不能用单次返回 会有问题 可能会有侦测不到的问题

  是否需要两个文件？ 好像只设置一个文件就可以了 反正可以自定义




5.12 记录
当前要做的 
1.视频文件的在线播放
2.基于toml 做一个解析 html 而不用修改代码的工具





[default]
address = "127.0.0.1"
port = 8000
workers = 16
keep_alive = 5
ident = "Rocket"
log_level = "normal"
temp_dir = ""
cli_colors = true

[default.limits]
forms = "64 kB"
json = "1 MiB"
msgpack = "2 MiB"
"file/jpg" = "5 MiB"

[default.shutdown]
ctrlc = true
signals = ["term", "hup"]
grace = 5
mercy = 5