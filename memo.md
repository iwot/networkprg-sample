# 第一回　ネットワークソフトウェアのしくみを知ろう

ソフトウェアデザイン ２０２１年１月号から。

# 01

TCPエコーサーバーの実装。

TCPクライアントの実装。

# 02

pnetを使ってIPv4パケットを送信するプログラムの実装。

IPパケットを直接やり取りする（インターネットレイヤ以下の操作を扱う）。

```
ip link show

3: wlp4s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP mode DORMANT group default qlen 1000
```

```
sudo ./target/debug/ip wlp4s0 127.0.0.1
```
 

