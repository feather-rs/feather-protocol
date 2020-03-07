// This is GENERATED CODE. Do not edit.
use crate::{BytesExt, BytesMutExt, Error, ProtocolVersion, Provider};
use bytes::{Buf, BufMut, Bytes, BytesMut};
const VERSION: ProtocolVersion = ProtocolVersion::V1_15_2;
pub enum SpawnGlobalEntityType<P: Provider> {
    Thunderbolt {},
}
impl<P> SpawnGlobalEntityType<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(1i64) => Ok(SpawnGlobalEntityType::Thunderbolt {}),
            repr => Err(Error::InvalidEnumRepr(repr, "SpawnGlobalEntityType")),
        }
    }
}
pub enum SpawnPaintingDirection<P: Provider> {
    North {},
    South {},
    West {},
    East {},
}
impl<P> SpawnPaintingDirection<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(2i64) => Ok(SpawnPaintingDirection::North {}),
            EnumRepr::Integer(0i64) => Ok(SpawnPaintingDirection::South {}),
            EnumRepr::Integer(1i64) => Ok(SpawnPaintingDirection::West {}),
            EnumRepr::Integer(3i64) => Ok(SpawnPaintingDirection::East {}),
            repr => Err(Error::InvalidEnumRepr(repr, "SpawnPaintingDirection")),
        }
    }
}
pub enum EntityAnimationType<P: Provider> {
    SwingMainArm {},
    TakeDamage {},
    LeaveBed {},
    SwingOffhand {},
    CriticalEffect {},
    MagicCriticalEffect {},
}
impl<P> EntityAnimationType<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(0i64) => Ok(EntityAnimationType::SwingMainArm {}),
            EnumRepr::Integer(1i64) => Ok(EntityAnimationType::TakeDamage {}),
            EnumRepr::Integer(2i64) => Ok(EntityAnimationType::LeaveBed {}),
            EnumRepr::Integer(3i64) => Ok(EntityAnimationType::SwingOffhand {}),
            EnumRepr::Integer(4i64) => Ok(EntityAnimationType::CriticalEffect {}),
            EnumRepr::Integer(5i64) => Ok(EntityAnimationType::MagicCriticalEffect {}),
            repr => Err(Error::InvalidEnumRepr(repr, "EntityAnimationType")),
        }
    }
}
pub struct Statistic<P: Provider> {
    category_id: i32,
    statistic_id: i32,
}
impl<P> Statistic<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
        let version = VERSION;
        let category_id = try_get_i32()?;
        let statistic_id = try_get_i32()?;
        Ok(Self {
            category_id,
            statistic_id,
        })
    }
    pub fn write_to(&self, buf: &mut BytesMut) {
        let version = VERSION;
        let category_id = self.category_id;
        buf.put_i32(category_id);
        let statistic_id = self.statistic_id;
        buf.put_i32(statistic_id);
    }
}
pub enum AcknowledgePlayerDiggingStatus<P: Provider> {
    StartedDigging {},
    CancelledDigging {},
    FinishedDigging {},
}
impl<P> AcknowledgePlayerDiggingStatus<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(0i64) => Ok(AcknowledgePlayerDiggingStatus::StartedDigging {}),
            EnumRepr::Integer(1i64) => Ok(AcknowledgePlayerDiggingStatus::CancelledDigging {}),
            EnumRepr::Integer(2i64) => Ok(AcknowledgePlayerDiggingStatus::FinishedDigging {}),
            repr => Err(Error::InvalidEnumRepr(
                repr,
                "AcknowledgePlayerDiggingStatus",
            )),
        }
    }
}
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
    SetBeehiveData {},
}
impl<P> UpdateBlockEntityAction<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(1i64) => Ok(UpdateBlockEntityAction::SetMobSpawnerData {}),
            EnumRepr::Integer(2i64) => Ok(UpdateBlockEntityAction::SetCommandBlockText {}),
            EnumRepr::Integer(3i64) => Ok(UpdateBlockEntityAction::UpdateBeacon {}),
            EnumRepr::Integer(4i64) => Ok(UpdateBlockEntityAction::SetMobHead {}),
            EnumRepr::Integer(5i64) => Ok(UpdateBlockEntityAction::Conduit {}),
            EnumRepr::Integer(6i64) => Ok(UpdateBlockEntityAction::UpdateBanner {}),
            EnumRepr::Integer(7i64) => Ok(UpdateBlockEntityAction::SetStructureData {}),
            EnumRepr::Integer(8i64) => Ok(UpdateBlockEntityAction::SetGatewayDestination {}),
            EnumRepr::Integer(9i64) => Ok(UpdateBlockEntityAction::SetSignText {}),
            EnumRepr::Integer(11i64) => Ok(UpdateBlockEntityAction::DeclareBed {}),
            EnumRepr::Integer(12i64) => Ok(UpdateBlockEntityAction::SetJigsawBlockData {}),
            EnumRepr::Integer(13i64) => Ok(UpdateBlockEntityAction::SetCampfireItems {}),
            EnumRepr::Integer(14i64) => Ok(UpdateBlockEntityAction::SetBeehiveData {}),
            repr => Err(Error::InvalidEnumRepr(repr, "UpdateBlockEntityAction")),
        }
    }
}
pub enum BossBarData<P: Provider> {
    Add {
        title: String,
        health: f32,
        color: BossBarColor<P>,
        division: BossBarDivision<P>,
        flags: BossBarFlags<P>,
    },
    Remove {},
    UpdateHealth {
        health: f32,
    },
    UpdateTitle {
        title: String,
    },
    UpdateStyle {
        color: BossBarColor<P>,
        division: BossBarDivision<P>,
    },
    UpdateFlags {
        flags: BossBarFlags<P>,
    },
}
impl<P> BossBarData<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(0i64) => {
                let title = buf.try_get_string()?;
                let health = try_get_f32()?;
                let color = try_get_i32()?;
                let color = BossBarColor::from_repr(color as u16)?;
                let division = try_get_i32()?;
                let division = BossBarDivision::from_repr(division as u16)?;
                let flags = try_get_u8()?;
                let flags = BossBarFlags::from_repr(flags as u16)?;
                Ok(BossBarData::Add {
                    title,
                    health,
                    color,
                    division,
                    flags,
                })
            }
            EnumRepr::Integer(1i64) => Ok(BossBarData::Remove {}),
            EnumRepr::Integer(2i64) => {
                let health = try_get_f32()?;
                Ok(BossBarData::UpdateHealth { health })
            }
            EnumRepr::Integer(3i64) => {
                let title = buf.try_get_string()?;
                Ok(BossBarData::UpdateTitle { title })
            }
            EnumRepr::Integer(4i64) => {
                let color = try_get_i32()?;
                let color = BossBarColor::from_repr(color as u16)?;
                let division = try_get_i32()?;
                let division = BossBarDivision::from_repr(division as u16)?;
                Ok(BossBarData::UpdateStyle { color, division })
            }
            EnumRepr::Integer(5i64) => {
                let flags = try_get_u8()?;
                let flags = BossBarFlags::from_repr(flags as u16)?;
                Ok(BossBarData::UpdateFlags { flags })
            }
            repr => Err(Error::InvalidEnumRepr(repr, "BossBarData")),
        }
    }
}
pub enum BossBarColor<P: Provider> {
    Pink {},
    Blue {},
    Red {},
    Green {},
    Yellow {},
    Purple {},
    White {},
}
impl<P> BossBarColor<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(0i64) => Ok(BossBarColor::Pink {}),
            EnumRepr::Integer(1i64) => Ok(BossBarColor::Blue {}),
            EnumRepr::Integer(2i64) => Ok(BossBarColor::Red {}),
            EnumRepr::Integer(3i64) => Ok(BossBarColor::Green {}),
            EnumRepr::Integer(4i64) => Ok(BossBarColor::Yellow {}),
            EnumRepr::Integer(5i64) => Ok(BossBarColor::Purple {}),
            EnumRepr::Integer(6i64) => Ok(BossBarColor::White {}),
            repr => Err(Error::InvalidEnumRepr(repr, "BossBarColor")),
        }
    }
}
pub enum BossBarDivision<P: Provider> {
    None {},
    Six {},
    Ten {},
    Twelve {},
    Twenty {},
}
impl<P> BossBarDivision<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(0i64) => Ok(BossBarDivision::None {}),
            EnumRepr::Integer(1i64) => Ok(BossBarDivision::Six {}),
            EnumRepr::Integer(2i64) => Ok(BossBarDivision::Ten {}),
            EnumRepr::Integer(3i64) => Ok(BossBarDivision::Twelve {}),
            EnumRepr::Integer(4i64) => Ok(BossBarDivision::Twenty {}),
            repr => Err(Error::InvalidEnumRepr(repr, "BossBarDivision")),
        }
    }
}
pub enum ChatMessagePosition<P: Provider> {
    Chat {},
    SystemMessage {},
    GameInfo {},
}
impl<P> ChatMessagePosition<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(0i64) => Ok(ChatMessagePosition::Chat {}),
            EnumRepr::Integer(1i64) => Ok(ChatMessagePosition::SystemMessage {}),
            EnumRepr::Integer(2i64) => Ok(ChatMessagePosition::GameInfo {}),
            repr => Err(Error::InvalidEnumRepr(repr, "ChatMessagePosition")),
        }
    }
}
pub struct MultiBlockChangeRecord<P: Provider> {
    horizontal_position: u8,
    vertical_position: u8,
    block: P::Block,
}
impl<P> MultiBlockChangeRecord<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
        let version = VERSION;
        let horizontal_position = try_get_u8()?;
        let vertical_position = try_get_u8()?;
        let block = try_get_i32()?;
        let block = P::block_from_id(block as u16, version)?;
        Ok(Self {
            horizontal_position,
            vertical_position,
            block,
        })
    }
    pub fn write_to(&self, buf: &mut BytesMut) {
        let version = VERSION;
        let horizontal_position = self.horizontal_position;
        buf.put_u8(horizontal_position);
        let vertical_position = self.vertical_position;
        buf.put_u8(vertical_position);
        let block = self.block;
        let block = P::block_id(self.block, version);
        buf.put_i32(block);
    }
}
pub struct TabCompleteMatch<P: Provider> {
    name: String,
    tooltip: Option<String>,
}
impl<P> TabCompleteMatch<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
        let version = VERSION;
        let name = try_get_String()?;
        let tooltip = {
            let exists = buf.try_get_bool()?;
            if exists {
                Some(buf.try_get_string()?)
            } else {
                None
            }
        };
        Ok(Self { name, tooltip })
    }
    pub fn write_to(&self, buf: &mut BytesMut) {
        let version = VERSION;
        let name = self.name;
        buf.put_string(name);
        let tooltip = self.tooltip;
        buf.put_bool(tooltip.is_some());
        if let Some(ref tooltip) = tooltip {
            buf.put_string(tooltip);
        };
    }
}
pub struct ExplosionRecord<P: Provider> {
    x: i8,
    y: i8,
    z: i8,
}
impl<P> ExplosionRecord<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
        let version = VERSION;
        let x = try_get_i8()?;
        let y = try_get_i8()?;
        let z = try_get_i8()?;
        Ok(Self { x, y, z })
    }
    pub fn write_to(&self, buf: &mut BytesMut) {
        let version = VERSION;
        let x = self.x;
        buf.put_i8(x);
        let y = self.y;
        buf.put_i8(y);
        let z = self.z;
        buf.put_i8(z);
    }
}
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
    EnableRespawnScreen {},
}
impl<P> ChangeGameStateReason<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(0i64) => Ok(ChangeGameStateReason::InvalidBed {}),
            EnumRepr::Integer(1i64) => Ok(ChangeGameStateReason::EndRaining {}),
            EnumRepr::Integer(2i64) => Ok(ChangeGameStateReason::BeginRaining {}),
            EnumRepr::Integer(3i64) => Ok(ChangeGameStateReason::ChangeGamemode {}),
            EnumRepr::Integer(4i64) => Ok(ChangeGameStateReason::ExitEnd {}),
            EnumRepr::Integer(5i64) => Ok(ChangeGameStateReason::DemoMessage {}),
            EnumRepr::Integer(6i64) => Ok(ChangeGameStateReason::ArrowHittingPlayer {}),
            EnumRepr::Integer(7i64) => Ok(ChangeGameStateReason::FadeValue {}),
            EnumRepr::Integer(8i64) => Ok(ChangeGameStateReason::FadeTime {}),
            EnumRepr::Integer(9i64) => Ok(ChangeGameStateReason::PufferfishSting {}),
            EnumRepr::Integer(10i64) => Ok(ChangeGameStateReason::ElderGuardianAppearance {}),
            EnumRepr::Integer(11i64) => Ok(ChangeGameStateReason::EnableRespawnScreen {}),
            repr => Err(Error::InvalidEnumRepr(repr, "ChangeGameStateReason")),
        }
    }
}
pub enum JoinGameLevelType<P: Provider> {
    Default {},
    Flat {},
    LargeBiomes {},
    Amplified {},
    Customized {},
    Buffet {},
    Default11 {},
}
impl<P> JoinGameLevelType<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::String("default") => Ok(JoinGameLevelType::Default {}),
            EnumRepr::String("flat") => Ok(JoinGameLevelType::Flat {}),
            EnumRepr::String("largeBiomes") => Ok(JoinGameLevelType::LargeBiomes {}),
            EnumRepr::String("amplified") => Ok(JoinGameLevelType::Amplified {}),
            EnumRepr::String("customized") => Ok(JoinGameLevelType::Customized {}),
            EnumRepr::String("buffet") => Ok(JoinGameLevelType::Buffet {}),
            EnumRepr::String("default_1_1") => Ok(JoinGameLevelType::Default11 {}),
            repr => Err(Error::InvalidEnumRepr(repr, "JoinGameLevelType")),
        }
    }
}
pub enum CombatEventType<P: Provider> {
    EnterCombat {},
    EndCombat {
        duration: i32,
        entity: i32,
    },
    EntityDead {
        player: i32,
        entity: i32,
        message: String,
    },
}
impl<P> CombatEventType<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: EnumRepr) -> anyhow::Result<Self> {
        match repr {
            EnumRepr::Integer(0i64) => Ok(CombatEventType::EnterCombat {}),
            EnumRepr::Integer(1i64) => {
                let duration = try_get_i32()?;
                let entity = try_get_i32()?;
                Ok(CombatEventType::EndCombat { duration, entity })
            }
            EnumRepr::Integer(2i64) => {
                let player = try_get_i32()?;
                let entity = try_get_i32()?;
                let message = buf.try_get_string()?;
                Ok(CombatEventType::EntityDead {
                    player,
                    entity,
                    message,
                })
            }
            repr => Err(Error::InvalidEnumRepr(repr, "CombatEventType")),
        }
    }
}
