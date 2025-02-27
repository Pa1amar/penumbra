use ibc::core::{
    ics02_client::client_state::ClientState,
    ics03_connection::connection::ConnectionEnd,
    ics03_connection::connection::Counterparty,
    ics04_channel::{channel::ChannelEnd, packet::Packet},
    ics24_host::identifier::ClientId,
    ics24_host::identifier::{ChannelId, ConnectionId, PortId},
};
// TODO(erwan): generalize this
use ibc::clients::ics07_tendermint as tm;
use tendermint::abci::{Event, EventAttributeIndexExt};

pub fn create_client(client_id: ClientId, client_state: tm::client_state::ClientState) -> Event {
    Event::new(
        "create_client",
        vec![
            ("client_id", client_id.to_string()).index(),
            ("client_type", client_state.client_type().to_string()).index(),
            ("consensus_height", client_state.latest_height().to_string()).index(),
        ],
    )
}

pub fn update_client(
    client_id: ClientId,
    client_state: tm::client_state::ClientState,
    header: tm::header::Header,
) -> Event {
    Event::new(
        "update_client",
        vec![
            ("client_id", client_id.to_string()).index(),
            ("client_type", client_state.client_type().to_string()).index(),
            ("consensus_height", header.height().to_string()).index(),
            ("header", header.to_string()).index(),
        ],
    )
}

