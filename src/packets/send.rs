#![allow(warnings)]
use super::*;
use crate::{BytesExt, BytesMutExt, Error, ProtocolVersion, Provider};
use bytes::{Buf, BufMut, Bytes, BytesMut};
pub struct DeclareCommands {
    pub nodes: Vec<Node>,
    pub root_index: i32,
}
impl UniversalPacketSend for DeclareCommands {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::DeclareCommands {
                nodes: self.nodes,
                root_index: self.root_index,
            })),
            _ => None,
        }
    }
}
pub struct SpawnMob {
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
    pub entity: i32,
    pub head_pitch: u8,
    pub pitch: u8,
    pub ty: i32,
    pub uuid: uuid::Uuid,
    pub x: f64,
    pub y: f64,
    pub yaw: u8,
    pub z: f64,
}
impl UniversalPacketSend for SpawnMob {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::SpawnMob {
                entity: self.entity,
                uuid: self.uuid,
                ty: self.ty,
                x: self.x,
                y: self.y,
                z: self.z,
                yaw: self.yaw,
                pitch: self.pitch,
                head_pitch: self.head_pitch,
                dx: self.dx,
                dy: self.dy,
                dz: self.dz,
            })),
            _ => None,
        }
    }
}
pub struct SpawnPainting<P: Provider> {
    pub direction: SpawnPaintingDirection<P>,
    pub entity: i32,
    pub motive: i32,
    pub position: BlockPosition,
    pub uuid: uuid::Uuid,
}
impl<P> UniversalPacketSend for SpawnPainting<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::SpawnPainting {
                entity: self.entity,
                uuid: self.uuid,
                motive: self.motive,
                position: self.position,
                direction: self.direction,
            })),
            _ => None,
        }
    }
}
pub struct BlockAction<P: Provider> {
    pub action_id: u8,
    pub action_param: u8,
    pub block_type: P::Block,
    pub position: BlockPosition,
}
impl<P> UniversalPacketSend for BlockAction<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::BlockAction {
                position: self.position,
                action_id: self.action_id,
                action_param: self.action_param,
                block_type: self.block_type,
            })),
            _ => None,
        }
    }
}
pub struct EntityAnimation<P: Provider> {
    pub animation: EntityAnimationType<P>,
    pub entity: i32,
}
impl<P> UniversalPacketSend for EntityAnimation<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::EntityAnimation {
                entity: self.entity,
                animation: self.animation,
            })),
            _ => None,
        }
    }
}
pub struct ServerDifficulty {
    pub difficulty: u8,
    pub difficulty_locked: bool,
}
impl UniversalPacketSend for ServerDifficulty {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::ServerDifficulty {
                difficulty: self.difficulty,
                difficulty_locked: self.difficulty_locked,
            })),
            _ => None,
        }
    }
}
pub struct AcknowledgePlayerDigging<P: Provider> {
    pub block: P::Block,
    pub position: BlockPosition,
    pub status: AcknowledgePlayerDiggingStatus<P>,
    pub successful: bool,
}
impl<P> UniversalPacketSend for AcknowledgePlayerDigging<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => {
                Some(smallbox::smallbox!(v1_15_2::AcknowledgePlayerDigging {
                    position: self.position,
                    block: self.block,
                    status: self.status,
                    successful: self.successful,
                }))
            }
            _ => None,
        }
    }
}
pub struct TabComplete<P: Provider> {
    pub length: i32,
    pub matches: Vec<TabCompleteMatch<P>>,
    pub start: i32,
    pub transaction_id: i32,
}
impl<P> UniversalPacketSend for TabComplete<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::TabComplete {
                transaction_id: self.transaction_id,
                start: self.start,
                length: self.length,
                matches: self.matches,
            })),
            _ => None,
        }
    }
}
pub struct BlockBreakAnimation {
    pub breaker: i32,
    pub destroy_stage: u8,
    pub position: BlockPosition,
}
impl UniversalPacketSend for BlockBreakAnimation {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::BlockBreakAnimation {
                breaker: self.breaker,
                position: self.position,
                destroy_stage: self.destroy_stage,
            })),
            _ => None,
        }
    }
}
pub struct UnloadChunk {
    pub x: i32,
    pub z: i32,
}
impl UniversalPacketSend for UnloadChunk {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::UnloadChunk {
                x: self.x,
                z: self.z,
            })),
            _ => None,
        }
    }
}
pub struct EntityRotation {
    pub entity: i32,
    pub on_ground: bool,
    pub pitch: u8,
    pub yaw: u8,
}
impl UniversalPacketSend for EntityRotation {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::EntityRotation {
                entity: self.entity,
                yaw: self.yaw,
                pitch: self.pitch,
                on_ground: self.on_ground,
            })),
            _ => None,
        }
    }
}
pub struct OpenBook {
    pub hand: i32,
}
impl UniversalPacketSend for OpenBook {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => {
                Some(smallbox::smallbox!(v1_15_2::OpenBook { hand: self.hand }))
            }
            _ => None,
        }
    }
}
pub struct SetSlot {
    pub slot: Slot,
    pub slot_index: i16,
    pub window_id: u8,
}
impl UniversalPacketSend for SetSlot {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::SetSlot {
                window_id: self.window_id,
                slot_index: self.slot_index,
                slot: self.slot,
            })),
            _ => None,
        }
    }
}
pub struct CloseWindow {
    pub window_id: u8,
}
impl UniversalPacketSend for CloseWindow {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::CloseWindow {
                window_id: self.window_id,
            })),
            _ => None,
        }
    }
}
pub struct EntityMovement {
    pub entity: i32,
}
impl UniversalPacketSend for EntityMovement {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::EntityMovement {
                entity: self.entity,
            })),
            _ => None,
        }
    }
}
pub struct PlayerAbilities {
    pub flags: i8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}
