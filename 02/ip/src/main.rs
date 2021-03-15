use pnet::{
    datalink::{self, Channel, ChannelType, Config},
    packet::{
        ip::IpNextHeaderProtocol,
        ipv4::MutableIpv4Packet,
        Packet,
    },
    util,
};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // IPを表す識別子（EtherType）
    const INTERNET_PROTOCOL: u16 = 0x0800;
    // テスト用プロトコルを表すプロトコル番号
    const TESTING: u8 = 0xFE;
    let args: Vec<String> = env::args().collect();
    // ネットワークインターフェイス名
    let if_name = &args[1];
    // 宛先IPアドレス
    let destination = args[2].parse()?;

    // ネットワークインターフェイスの取得
    let iface = datalink::interfaces()
        .into_iter()
        .find(|i| i.name == *if_name)
        .ok_or("failed to get interface")?;
    
    let config = Default::default();
    // ソケット生成
    let (mut sender, _reciever) = match datalink::channel(
        &iface,
        Config {
            channel_type: ChannelType::Layer3(INTERNET_PROTOCOL),
            ..config
        },
    ) {
        Ok(Channel::Ethernet(sender, reciever)) => (sender, reciever),
        _ => return Err("failed to open channel".into()),
    };

    let mut ip_buf = [0; 1500];
    let mut packet = MutableIpv4Packet::new(&mut ip_buf).ok_or("failed to create packet")?;

    // IPv4
    packet.set_version(4);
    // ヘッダ長。単位は４オクテット（32ビット）。今回のヘッダは20オクテットなので5。
    packet.set_header_length(5);
    // IPパケットのペイロードを含めた全長（オクテット）。
    packet.set_total_length(1500);
    // パケットのID。
    packet.set_identification(0xabcd);
    // Time to Live, ルーターを越えるたびに減っていく。パケットの無限ループ防止。
    packet.set_ttl(64);
    // IPの上位レイヤのプロトコル。今回はテスト用に予約された値。
    packet.set_next_level_protocol(IpNextHeaderProtocol::new(TESTING));
    // 送信元IPアドレス。
    packet.set_source("127.0.0.1".parse()?);
    // 宛先IPアドレス。
    packet.set_destination(destination);
    // チェックサム。パケットの破損検出用。
    packet.set_checksum(util::checksum(&packet.packet(), 5));
    sender
        .send_to(packet.packet(), None)
        .ok_or("failed to send")??;
    
    Ok(())
}