pub fn connection_open_init(
    connection_id: &ConnectionId,
    client_id: &ClientId,
    counterparty: &Counterparty,
) -> Event {
    Event::new(
        "connection_open_init",
        vec![
            ("connection_id", connection_id.to_string()).index(),
            ("client_id", client_id.to_string()).index(),
            (
                "counterparty_client_id",
                counterparty.client_id().to_string(),
            )
                .index(),
            (
                "counterparty_connection_id",
                counterparty
                    .connection_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
        ],
    )
}

pub fn connection_open_try(
    connection_id: &ConnectionId,
    client_id: &ClientId,
    counterparty: &Counterparty,
) -> Event {
    Event::new(
        "connection_open_try",
        vec![
            ("connection_id", connection_id.to_string()).index(),
            ("client_id", client_id.to_string()).index(),
            (
                "counterparty_client_id",
                counterparty.client_id().to_string(),
            )
                .index(),
            (
                "counterparty_connection_id",
                counterparty
                    .connection_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
        ],
    )
}

pub fn connection_open_ack(connection_id: &ConnectionId, connection_end: &ConnectionEnd) -> Event {
    Event::new(
        "connection_open_ack",
        vec![
            ("connection_id", connection_id.to_string()).index(),
            ("client_id", connection_end.client_id().to_string()).index(),
            (
                "counterparty_client_id",
                connection_end.counterparty().client_id().to_string(),
            )
                .index(),
            (
                "counterparty_connection_id",
                connection_end
                    .counterparty()
                    .connection_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
        ],
    )
}

pub fn connection_open_confirm(
    connection_id: &ConnectionId,
    connection_end: &ConnectionEnd,
) -> Event {
    Event::new(
        "connection_open_confirm",
        vec![
            ("connection_id", connection_id.to_string()).index(),
            ("client_id", connection_end.client_id().to_string()).index(),
            (
                "counterparty_client_id",
                connection_end.counterparty().client_id().to_string(),
            )
                .index(),
            (
                "counterparty_connection_id",
                connection_end
                    .counterparty()
                    .connection_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
        ],
    )
}

pub fn channel_open_init(port_id: &PortId, channel_id: &ChannelId, channel: &ChannelEnd) -> Event {
    Event::new(
        "channel_open_init",
        vec![
            ("port_id", port_id.to_string()).index(),
            ("channel_id", channel_id.to_string()).index(),
            (
                "counterparty_port_id",
                channel.counterparty().port_id().to_string(),
            )
                .index(),
            (
                "counterparty_channel_id",
                channel
                    .counterparty()
                    .channel_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
            ("connection_id", channel.connection_hops[0].to_string()).index(),
        ],
    )
}

pub fn channel_open_try(port_id: &PortId, channel_id: &ChannelId, channel: &ChannelEnd) -> Event {
    Event::new(
        "channel_open_try",
        vec![
            ("port_id", port_id.to_string()).index(),
            ("channel_id", channel_id.to_string()).index(),
            (
                "counterparty_port_id",
                channel.counterparty().port_id().to_string(),
            )
                .index(),
            (
                "counterparty_channel_id",
                channel
                    .counterparty()
                    .channel_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
            ("connection_id", channel.connection_hops[0].to_string()).index(),
        ],
    )
}

pub fn channel_open_ack(port_id: &PortId, channel_id: &ChannelId, channel: &ChannelEnd) -> Event {
    Event::new(
        "channel_open_ack",
        vec![
            ("port_id", port_id.to_string()).index(),
            ("channel_id", channel_id.to_string()).index(),
            (
                "counterparty_port_id",
                channel.counterparty().port_id().to_string(),
            )
                .index(),
            (
                "counterparty_channel_id",
                channel
                    .counterparty()
                    .channel_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
            ("connection_id", channel.connection_hops[0].to_string()).index(),
        ],
    )
}

pub fn channel_open_confirm(
    port_id: &PortId,
    channel_id: &ChannelId,
    channel: &ChannelEnd,
) -> Event {
    Event::new(
        "channel_open_confirm",
        vec![
            ("port_id", port_id.to_string()).index(),
            ("channel_id", channel_id.to_string()).index(),
            (
                "counterparty_port_id",
                channel.counterparty().port_id().to_string(),
            )
                .index(),
            (
                "counterparty_channel_id",
                channel
                    .counterparty()
                    .channel_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
            ("connection_id", channel.connection_hops[0].to_string()).index(),
        ],
    )
}

pub fn channel_close_init(port_id: &PortId, channel_id: &ChannelId, channel: &ChannelEnd) -> Event {
    Event::new(
        "channel_close_init",
        vec![
            ("port_id", port_id.to_string()).index(),
            ("channel_id", channel_id.to_string()).index(),
            (
                "counterparty_port_id",
                channel.counterparty().port_id().to_string(),
            )
                .index(),
            (
                "counterparty_channel_id",
                channel
                    .counterparty()
                    .channel_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
            ("connection_id", channel.connection_hops[0].to_string()).index(),
        ],
    )
}

pub fn channel_close_confirm(
    port_id: &PortId,
    channel_id: &ChannelId,
    channel: &ChannelEnd,
) -> Event {
    Event::new(
        "channel_close_confirm",
        vec![
            ("port_id", port_id.to_string()).index(),
            ("channel_id", channel_id.to_string()).index(),
            (
                "counterparty_port_id",
                channel.counterparty().port_id().to_string(),
            )
                .index(),
            (
                "counterparty_channel_id",
                channel
                    .counterparty()
                    .channel_id()
                    .map(|id| id.to_string())
                    .unwrap_or_default(),
            )
                .index(),
            ("connection_id", channel.connection_hops[0].to_string()).index(),
        ],
    )
}

// TODO: add packet send

pub fn receive_packet(packet: &Packet, channel: &ChannelEnd) -> Event {
    Event::new(
        "recv_packet",
        vec![
            ("packet_data_hex", hex::encode(packet.data.clone())).index(),
            (
                "packet_timeout_height",
                packet.timeout_height_on_b.to_string(),
            )
                .index(),
            (
                "packet_timeout_timestamp",
                packet.timeout_timestamp_on_b.to_string(),
            )
                .index(),
            ("packet_sequence", packet.sequence.to_string()).index(),
            ("packet_src_port", packet.port_on_a.to_string()).index(),
            ("packet_src_channel", packet.chan_on_a.to_string()).index(),
            ("packet_dst_port", packet.port_on_b.to_string()).index(),
            ("packet_dst_channel", packet.chan_on_b.to_string()).index(),
            ("packet_channel_ordering", channel.ordering.to_string()).index(),
            ("packet_connection", channel.connection_hops[0].to_string()).index(),
        ],
    )
}

pub fn acknowledge_packet(packet: &Packet, channel: &ChannelEnd) -> Event {
    Event::new(
        "acknowledge_packet",
        vec![
            ("packet_data_hex", hex::encode(packet.data.clone())).index(),
            (
                "packet_timeout_height",
                packet.timeout_height_on_b.to_string(),
            )
                .index(),
            (
                "packet_timeout_timestamp",
                packet.timeout_timestamp_on_b.to_string(),
            )
                .index(),
            ("packet_sequence", packet.sequence.to_string()).index(),
            ("packet_src_port", packet.port_on_a.to_string()).index(),
            ("packet_src_channel", packet.chan_on_a.to_string()).index(),
            ("packet_dst_port", packet.port_on_b.to_string()).index(),
            ("packet_dst_channel", packet.chan_on_b.to_string()).index(),
            ("packet_channel_ordering", channel.ordering.to_string()).index(),
            ("packet_connection", channel.connection_hops[0].to_string()).index(),
        ],
    )
}

pub fn timeout_packet(packet: &Packet, channel: &ChannelEnd) -> Event {
    Event::new(
        "timeout_packet",
        vec![
            ("packet_data_hex", hex::encode(packet.data.clone())).index(),
            (
                "packet_timeout_height",
                packet.timeout_height_on_b.to_string(),
            )
                .index(),
            (
                "packet_timeout_timestamp",
                packet.timeout_timestamp_on_b.to_string(),
            )
                .index(),
            ("packet_sequence", packet.sequence.to_string()).index(),
            ("packet_src_port", packet.port_on_a.to_string()).index(),
            ("packet_src_channel", packet.chan_on_a.to_string()).index(),
            ("packet_dst_port", packet.port_on_b.to_string()).index(),
            ("packet_dst_channel", packet.chan_on_b.to_string()).index(),
            ("packet_channel_ordering", channel.ordering.to_string()).index(),
            ("packet_connection", channel.connection_hops[0].to_string()).index(),
        ],
    )
}