impl UniversalPacketSend for PlayerAbilities {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::PlayerAbilities {
                flags: self.flags,
                flying_speed: self.flying_speed,
                fov_modifier: self.fov_modifier,
            })),
            _ => None,
        }
    }
}
pub struct SetCooldown<P: Provider> {
    pub cooldown_ticks: i32,
    pub item: P::Item,
}
impl<P> UniversalPacketSend for SetCooldown<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::SetCooldown {
                item: self.item,
                cooldown_ticks: self.cooldown_ticks,
            })),
            _ => None,
        }
    }
}
pub struct OpenHorseWindow {
    pub entity: i32,
    pub num_slots: i32,
    pub window_id: u8,
}
impl UniversalPacketSend for OpenHorseWindow {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::OpenHorseWindow {
                window_id: self.window_id,
                num_slots: self.num_slots,
                entity: self.entity,
            })),
            _ => None,
        }
    }
}
pub struct EntityPositionAndRotation {
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
    pub entity: i32,
    pub on_ground: bool,
    pub pitch: u8,
    pub yaw: u8,
}
impl UniversalPacketSend for EntityPositionAndRotation {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => {
                Some(smallbox::smallbox!(v1_15_2::EntityPositionAndRotation {
                    entity: self.entity,
                    dx: self.dx,
                    dy: self.dy,
                    dz: self.dz,
                    yaw: self.yaw,
                    pitch: self.pitch,
                    on_ground: self.on_ground,
                }))
            }
            _ => None,
        }
    }
}
pub struct Statistics<P: Provider> {
    pub statistics: Vec<Statistic<P>>,
    pub value: i32,
}
impl<P> UniversalPacketSend for Statistics<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::Statistics {
                statistics: self.statistics,
                value: self.value,
            })),
            _ => None,
        }
    }
}
pub struct BossBar<P: Provider> {
    pub data: BossBarData<P>,
    pub uuid: uuid::Uuid,
}
impl<P> UniversalPacketSend for BossBar<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::BossBar {
                uuid: self.uuid,
                data: self.data,
            })),
            _ => None,
        }
    }
}
pub struct SpawnPlayer {
    pub entity: i32,
    pub pitch: u8,
    pub uuid: uuid::Uuid,
    pub x: f64,
    pub y: f64,
    pub yaw: u8,
    pub z: f64,
}
impl UniversalPacketSend for SpawnPlayer {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::SpawnPlayer {
                entity: self.entity,
                uuid: self.uuid,
                x: self.x,
                y: self.y,
                z: self.z,
                yaw: self.yaw,
                pitch: self.pitch,
            })),
            _ => None,
        }
    }
}
pub struct OpenWindow {
    pub title: String,
    pub ty: i32,
    pub window_id: i32,
}
impl UniversalPacketSend for OpenWindow {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::OpenWindow {
                window_id: self.window_id,
                ty: self.ty,
                title: self.title,
            })),
            _ => None,
        }
    }
}
pub struct Explosion<P: Provider> {
    pub player_dx: f32,
    pub player_dy: f32,
    pub player_dz: f32,
    pub records: Vec<ExplosionRecord<P>>,
    pub strength: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl<P> UniversalPacketSend for Explosion<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::Explosion {
                x: self.x,
                y: self.y,
                z: self.z,
                strength: self.strength,
                records: self.records,
                player_dx: self.player_dx,
                player_dy: self.player_dy,
                player_dz: self.player_dz,
            })),
            _ => None,
        }
    }
}
pub struct CraftRecipeResponse {
    pub recipe: String,
    pub window_id: u8,
}
impl UniversalPacketSend for CraftRecipeResponse {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::CraftRecipeResponse {
                window_id: self.window_id,
                recipe: self.recipe,
            })),
            _ => None,
        }
    }
}
pub struct SpawnObject {
    pub data: i32,
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
    pub entity: i32,
    pub pitch: u8,
    pub ty: i32,
    pub uuid: uuid::Uuid,
    pub x: f64,
    pub y: f64,
    pub yaw: u8,
    pub z: f64,
}
impl UniversalPacketSend for SpawnObject {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::SpawnObject {
                entity: self.entity,
                uuid: self.uuid,
                ty: self.ty,
                x: self.x,
                y: self.y,
                z: self.z,
                pitch: self.pitch,
                yaw: self.yaw,
                data: self.data,
                dx: self.dx,
                dy: self.dy,
                dz: self.dz,
            })),
            _ => None,
        }
    }
}
pub struct undefined {}
impl UniversalPacketSend for undefined {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_14_4 => Some(smallbox::smallbox!(v1_14_4::undefined {})),
            ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_2 => {
                Some(smallbox::smallbox!(v1_15_2::undefined {}))
            }
            ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_2 => {
                Some(smallbox::smallbox!(v1_15_2::undefined {}))
            }
            ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_2 => {
                Some(smallbox::smallbox!(v1_15_2::undefined {}))
            }
            _ => None,
        }
    }
}
pub struct ChangeGameState<P: Provider> {
    pub reason: ChangeGameStateReason<P>,
    pub value: f32,
}
impl<P> UniversalPacketSend for ChangeGameState<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_14_4 => Some(smallbox::smallbox!(v1_14_4::ChangeGameState {
                reason: self.reason,
                value: self.value,
            })),
            ProtocolVersion::V1_15_1 | ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_0 => {
                Some(smallbox::smallbox!(v1_15_2::ChangeGameState {
                    reason: self.reason,
                    value: self.value,
                }))
            }
            _ => None,
        }
    }
}
pub struct PluginMessage {
    pub channel: String,
    pub data: Vec<i8>,
}
impl UniversalPacketSend for PluginMessage {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::PluginMessage {
                channel: self.channel,
                data: self.data,
            })),
            _ => None,
        }
    }
}
pub struct EntityPosition {
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
    pub entity: i32,
    pub on_ground: bool,
}
impl UniversalPacketSend for EntityPosition {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::EntityPosition {
                entity: self.entity,
                dx: self.dx,
                dy: self.dy,
                dz: self.dz,
                on_ground: self.on_ground,
            })),
            _ => None,
        }
    }
}
pub struct MultiBlockChange<P: Provider> {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub records: Vec<MultiBlockChangeRecord<P>>,
}
impl<P> UniversalPacketSend for MultiBlockChange<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::MultiBlockChange {
                chunk_x: self.chunk_x,
                chunk_z: self.chunk_z,
                records: self.records,
            })),
            _ => None,
        }
    }
}
pub struct JoinGame {
    pub dimension: i32,
    pub enable_respawn_screen: bool,
    pub gamemode: u8,
    pub hashed_seed: i64,
    pub level_type: String,
    pub max_players: u8,
    pub player_eid: i32,
    pub reduced_debug_info: bool,
    pub view_distance: i32,
}
impl UniversalPacketSend for JoinGame {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_14_4 => Some(smallbox::smallbox!(v1_14_4::JoinGame {
                player_eid: self.player_eid,
                gamemode: self.gamemode,
                dimension: self.dimension,
                max_players: self.max_players,
                level_type: self.level_type,
                view_distance: self.view_distance,
                reduced_debug_info: self.reduced_debug_info,
            })),
            ProtocolVersion::V1_15_1 | ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_0 => {
                Some(smallbox::smallbox!(v1_15_2::JoinGame {
                    player_eid: self.player_eid,
                    gamemode: self.gamemode,
                    dimension: self.dimension,
                    hashed_seed: self.hashed_seed,
                    max_players: self.max_players,
                    level_type: self.level_type,
                    view_distance: self.view_distance,
                    reduced_debug_info: self.reduced_debug_info,
                    enable_respawn_screen: self.enable_respawn_screen,
                }))
            }
            _ => None,
        }
    }
}
pub struct WindowProperty {
    pub property: i16,
    pub value: i16,
    pub window_id: u8,
}
impl UniversalPacketSend for WindowProperty {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::WindowProperty {
                window_id: self.window_id,
                property: self.property,
                value: self.value,
            })),
            _ => None,
        }
    }
}
pub struct SpawnExperienceOrb {
    pub count: i16,
    pub entity: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl UniversalPacketSend for SpawnExperienceOrb {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::SpawnExperienceOrb {
                entity: self.entity,
                x: self.x,
                y: self.y,
                z: self.z,
                count: self.count,
            })),
            _ => None,
        }
    }
}
pub struct Disconnect {
    pub reason: String,
}
impl UniversalPacketSend for Disconnect {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::Disconnect {
                reason: self.reason,
            })),
            _ => None,
        }
    }
}
pub struct Effect {
    pub data: i32,
    pub disable_relative_volume: bool,
    pub id: i32,
    pub position: BlockPosition,
}
impl UniversalPacketSend for Effect {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::Effect {
                id: self.id,
                position: self.position,
                data: self.data,
                disable_relative_volume: self.disable_relative_volume,
            })),
            _ => None,
        }
    }
}
pub struct UpdateBlockEntity<P: Provider> {
    pub action: UpdateBlockEntityAction<P>,
    pub data: nbt::Blob,
    pub os: BlockPosition,
}
impl<P> UniversalPacketSend for UpdateBlockEntity<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_14_4 => Some(smallbox::smallbox!(v1_14_4::UpdateBlockEntity {
                os: self.os,
                action: self.action,
                data: self.data,
            })),
            ProtocolVersion::V1_15_1 | ProtocolVersion::V1_15_2 | ProtocolVersion::V1_15_0 => {
                Some(smallbox::smallbox!(v1_15_2::UpdateBlockEntity {
                    os: self.os,
                    action: self.action,
                    data: self.data,
                }))
            }
            _ => None,
        }
    }
}
pub struct BlockChange<P: Provider> {
    pub block: P::Block,
    pub position: BlockPosition,
}
impl<P> UniversalPacketSend for BlockChange<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::BlockChange {
                position: self.position,
                block: self.block,
            })),
            _ => None,
        }
    }
}
pub struct ChatMessage<P: Provider> {
    pub data: String,
    pub position: ChatMessagePosition<P>,
}
impl<P> UniversalPacketSend for ChatMessage<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::ChatMessage {
                data: self.data,
                position: self.position,
            })),
            _ => None,
        }
    }
}
pub struct VehicleMove {
    pub pitch: f32,
    pub x: f64,
    pub y: f64,
    pub yaw: f32,
    pub z: f64,
}
impl UniversalPacketSend for VehicleMove {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::VehicleMove {
                x: self.x,
                y: self.y,
                z: self.z,
                yaw: self.yaw,
                pitch: self.pitch,
            })),
            _ => None,
        }
    }
}
pub struct OpenSignEditor {
    pub position: BlockPosition,
}
impl UniversalPacketSend for OpenSignEditor {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::OpenSignEditor {
                position: self.position,
            })),
            _ => None,
        }
    }
}
pub struct EntityStatus {
    pub entity: i32,
    pub status: i8,
}
impl UniversalPacketSend for EntityStatus {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::EntityStatus {
                entity: self.entity,
                status: self.status,
            })),
            _ => None,
        }
    }
}
pub struct CombatEvent<P: Provider> {
    pub event: CombatEventType<P>,
}
impl<P> UniversalPacketSend for CombatEvent<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::CombatEvent {
                event: self.event,
            })),
            _ => None,
        }
    }
}
pub struct SpawnGlobalEntity<P: Provider> {
    pub entity: i32,
    pub ty: SpawnGlobalEntityType<P>,
}
impl<P> UniversalPacketSend for SpawnGlobalEntity<P>
where
    P: Provider,
{
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::SpawnGlobalEntity {
                entity: self.entity,
                ty: self.ty,
            })),
            _ => None,
        }
    }
}
pub struct NamedSoundEffect {
    pub category: i32,
    pub name: String,
    pub pitch: f32,
    pub posx: i32,
    pub posy: i32,
    pub posz: i32,
    pub volume: f32,
}
impl UniversalPacketSend for NamedSoundEffect {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::NamedSoundEffect {
                name: self.name,
                category: self.category,
                posx: self.posx,
                posy: self.posy,
                posz: self.posz,
                volume: self.volume,
                pitch: self.pitch,
            })),
            _ => None,
        }
    }
}
pub struct KeepAlive {
    pub id: i64,
}
impl UniversalPacketSend for KeepAlive {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => {
                Some(smallbox::smallbox!(v1_15_2::KeepAlive { id: self.id }))
            }
            _ => None,
        }
    }
}
pub struct WindowConfirmation {
    pub accepted: bool,
    pub action_number: i16,
    pub window_id: u8,
}
impl UniversalPacketSend for WindowConfirmation {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::WindowConfirmation {
                window_id: self.window_id,
                action_number: self.action_number,
                accepted: self.accepted,
            })),
            _ => None,
        }
    }
}
pub struct WindowItems {
    pub slots: Vec<Slot>,
    pub window_id: u8,
}
impl UniversalPacketSend for WindowItems {
    fn try_into_version(self, version: ProtocolVersion) -> Option<DynPacket> {
        match version {
            ProtocolVersion::V1_15_1
            | ProtocolVersion::V1_14_4
            | ProtocolVersion::V1_15_2
            | ProtocolVersion::V1_15_0 => Some(smallbox::smallbox!(v1_15_2::WindowItems {
                window_id: self.window_id,
                slots: self.slots,
            })),
            _ => None,
        }
    }
}
