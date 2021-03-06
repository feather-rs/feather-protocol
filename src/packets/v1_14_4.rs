// This is GENERATED CODE. Please do not edit.
#![allow(warnings)]
use crate::{BytesExt, BytesMutExt, Error, ProtocolVersion, Provider};
use bytes::{Buf, BufMut, Bytes, BytesMut};
#[derive(Debug, Clone)]
pub enum ChangeGameStateReason<P: Provider> {
    InvalidBed {},
    EndRaining {},
    BeginRaining {},
    ChangeGamemode {},
    ExitEnd {},
    DemoMessage {},
    ArrowHittingPlayer {},
    FadeValue {},
    FadeTime {},
    PufferfishSting {},
    ElderGuardianAppearance {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> ChangeGameStateReason<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64, version: ProtocolVersion) -> anyhow::Result<Self> {
        match repr {
            0i64 => Ok(ChangeGameStateReason::InvalidBed {}),
            1i64 => Ok(ChangeGameStateReason::EndRaining {}),
            2i64 => Ok(ChangeGameStateReason::BeginRaining {}),
            3i64 => Ok(ChangeGameStateReason::ChangeGamemode {}),
            4i64 => Ok(ChangeGameStateReason::ExitEnd {}),
            5i64 => Ok(ChangeGameStateReason::DemoMessage {}),
            6i64 => Ok(ChangeGameStateReason::ArrowHittingPlayer {}),
            7i64 => Ok(ChangeGameStateReason::FadeValue {}),
            8i64 => Ok(ChangeGameStateReason::FadeTime {}),
            9i64 => Ok(ChangeGameStateReason::PufferfishSting {}),
            10i64 => Ok(ChangeGameStateReason::ElderGuardianAppearance {}),
            repr => Err(Error::InvalidEnumRepr(repr, "ChangeGameStateReason").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut, version: ProtocolVersion) {
        match self {
            ChangeGameStateReason::InvalidBed {} => {}
            ChangeGameStateReason::EndRaining {} => {}
            ChangeGameStateReason::BeginRaining {} => {}
            ChangeGameStateReason::ChangeGamemode {} => {}
            ChangeGameStateReason::ExitEnd {} => {}
            ChangeGameStateReason::DemoMessage {} => {}
            ChangeGameStateReason::ArrowHittingPlayer {} => {}
            ChangeGameStateReason::FadeValue {} => {}
            ChangeGameStateReason::FadeTime {} => {}
            ChangeGameStateReason::PufferfishSting {} => {}
            ChangeGameStateReason::ElderGuardianAppearance {} => {}
            ChangeGameStateReason::_Phantom(_) => {
                panic!("phantom in {} not allowed", "ChangeGameStateReason")
            }
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            ChangeGameStateReason::InvalidBed { .. } => 0i64,
            ChangeGameStateReason::EndRaining { .. } => 1i64,
            ChangeGameStateReason::BeginRaining { .. } => 2i64,
            ChangeGameStateReason::ChangeGamemode { .. } => 3i64,
            ChangeGameStateReason::ExitEnd { .. } => 4i64,
            ChangeGameStateReason::DemoMessage { .. } => 5i64,
            ChangeGameStateReason::ArrowHittingPlayer { .. } => 6i64,
            ChangeGameStateReason::FadeValue { .. } => 7i64,
            ChangeGameStateReason::FadeTime { .. } => 8i64,
            ChangeGameStateReason::PufferfishSting { .. } => 9i64,
            ChangeGameStateReason::ElderGuardianAppearance { .. } => 10i64,
            ChangeGameStateReason::_Phantom(_) => {
                panic!("phantom in {} not allowed", "ChangeGameStateReason")
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum UpdateBlockEntityAction<P: Provider> {
    SetMobSpawnerData {},
    SetCommandBlockText {},
    UpdateBeacon {},
    SetMobHead {},
    Conduit {},
    UpdateBanner {},
    SetStructureData {},
    SetGatewayDestination {},
    SetSignText {},
    DeclareBed {},
    SetJigsawBlockData {},
    SetCampfireItems {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> UpdateBlockEntityAction<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64, version: ProtocolVersion) -> anyhow::Result<Self> {
        match repr {
            1i64 => Ok(UpdateBlockEntityAction::SetMobSpawnerData {}),
            2i64 => Ok(UpdateBlockEntityAction::SetCommandBlockText {}),
            3i64 => Ok(UpdateBlockEntityAction::UpdateBeacon {}),
            4i64 => Ok(UpdateBlockEntityAction::SetMobHead {}),
            5i64 => Ok(UpdateBlockEntityAction::Conduit {}),
            6i64 => Ok(UpdateBlockEntityAction::UpdateBanner {}),
            7i64 => Ok(UpdateBlockEntityAction::SetStructureData {}),
            8i64 => Ok(UpdateBlockEntityAction::SetGatewayDestination {}),
            9i64 => Ok(UpdateBlockEntityAction::SetSignText {}),
            11i64 => Ok(UpdateBlockEntityAction::DeclareBed {}),
            12i64 => Ok(UpdateBlockEntityAction::SetJigsawBlockData {}),
            13i64 => Ok(UpdateBlockEntityAction::SetCampfireItems {}),
            repr => Err(Error::InvalidEnumRepr(repr, "UpdateBlockEntityAction").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut, version: ProtocolVersion) {
        match self {
            UpdateBlockEntityAction::SetMobSpawnerData {} => {}
            UpdateBlockEntityAction::SetCommandBlockText {} => {}
            UpdateBlockEntityAction::UpdateBeacon {} => {}
            UpdateBlockEntityAction::SetMobHead {} => {}
            UpdateBlockEntityAction::Conduit {} => {}
            UpdateBlockEntityAction::UpdateBanner {} => {}
            UpdateBlockEntityAction::SetStructureData {} => {}
            UpdateBlockEntityAction::SetGatewayDestination {} => {}
            UpdateBlockEntityAction::SetSignText {} => {}
            UpdateBlockEntityAction::DeclareBed {} => {}
            UpdateBlockEntityAction::SetJigsawBlockData {} => {}
            UpdateBlockEntityAction::SetCampfireItems {} => {}
            UpdateBlockEntityAction::_Phantom(_) => {
                panic!("phantom in {} not allowed", "UpdateBlockEntityAction")
            }
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            UpdateBlockEntityAction::SetMobSpawnerData { .. } => 1i64,
            UpdateBlockEntityAction::SetCommandBlockText { .. } => 2i64,
            UpdateBlockEntityAction::UpdateBeacon { .. } => 3i64,
            UpdateBlockEntityAction::SetMobHead { .. } => 4i64,
            UpdateBlockEntityAction::Conduit { .. } => 5i64,
            UpdateBlockEntityAction::UpdateBanner { .. } => 6i64,
            UpdateBlockEntityAction::SetStructureData { .. } => 7i64,
            UpdateBlockEntityAction::SetGatewayDestination { .. } => 8i64,
            UpdateBlockEntityAction::SetSignText { .. } => 9i64,
            UpdateBlockEntityAction::DeclareBed { .. } => 11i64,
            UpdateBlockEntityAction::SetJigsawBlockData { .. } => 12i64,
            UpdateBlockEntityAction::SetCampfireItems { .. } => 13i64,
            UpdateBlockEntityAction::_Phantom(_) => {
                panic!("phantom in {} not allowed", "UpdateBlockEntityAction")
            }
        }
    }
}
use crate::{BlockPosition, DynPacket, Node, Packet, PacketReader, Slot};
#[derive(Debug, Clone)]
pub struct JoinGame {
    pub player_eid: i32,
    pub gamemode: u8,
    pub dimension: i32,
    pub max_players: u8,
    pub level_type: String,
    pub view_distance: i32,
    pub reduced_debug_info: bool,
}
impl Packet for JoinGame {
    fn id(&self, version: ProtocolVersion) -> u32 {
        match version {
            ProtocolVersion::V1_14_4 => 37u32,
            x => panic!(
                "unsupported protocol version {:?} for packet {} defined for version {:?}",
                x,
                "JoinGame",
                ProtocolVersion::V1_14_4
            ),
        }
    }
    fn name(&self) -> &'static str {
        "JoinGame"
    }
    fn write_to(self, buf: &mut BytesMut, version: ProtocolVersion) {
        let player_eid = self.player_eid;
        buf.put_i32(player_eid as i32);
        let gamemode = self.gamemode;
        buf.put_u8(gamemode as u8);
        let dimension = self.dimension;
        buf.put_i32(dimension as i32);
        let max_players = self.max_players;
        buf.put_u8(max_players as u8);
        let level_type = self.level_type;
        buf.put_string(level_type);
        let view_distance = self.view_distance;
        buf.put_i32(view_distance as i32);
        let reduced_debug_info = self.reduced_debug_info;
        buf.put_bool(reduced_debug_info as bool);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct JoinGameReader;
impl Default for JoinGameReader {
    fn default() -> Self {
        JoinGameReader
    }
}
impl PacketReader for JoinGameReader {
    fn read_from(buf: &mut Bytes, version: ProtocolVersion) -> anyhow::Result<DynPacket> {
        let player_eid = buf.try_get_i32()?;
        let gamemode = buf.try_get_u8()?;
        let dimension = buf.try_get_i32()?;
        let max_players = buf.try_get_u8()?;
        let level_type = buf.try_get_string()?;
        let view_distance = buf.try_get_i32()?;
        let reduced_debug_info = buf.try_get_bool()?;
        let packet = JoinGame {
            player_eid,
            gamemode,
            dimension,
            max_players,
            level_type,
            view_distance,
            reduced_debug_info,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct ChangeGameState<P: Provider> {
    pub reason: ChangeGameStateReason<P>,
    pub value: f32,
}
impl<P> Packet for ChangeGameState<P>
where
    P: Provider,
{
    fn id(&self, version: ProtocolVersion) -> u32 {
        match version {
            ProtocolVersion::V1_14_4 => 30u32,
            x => panic!(
                "unsupported protocol version {:?} for packet {} defined for version {:?}",
                x,
                "ChangeGameState",
                ProtocolVersion::V1_14_4
            ),
        }
    }
    fn name(&self) -> &'static str {
        "ChangeGameState"
    }
    fn write_to(self, buf: &mut BytesMut, version: ProtocolVersion) {
        let reason = self.reason;
        let reason = reason.repr();
        buf.put_u8(reason as u8);
        let value = self.value;
        buf.put_f32(value as f32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ChangeGameStateReader<P>(std::marker::PhantomData<P>);
impl<P> Default for ChangeGameStateReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for ChangeGameStateReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes, version: ProtocolVersion) -> anyhow::Result<DynPacket> {
        let reason = buf.try_get_u8()?;
        let reason = ChangeGameStateReason::read_from(buf, reason as i64, version)?;
        let value = buf.try_get_f32()?;
        let packet = ChangeGameState::<P> { reason, value };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct UpdateBlockEntity<P: Provider> {
    pub os: BlockPosition,
    pub action: UpdateBlockEntityAction<P>,
    pub data: nbt::Blob,
}
impl<P> Packet for UpdateBlockEntity<P>
where
    P: Provider,
{
    fn id(&self, version: ProtocolVersion) -> u32 {
        match version {
            ProtocolVersion::V1_14_4 => 9u32,
            x => panic!(
                "unsupported protocol version {:?} for packet {} defined for version {:?}",
                x,
                "UpdateBlockEntity",
                ProtocolVersion::V1_14_4
            ),
        }
    }
    fn name(&self) -> &'static str {
        "UpdateBlockEntity"
    }
    fn write_to(self, buf: &mut BytesMut, version: ProtocolVersion) {
        let os = self.os;
        buf.put_position(os);
        let action = self.action;
        let action = action.repr();
        buf.put_u8(action as u8);
        let data = self.data;
        buf.put_nbt(data);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UpdateBlockEntityReader<P>(std::marker::PhantomData<P>);
impl<P> Default for UpdateBlockEntityReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for UpdateBlockEntityReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes, version: ProtocolVersion) -> anyhow::Result<DynPacket> {
        let os = buf.try_get_position()?;
        let action = buf.try_get_u8()?;
        let action = UpdateBlockEntityAction::read_from(buf, action as i64, version)?;
        let data = buf.try_get_nbt()?;
        let packet = UpdateBlockEntity::<P> { os, action, data };
        Ok(smallbox::smallbox!(packet))
    }
}
