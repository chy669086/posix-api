# posix-api

2024 秋冬季开源操作系统训练营四阶段项目，目标是写一系列与操作系统解耦合的 posix 接口，现阶段在实现信号量模块（好像写成用户态了）。

本项目思路是有一个基础 crate: posix-api，用来定义各种基本调用，然后由外部库来实现这些接口，操作系统就可以通过组合的形式来使用这些接口，实现接口与操作系统解耦合，同时也实现各个模块直接的解耦合。

使用了 `linkme` 作为连接的接口。