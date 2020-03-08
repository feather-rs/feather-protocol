// This is GENERATED CODE. Do not edit.
#![allow(warnings)]
use crate::{BytesExt, BytesMutExt, Error, ProtocolVersion, Provider};
use bytes::{Buf, BufMut, Bytes, BytesMut};
const VERSION: ProtocolVersion = ProtocolVersion::V1_15_2;
#[derive(Debug, Clone)]
pub enum SpawnGlobalEntityType<P: Provider> {
    Thunderbolt {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> SpawnGlobalEntityType<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            1i64 => Ok(SpawnGlobalEntityType::Thunderbolt {}),
            repr => Err(Error::InvalidEnumRepr(repr, "SpawnGlobalEntityType").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            SpawnGlobalEntityType::Thunderbolt {} => {}
            SpawnGlobalEntityType::_Phantom(_) => {
                panic!("phantom in {} not allowed", "SpawnGlobalEntityType")
            }
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            SpawnGlobalEntityType::Thunderbolt { .. } => 1i64,
            SpawnGlobalEntityType::_Phantom(_) => {
                panic!("phantom in {} not allowed", "SpawnGlobalEntityType")
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum SpawnPaintingDirection<P: Provider> {
    North {},
    South {},
    West {},
    East {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> SpawnPaintingDirection<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            2i64 => Ok(SpawnPaintingDirection::North {}),
            0i64 => Ok(SpawnPaintingDirection::South {}),
            1i64 => Ok(SpawnPaintingDirection::West {}),
            3i64 => Ok(SpawnPaintingDirection::East {}),
            repr => Err(Error::InvalidEnumRepr(repr, "SpawnPaintingDirection").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            SpawnPaintingDirection::North {} => {}
            SpawnPaintingDirection::South {} => {}
            SpawnPaintingDirection::West {} => {}
            SpawnPaintingDirection::East {} => {}
            SpawnPaintingDirection::_Phantom(_) => {
                panic!("phantom in {} not allowed", "SpawnPaintingDirection")
            }
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            SpawnPaintingDirection::North { .. } => 2i64,
            SpawnPaintingDirection::South { .. } => 0i64,
            SpawnPaintingDirection::West { .. } => 1i64,
            SpawnPaintingDirection::East { .. } => 3i64,
            SpawnPaintingDirection::_Phantom(_) => {
                panic!("phantom in {} not allowed", "SpawnPaintingDirection")
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum EntityAnimationType<P: Provider> {
    SwingMainArm {},
    TakeDamage {},
    LeaveBed {},
    SwingOffhand {},
    CriticalEffect {},
    MagicCriticalEffect {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> EntityAnimationType<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            0i64 => Ok(EntityAnimationType::SwingMainArm {}),
            1i64 => Ok(EntityAnimationType::TakeDamage {}),
            2i64 => Ok(EntityAnimationType::LeaveBed {}),
            3i64 => Ok(EntityAnimationType::SwingOffhand {}),
            4i64 => Ok(EntityAnimationType::CriticalEffect {}),
            5i64 => Ok(EntityAnimationType::MagicCriticalEffect {}),
            repr => Err(Error::InvalidEnumRepr(repr, "EntityAnimationType").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            EntityAnimationType::SwingMainArm {} => {}
            EntityAnimationType::TakeDamage {} => {}
            EntityAnimationType::LeaveBed {} => {}
            EntityAnimationType::SwingOffhand {} => {}
            EntityAnimationType::CriticalEffect {} => {}
            EntityAnimationType::MagicCriticalEffect {} => {}
            EntityAnimationType::_Phantom(_) => {
                panic!("phantom in {} not allowed", "EntityAnimationType")
            }
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            EntityAnimationType::SwingMainArm { .. } => 0i64,
            EntityAnimationType::TakeDamage { .. } => 1i64,
            EntityAnimationType::LeaveBed { .. } => 2i64,
            EntityAnimationType::SwingOffhand { .. } => 3i64,
            EntityAnimationType::CriticalEffect { .. } => 4i64,
            EntityAnimationType::MagicCriticalEffect { .. } => 5i64,
            EntityAnimationType::_Phantom(_) => {
                panic!("phantom in {} not allowed", "EntityAnimationType")
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct Statistic<P: Provider> {
    pub category_id: i32,
    pub statistic_id: i32,
    _phantom: std::marker::PhantomData<P>,
}
impl<P> Statistic<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
        let version = VERSION;
        let category_id = buf.try_get_i32()?;
        let statistic_id = buf.try_get_i32()?;
        Ok(Self {
            category_id,
            statistic_id,
            _phantom: Default::default(),
        })
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let category_id = self.category_id;
        buf.put_i32(category_id as i32);
        let statistic_id = self.statistic_id;
        buf.put_i32(statistic_id as i32);
    }
}
#[derive(Debug, Clone)]
pub enum AcknowledgePlayerDiggingStatus<P: Provider> {
    StartedDigging {},
    CancelledDigging {},
    FinishedDigging {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> AcknowledgePlayerDiggingStatus<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            0i64 => Ok(AcknowledgePlayerDiggingStatus::StartedDigging {}),
            1i64 => Ok(AcknowledgePlayerDiggingStatus::CancelledDigging {}),
            2i64 => Ok(AcknowledgePlayerDiggingStatus::FinishedDigging {}),
            repr => Err(Error::InvalidEnumRepr(repr, "AcknowledgePlayerDiggingStatus").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            AcknowledgePlayerDiggingStatus::StartedDigging {} => {}
            AcknowledgePlayerDiggingStatus::CancelledDigging {} => {}
            AcknowledgePlayerDiggingStatus::FinishedDigging {} => {}
            AcknowledgePlayerDiggingStatus::_Phantom(_) => panic!(
                "phantom in {} not allowed",
                "AcknowledgePlayerDiggingStatus"
            ),
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            AcknowledgePlayerDiggingStatus::StartedDigging { .. } => 0i64,
            AcknowledgePlayerDiggingStatus::CancelledDigging { .. } => 1i64,
            AcknowledgePlayerDiggingStatus::FinishedDigging { .. } => 2i64,
            AcknowledgePlayerDiggingStatus::_Phantom(_) => panic!(
                "phantom in {} not allowed",
                "AcknowledgePlayerDiggingStatus"
            ),
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
    SetBeehiveData {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> UpdateBlockEntityAction<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
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
            14i64 => Ok(UpdateBlockEntityAction::SetBeehiveData {}),
            repr => Err(Error::InvalidEnumRepr(repr, "UpdateBlockEntityAction").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
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
            UpdateBlockEntityAction::SetBeehiveData {} => {}
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
            UpdateBlockEntityAction::SetBeehiveData { .. } => 14i64,
            UpdateBlockEntityAction::_Phantom(_) => {
                panic!("phantom in {} not allowed", "UpdateBlockEntityAction")
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum BossBarData<P: Provider> {
    Add {
        title: String,
        health: f32,
        color: BossBarColor<P>,
        division: BossBarDivision<P>,
        flags: u8,
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
        flags: u8,
    },
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> BossBarData<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            0i64 => {
                let title = buf.try_get_string()?;
                let health = buf.try_get_f32()?;
                let color = buf.try_get_i32()?;
                let color = BossBarColor::read_from(buf, color as i64)?;
                let division = buf.try_get_i32()?;
                let division = BossBarDivision::read_from(buf, division as i64)?;
                let flags = buf.try_get_u8()?;
                Ok(BossBarData::Add {
                    title,
                    health,
                    color,
                    division,
                    flags,
                })
            }
            1i64 => Ok(BossBarData::Remove {}),
            2i64 => {
                let health = buf.try_get_f32()?;
                Ok(BossBarData::UpdateHealth { health })
            }
            3i64 => {
                let title = buf.try_get_string()?;
                Ok(BossBarData::UpdateTitle { title })
            }
            4i64 => {
                let color = buf.try_get_i32()?;
                let color = BossBarColor::read_from(buf, color as i64)?;
                let division = buf.try_get_i32()?;
                let division = BossBarDivision::read_from(buf, division as i64)?;
                Ok(BossBarData::UpdateStyle { color, division })
            }
            5i64 => {
                let flags = buf.try_get_u8()?;
                Ok(BossBarData::UpdateFlags { flags })
            }
            repr => Err(Error::InvalidEnumRepr(repr, "BossBarData").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            BossBarData::Add {
                title,
                health,
                color,
                division,
                flags,
            } => {
                buf.put_string(title);
                buf.put_f32(health as f32);
                let color = color.repr();
                buf.put_i32(color as i32);
                let division = division.repr();
                buf.put_i32(division as i32);
                buf.put_u8(flags as u8);
            }
            BossBarData::Remove {} => {}
            BossBarData::UpdateHealth { health } => {
                buf.put_f32(health as f32);
            }
            BossBarData::UpdateTitle { title } => {
                buf.put_string(title);
            }
            BossBarData::UpdateStyle { color, division } => {
                let color = color.repr();
                buf.put_i32(color as i32);
                let division = division.repr();
                buf.put_i32(division as i32);
            }
            BossBarData::UpdateFlags { flags } => {
                buf.put_u8(flags as u8);
            }
            BossBarData::_Phantom(_) => panic!("phantom in {} not allowed", "BossBarData"),
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            BossBarData::Add { .. } => 0i64,
            BossBarData::Remove { .. } => 1i64,
            BossBarData::UpdateHealth { .. } => 2i64,
            BossBarData::UpdateTitle { .. } => 3i64,
            BossBarData::UpdateStyle { .. } => 4i64,
            BossBarData::UpdateFlags { .. } => 5i64,
            BossBarData::_Phantom(_) => panic!("phantom in {} not allowed", "BossBarData"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BossBarColor<P: Provider> {
    Pink {},
    Blue {},
    Red {},
    Green {},
    Yellow {},
    Purple {},
    White {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> BossBarColor<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            0i64 => Ok(BossBarColor::Pink {}),
            1i64 => Ok(BossBarColor::Blue {}),
            2i64 => Ok(BossBarColor::Red {}),
            3i64 => Ok(BossBarColor::Green {}),
            4i64 => Ok(BossBarColor::Yellow {}),
            5i64 => Ok(BossBarColor::Purple {}),
            6i64 => Ok(BossBarColor::White {}),
            repr => Err(Error::InvalidEnumRepr(repr, "BossBarColor").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            BossBarColor::Pink {} => {}
            BossBarColor::Blue {} => {}
            BossBarColor::Red {} => {}
            BossBarColor::Green {} => {}
            BossBarColor::Yellow {} => {}
            BossBarColor::Purple {} => {}
            BossBarColor::White {} => {}
            BossBarColor::_Phantom(_) => panic!("phantom in {} not allowed", "BossBarColor"),
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            BossBarColor::Pink { .. } => 0i64,
            BossBarColor::Blue { .. } => 1i64,
            BossBarColor::Red { .. } => 2i64,
            BossBarColor::Green { .. } => 3i64,
            BossBarColor::Yellow { .. } => 4i64,
            BossBarColor::Purple { .. } => 5i64,
            BossBarColor::White { .. } => 6i64,
            BossBarColor::_Phantom(_) => panic!("phantom in {} not allowed", "BossBarColor"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BossBarDivision<P: Provider> {
    None {},
    Six {},
    Ten {},
    Twelve {},
    Twenty {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> BossBarDivision<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            0i64 => Ok(BossBarDivision::None {}),
            1i64 => Ok(BossBarDivision::Six {}),
            2i64 => Ok(BossBarDivision::Ten {}),
            3i64 => Ok(BossBarDivision::Twelve {}),
            4i64 => Ok(BossBarDivision::Twenty {}),
            repr => Err(Error::InvalidEnumRepr(repr, "BossBarDivision").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            BossBarDivision::None {} => {}
            BossBarDivision::Six {} => {}
            BossBarDivision::Ten {} => {}
            BossBarDivision::Twelve {} => {}
            BossBarDivision::Twenty {} => {}
            BossBarDivision::_Phantom(_) => panic!("phantom in {} not allowed", "BossBarDivision"),
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            BossBarDivision::None { .. } => 0i64,
            BossBarDivision::Six { .. } => 1i64,
            BossBarDivision::Ten { .. } => 2i64,
            BossBarDivision::Twelve { .. } => 3i64,
            BossBarDivision::Twenty { .. } => 4i64,
            BossBarDivision::_Phantom(_) => panic!("phantom in {} not allowed", "BossBarDivision"),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ChatMessagePosition<P: Provider> {
    Chat {},
    SystemMessage {},
    GameInfo {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> ChatMessagePosition<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            0i64 => Ok(ChatMessagePosition::Chat {}),
            1i64 => Ok(ChatMessagePosition::SystemMessage {}),
            2i64 => Ok(ChatMessagePosition::GameInfo {}),
            repr => Err(Error::InvalidEnumRepr(repr, "ChatMessagePosition").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            ChatMessagePosition::Chat {} => {}
            ChatMessagePosition::SystemMessage {} => {}
            ChatMessagePosition::GameInfo {} => {}
            ChatMessagePosition::_Phantom(_) => {
                panic!("phantom in {} not allowed", "ChatMessagePosition")
            }
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            ChatMessagePosition::Chat { .. } => 0i64,
            ChatMessagePosition::SystemMessage { .. } => 1i64,
            ChatMessagePosition::GameInfo { .. } => 2i64,
            ChatMessagePosition::_Phantom(_) => {
                panic!("phantom in {} not allowed", "ChatMessagePosition")
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct MultiBlockChangeRecord<P: Provider> {
    pub horizontal_position: u8,
    pub vertical_position: u8,
    pub block: P::Block,
    _phantom: std::marker::PhantomData<P>,
}
impl<P> MultiBlockChangeRecord<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
        let version = VERSION;
        let horizontal_position = buf.try_get_u8()?;
        let vertical_position = buf.try_get_u8()?;
        let block = buf.try_get_i32()?;
        let block = P::block_from_id(block as u16, version)?;
        Ok(Self {
            horizontal_position,
            vertical_position,
            block,
            _phantom: Default::default(),
        })
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let horizontal_position = self.horizontal_position;
        buf.put_u8(horizontal_position as u8);
        let vertical_position = self.vertical_position;
        buf.put_u8(vertical_position as u8);
        let block = self.block;
        let block = P::block_id(block, version);
        buf.put_i32(block as i32);
    }
}
#[derive(Debug, Clone)]
pub struct TabCompleteMatch<P: Provider> {
    pub name: String,
    pub tooltip: Option<String>,
    _phantom: std::marker::PhantomData<P>,
}
impl<P> TabCompleteMatch<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
        let version = VERSION;
        let name = buf.try_get_string()?;
        let tooltip = {
            let exists = buf.try_get_bool()?;
            if exists {
                Some(buf.try_get_string()?)
            } else {
                None
            }
        };
        Ok(Self {
            name,
            tooltip,
            _phantom: Default::default(),
        })
    }
    pub fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct ExplosionRecord<P: Provider> {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    _phantom: std::marker::PhantomData<P>,
}
impl<P> ExplosionRecord<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes) -> anyhow::Result<Self> {
        let version = VERSION;
        let x = buf.try_get_i8()?;
        let y = buf.try_get_i8()?;
        let z = buf.try_get_i8()?;
        Ok(Self {
            x,
            y,
            z,
            _phantom: Default::default(),
        })
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let x = self.x;
        buf.put_i8(x as i8);
        let y = self.y;
        buf.put_i8(y as i8);
        let z = self.z;
        buf.put_i8(z as i8);
    }
}
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
    EnableRespawnScreen {},
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> ChangeGameStateReason<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
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
            11i64 => Ok(ChangeGameStateReason::EnableRespawnScreen {}),
            repr => Err(Error::InvalidEnumRepr(repr, "ChangeGameStateReason").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
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
            ChangeGameStateReason::EnableRespawnScreen {} => {}
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
            ChangeGameStateReason::EnableRespawnScreen { .. } => 11i64,
            ChangeGameStateReason::_Phantom(_) => {
                panic!("phantom in {} not allowed", "ChangeGameStateReason")
            }
        }
    }
}
#[derive(Debug, Clone)]
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
    _Phantom(std::marker::PhantomData<P>),
}
impl<P> CombatEventType<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut Bytes, repr: i64) -> anyhow::Result<Self> {
        match repr {
            0i64 => Ok(CombatEventType::EnterCombat {}),
            1i64 => {
                let duration = buf.try_get_i32()?;
                let entity = buf.try_get_i32()?;
                Ok(CombatEventType::EndCombat { duration, entity })
            }
            2i64 => {
                let player = buf.try_get_i32()?;
                let entity = buf.try_get_i32()?;
                let message = buf.try_get_string()?;
                Ok(CombatEventType::EntityDead {
                    player,
                    entity,
                    message,
                })
            }
            repr => Err(Error::InvalidEnumRepr(repr, "CombatEventType").into()),
        }
    }
    pub fn write_to(self, buf: &mut BytesMut) {
        match self {
            CombatEventType::EnterCombat {} => {}
            CombatEventType::EndCombat { duration, entity } => {
                buf.put_i32(duration as i32);
                buf.put_i32(entity as i32);
            }
            CombatEventType::EntityDead {
                player,
                entity,
                message,
            } => {
                buf.put_i32(player as i32);
                buf.put_i32(entity as i32);
                buf.put_string(message);
            }
            CombatEventType::_Phantom(_) => panic!("phantom in {} not allowed", "CombatEventType"),
        }
    }
    pub fn repr(&self) -> i64 {
        match self {
            CombatEventType::EnterCombat { .. } => 0i64,
            CombatEventType::EndCombat { .. } => 1i64,
            CombatEventType::EntityDead { .. } => 2i64,
            CombatEventType::_Phantom(_) => panic!("phantom in {} not allowed", "CombatEventType"),
        }
    }
}
use crate::{BlockPosition, DynPacket, Node, Packet, PacketReader, Slot};
#[derive(Debug, Clone)]
pub struct SpawnObject {
    pub entity: i32,
    pub uuid: uuid::Uuid,
    pub ty: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: u8,
    pub yaw: u8,
    pub data: i32,
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
}
impl Packet for SpawnObject {
    fn id(&self) -> u32 {
        0u32
    }
    fn name(&self) -> &'static str {
        "SpawnObject"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let uuid = self.uuid;
        buf.put_uuid(uuid);
        let ty = self.ty;
        buf.put_i32(ty as i32);
        let x = self.x;
        buf.put_f64(x as f64);
        let y = self.y;
        buf.put_f64(y as f64);
        let z = self.z;
        buf.put_f64(z as f64);
        let pitch = self.pitch;
        buf.put_u8(pitch as u8);
        let yaw = self.yaw;
        buf.put_u8(yaw as u8);
        let data = self.data;
        buf.put_i32(data as i32);
        let dx = self.dx;
        buf.put_i16(dx as i16);
        let dy = self.dy;
        buf.put_i16(dy as i16);
        let dz = self.dz;
        buf.put_i16(dz as i16);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SpawnObjectReader;
impl Default for SpawnObjectReader {
    fn default() -> Self {
        SpawnObjectReader
    }
}
impl PacketReader for SpawnObjectReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let uuid = buf.try_get_uuid()?;
        let ty = buf.try_get_i32()?;
        let x = buf.try_get_f64()?;
        let y = buf.try_get_f64()?;
        let z = buf.try_get_f64()?;
        let pitch = buf.try_get_u8()?;
        let yaw = buf.try_get_u8()?;
        let data = buf.try_get_i32()?;
        let dx = buf.try_get_i16()?;
        let dy = buf.try_get_i16()?;
        let dz = buf.try_get_i16()?;
        let packet = SpawnObject {
            entity,
            uuid,
            ty,
            x,
            y,
            z,
            pitch,
            yaw,
            data,
            dx,
            dy,
            dz,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct SpawnExperienceOrb {
    pub entity: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}
impl Packet for SpawnExperienceOrb {
    fn id(&self) -> u32 {
        1u32
    }
    fn name(&self) -> &'static str {
        "SpawnExperienceOrb"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let x = self.x;
        buf.put_f64(x as f64);
        let y = self.y;
        buf.put_f64(y as f64);
        let z = self.z;
        buf.put_f64(z as f64);
        let count = self.count;
        buf.put_i16(count as i16);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SpawnExperienceOrbReader;
impl Default for SpawnExperienceOrbReader {
    fn default() -> Self {
        SpawnExperienceOrbReader
    }
}
impl PacketReader for SpawnExperienceOrbReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let x = buf.try_get_f64()?;
        let y = buf.try_get_f64()?;
        let z = buf.try_get_f64()?;
        let count = buf.try_get_i16()?;
        let packet = SpawnExperienceOrb {
            entity,
            x,
            y,
            z,
            count,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct SpawnGlobalEntity<P: Provider> {
    pub entity: i32,
    pub ty: SpawnGlobalEntityType<P>,
}
impl<P> Packet for SpawnGlobalEntity<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        2u32
    }
    fn name(&self) -> &'static str {
        "SpawnGlobalEntity"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let ty = self.ty;
        let ty = ty.repr();
        buf.put_u8(ty as u8);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SpawnGlobalEntityReader<P>(std::marker::PhantomData<P>);
impl<P> Default for SpawnGlobalEntityReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for SpawnGlobalEntityReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let ty = buf.try_get_u8()?;
        let ty = SpawnGlobalEntityType::read_from(buf, ty as i64)?;
        let packet = SpawnGlobalEntity::<P> { entity, ty };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct SpawnMob {
    pub entity: i32,
    pub uuid: uuid::Uuid,
    pub ty: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: u8,
    pub pitch: u8,
    pub head_pitch: u8,
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
}
impl Packet for SpawnMob {
    fn id(&self) -> u32 {
        3u32
    }
    fn name(&self) -> &'static str {
        "SpawnMob"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let uuid = self.uuid;
        buf.put_uuid(uuid);
        let ty = self.ty;
        buf.put_i32(ty as i32);
        let x = self.x;
        buf.put_f64(x as f64);
        let y = self.y;
        buf.put_f64(y as f64);
        let z = self.z;
        buf.put_f64(z as f64);
        let yaw = self.yaw;
        buf.put_u8(yaw as u8);
        let pitch = self.pitch;
        buf.put_u8(pitch as u8);
        let head_pitch = self.head_pitch;
        buf.put_u8(head_pitch as u8);
        let dx = self.dx;
        buf.put_i16(dx as i16);
        let dy = self.dy;
        buf.put_i16(dy as i16);
        let dz = self.dz;
        buf.put_i16(dz as i16);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SpawnMobReader;
impl Default for SpawnMobReader {
    fn default() -> Self {
        SpawnMobReader
    }
}
impl PacketReader for SpawnMobReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let uuid = buf.try_get_uuid()?;
        let ty = buf.try_get_i32()?;
        let x = buf.try_get_f64()?;
        let y = buf.try_get_f64()?;
        let z = buf.try_get_f64()?;
        let yaw = buf.try_get_u8()?;
        let pitch = buf.try_get_u8()?;
        let head_pitch = buf.try_get_u8()?;
        let dx = buf.try_get_i16()?;
        let dy = buf.try_get_i16()?;
        let dz = buf.try_get_i16()?;
        let packet = SpawnMob {
            entity,
            uuid,
            ty,
            x,
            y,
            z,
            yaw,
            pitch,
            head_pitch,
            dx,
            dy,
            dz,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct SpawnPainting<P: Provider> {
    pub entity: i32,
    pub uuid: uuid::Uuid,
    pub motive: i32,
    pub position: BlockPosition,
    pub direction: SpawnPaintingDirection<P>,
}
impl<P> Packet for SpawnPainting<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        4u32
    }
    fn name(&self) -> &'static str {
        "SpawnPainting"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let uuid = self.uuid;
        buf.put_uuid(uuid);
        let motive = self.motive;
        buf.put_i32(motive as i32);
        let position = self.position;
        buf.put_position(position);
        let direction = self.direction;
        let direction = direction.repr();
        buf.put_u8(direction as u8);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SpawnPaintingReader<P>(std::marker::PhantomData<P>);
impl<P> Default for SpawnPaintingReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for SpawnPaintingReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let uuid = buf.try_get_uuid()?;
        let motive = buf.try_get_i32()?;
        let position = buf.try_get_position()?;
        let direction = buf.try_get_u8()?;
        let direction = SpawnPaintingDirection::read_from(buf, direction as i64)?;
        let packet = SpawnPainting::<P> {
            entity,
            uuid,
            motive,
            position,
            direction,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct SpawnPlayer {
    pub entity: i32,
    pub uuid: uuid::Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: u8,
    pub pitch: u8,
}
impl Packet for SpawnPlayer {
    fn id(&self) -> u32 {
        5u32
    }
    fn name(&self) -> &'static str {
        "SpawnPlayer"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let uuid = self.uuid;
        buf.put_uuid(uuid);
        let x = self.x;
        buf.put_f64(x as f64);
        let y = self.y;
        buf.put_f64(y as f64);
        let z = self.z;
        buf.put_f64(z as f64);
        let yaw = self.yaw;
        buf.put_u8(yaw as u8);
        let pitch = self.pitch;
        buf.put_u8(pitch as u8);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SpawnPlayerReader;
impl Default for SpawnPlayerReader {
    fn default() -> Self {
        SpawnPlayerReader
    }
}
impl PacketReader for SpawnPlayerReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let uuid = buf.try_get_uuid()?;
        let x = buf.try_get_f64()?;
        let y = buf.try_get_f64()?;
        let z = buf.try_get_f64()?;
        let yaw = buf.try_get_u8()?;
        let pitch = buf.try_get_u8()?;
        let packet = SpawnPlayer {
            entity,
            uuid,
            x,
            y,
            z,
            yaw,
            pitch,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct EntityAnimation<P: Provider> {
    pub entity: i32,
    pub animation: EntityAnimationType<P>,
}
impl<P> Packet for EntityAnimation<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        6u32
    }
    fn name(&self) -> &'static str {
        "EntityAnimation"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let animation = self.animation;
        let animation = animation.repr();
        buf.put_u8(animation as u8);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EntityAnimationReader<P>(std::marker::PhantomData<P>);
impl<P> Default for EntityAnimationReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for EntityAnimationReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let animation = buf.try_get_u8()?;
        let animation = EntityAnimationType::read_from(buf, animation as i64)?;
        let packet = EntityAnimation::<P> { entity, animation };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct Statistics<P: Provider> {
    pub statistics: Vec<Statistic<P>>,
    pub value: i32,
}
impl<P> Packet for Statistics<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        7u32
    }
    fn name(&self) -> &'static str {
        "Statistics"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let count = self.statistics.len() as i32;
        buf.put_i32(count as i32);
        let statistics = self.statistics;
        statistics.into_iter().for_each(|x| x.write_to(buf));
        let value = self.value;
        buf.put_i32(value as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct StatisticsReader<P>(std::marker::PhantomData<P>);
impl<P> Default for StatisticsReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for StatisticsReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let statistics_len = buf.try_get_i32()?;
        let mut statistics = vec![];
        for _ in 0..statistics_len {
            let elem = Statistic::<P>::read_from(buf)?;
            statistics.push(elem);
        }
        let value = buf.try_get_i32()?;
        let packet = Statistics::<P> { statistics, value };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct AcknowledgePlayerDigging<P: Provider> {
    pub position: BlockPosition,
    pub block: P::Block,
    pub status: AcknowledgePlayerDiggingStatus<P>,
    pub successful: bool,
}
impl<P> Packet for AcknowledgePlayerDigging<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        8u32
    }
    fn name(&self) -> &'static str {
        "AcknowledgePlayerDigging"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let position = self.position;
        buf.put_position(position);
        let block = self.block;
        let block = P::block_id(block, version);
        buf.put_i32(block as i32);
        let status = self.status;
        let status = status.repr();
        buf.put_i32(status as i32);
        let successful = self.successful;
        buf.put_bool(successful as bool);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct AcknowledgePlayerDiggingReader<P>(std::marker::PhantomData<P>);
impl<P> Default for AcknowledgePlayerDiggingReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for AcknowledgePlayerDiggingReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let position = buf.try_get_position()?;
        let block = buf.try_get_i32()?;
        let block = P::block_from_id(block as u16, version)?;
        let status = buf.try_get_i32()?;
        let status = AcknowledgePlayerDiggingStatus::read_from(buf, status as i64)?;
        let successful = buf.try_get_bool()?;
        let packet = AcknowledgePlayerDigging::<P> {
            position,
            block,
            status,
            successful,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct BlockBreakAnimation {
    pub breaker: i32,
    pub position: BlockPosition,
    pub destroy_stage: u8,
}
impl Packet for BlockBreakAnimation {
    fn id(&self) -> u32 {
        9u32
    }
    fn name(&self) -> &'static str {
        "BlockBreakAnimation"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let breaker = self.breaker;
        buf.put_i32(breaker as i32);
        let position = self.position;
        buf.put_position(position);
        let destroy_stage = self.destroy_stage;
        buf.put_u8(destroy_stage as u8);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BlockBreakAnimationReader;
impl Default for BlockBreakAnimationReader {
    fn default() -> Self {
        BlockBreakAnimationReader
    }
}
impl PacketReader for BlockBreakAnimationReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let breaker = buf.try_get_i32()?;
        let position = buf.try_get_position()?;
        let destroy_stage = buf.try_get_u8()?;
        let packet = BlockBreakAnimation {
            breaker,
            position,
            destroy_stage,
        };
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
    fn id(&self) -> u32 {
        10u32
    }
    fn name(&self) -> &'static str {
        "UpdateBlockEntity"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
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
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let os = buf.try_get_position()?;
        let action = buf.try_get_u8()?;
        let action = UpdateBlockEntityAction::read_from(buf, action as i64)?;
        let data = buf.try_get_nbt()?;
        let packet = UpdateBlockEntity::<P> { os, action, data };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct BlockAction<P: Provider> {
    pub position: BlockPosition,
    pub action_id: u8,
    pub action_param: u8,
    pub block_type: P::Block,
}
impl<P> Packet for BlockAction<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        11u32
    }
    fn name(&self) -> &'static str {
        "BlockAction"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let position = self.position;
        buf.put_position(position);
        let action_id = self.action_id;
        buf.put_u8(action_id as u8);
        let action_param = self.action_param;
        buf.put_u8(action_param as u8);
        let block_type = self.block_type;
        let block_type = P::block_ty(block_type, version);
        buf.put_i32(block_type as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BlockActionReader<P>(std::marker::PhantomData<P>);
impl<P> Default for BlockActionReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for BlockActionReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let position = buf.try_get_position()?;
        let action_id = buf.try_get_u8()?;
        let action_param = buf.try_get_u8()?;
        let block_type = buf.try_get_i32()?;
        let block_type = P::block_from_ty(block_type as u16, version)?;
        let packet = BlockAction::<P> {
            position,
            action_id,
            action_param,
            block_type,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct BlockChange<P: Provider> {
    pub position: BlockPosition,
    pub block: P::Block,
}
impl<P> Packet for BlockChange<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        12u32
    }
    fn name(&self) -> &'static str {
        "BlockChange"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let position = self.position;
        buf.put_position(position);
        let block = self.block;
        let block = P::block_id(block, version);
        buf.put_i32(block as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BlockChangeReader<P>(std::marker::PhantomData<P>);
impl<P> Default for BlockChangeReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for BlockChangeReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let position = buf.try_get_position()?;
        let block = buf.try_get_i32()?;
        let block = P::block_from_id(block as u16, version)?;
        let packet = BlockChange::<P> { position, block };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct BossBar<P: Provider> {
    pub uuid: uuid::Uuid,
    pub data: BossBarData<P>,
}
impl<P> Packet for BossBar<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        13u32
    }
    fn name(&self) -> &'static str {
        "BossBar"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let uuid = self.uuid;
        buf.put_uuid(uuid);
        let action = self.data.repr() as i32;
        buf.put_i32(action as i32);
        let data = self.data;
        data.write_to(buf);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct BossBarReader<P>(std::marker::PhantomData<P>);
impl<P> Default for BossBarReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for BossBarReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let uuid = buf.try_get_uuid()?;
        let data_repr = buf.try_get_i32()?;
        let data = BossBarData::<P>::read_from(buf, data_repr as i64)?;
        let packet = BossBar::<P> { uuid, data };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct ServerDifficulty {
    pub difficulty: u8,
    pub difficulty_locked: bool,
}
impl Packet for ServerDifficulty {
    fn id(&self) -> u32 {
        14u32
    }
    fn name(&self) -> &'static str {
        "ServerDifficulty"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let difficulty = self.difficulty;
        buf.put_u8(difficulty as u8);
        let difficulty_locked = self.difficulty_locked;
        buf.put_bool(difficulty_locked as bool);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ServerDifficultyReader;
impl Default for ServerDifficultyReader {
    fn default() -> Self {
        ServerDifficultyReader
    }
}
impl PacketReader for ServerDifficultyReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let difficulty = buf.try_get_u8()?;
        let difficulty_locked = buf.try_get_bool()?;
        let packet = ServerDifficulty {
            difficulty,
            difficulty_locked,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct ChatMessage<P: Provider> {
    pub data: String,
    pub position: ChatMessagePosition<P>,
}
impl<P> Packet for ChatMessage<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        15u32
    }
    fn name(&self) -> &'static str {
        "ChatMessage"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let data = self.data;
        buf.put_string(data);
        let position = self.position;
        let position = position.repr();
        buf.put_u8(position as u8);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ChatMessageReader<P>(std::marker::PhantomData<P>);
impl<P> Default for ChatMessageReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for ChatMessageReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let data = buf.try_get_string()?;
        let position = buf.try_get_u8()?;
        let position = ChatMessagePosition::read_from(buf, position as i64)?;
        let packet = ChatMessage::<P> { data, position };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct MultiBlockChange<P: Provider> {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub records: Vec<MultiBlockChangeRecord<P>>,
}
impl<P> Packet for MultiBlockChange<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        16u32
    }
    fn name(&self) -> &'static str {
        "MultiBlockChange"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let chunk_x = self.chunk_x;
        buf.put_i32(chunk_x as i32);
        let chunk_z = self.chunk_z;
        buf.put_i32(chunk_z as i32);
        let count = self.records.len() as i32;
        buf.put_i32(count as i32);
        let records = self.records;
        records.into_iter().for_each(|x| x.write_to(buf));
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MultiBlockChangeReader<P>(std::marker::PhantomData<P>);
impl<P> Default for MultiBlockChangeReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for MultiBlockChangeReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let chunk_x = buf.try_get_i32()?;
        let chunk_z = buf.try_get_i32()?;
        let records_len = buf.try_get_i32()?;
        let mut records = vec![];
        for _ in 0..records_len {
            let elem = MultiBlockChangeRecord::<P>::read_from(buf)?;
            records.push(elem);
        }
        let packet = MultiBlockChange::<P> {
            chunk_x,
            chunk_z,
            records,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct TabComplete<P: Provider> {
    pub transaction_id: i32,
    pub start: i32,
    pub length: i32,
    pub matches: Vec<TabCompleteMatch<P>>,
}
impl<P> Packet for TabComplete<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        17u32
    }
    fn name(&self) -> &'static str {
        "TabComplete"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let transaction_id = self.transaction_id;
        buf.put_i32(transaction_id as i32);
        let start = self.start;
        buf.put_i32(start as i32);
        let length = self.length;
        buf.put_i32(length as i32);
        let count = self.matches.len() as i32;
        buf.put_i32(count as i32);
        let matches = self.matches;
        matches.into_iter().for_each(|x| x.write_to(buf));
    }
}
#[derive(Debug, Clone, Copy)]
pub struct TabCompleteReader<P>(std::marker::PhantomData<P>);
impl<P> Default for TabCompleteReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for TabCompleteReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let transaction_id = buf.try_get_i32()?;
        let start = buf.try_get_i32()?;
        let length = buf.try_get_i32()?;
        let matches_len = buf.try_get_i32()?;
        let mut matches = vec![];
        for _ in 0..matches_len {
            let elem = TabCompleteMatch::<P>::read_from(buf)?;
            matches.push(elem);
        }
        let packet = TabComplete::<P> {
            transaction_id,
            start,
            length,
            matches,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct DeclareCommands {
    pub nodes: Vec<Node>,
    pub root_index: i32,
}
impl Packet for DeclareCommands {
    fn id(&self) -> u32 {
        18u32
    }
    fn name(&self) -> &'static str {
        "DeclareCommands"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let count = self.nodes.len() as i32;
        buf.put_i32(count as i32);
        let nodes = self.nodes;
        nodes.into_iter().for_each(|x| buf.put_node(x));
        let root_index = self.root_index;
        buf.put_i32(root_index as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DeclareCommandsReader;
impl Default for DeclareCommandsReader {
    fn default() -> Self {
        DeclareCommandsReader
    }
}
impl PacketReader for DeclareCommandsReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let nodes_len = buf.try_get_i32()?;
        let mut nodes = vec![];
        for _ in 0..nodes_len {
            let elem = buf.try_get_node()?;
            nodes.push(elem);
        }
        let root_index = buf.try_get_i32()?;
        let packet = DeclareCommands { nodes, root_index };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct WindowConfirmation {
    pub window_id: u8,
    pub action_number: i16,
    pub accepted: bool,
}
impl Packet for WindowConfirmation {
    fn id(&self) -> u32 {
        19u32
    }
    fn name(&self) -> &'static str {
        "WindowConfirmation"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let action_number = self.action_number;
        buf.put_i16(action_number as i16);
        let accepted = self.accepted;
        buf.put_bool(accepted as bool);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WindowConfirmationReader;
impl Default for WindowConfirmationReader {
    fn default() -> Self {
        WindowConfirmationReader
    }
}
impl PacketReader for WindowConfirmationReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let window_id = buf.try_get_u8()?;
        let action_number = buf.try_get_i16()?;
        let accepted = buf.try_get_bool()?;
        let packet = WindowConfirmation {
            window_id,
            action_number,
            accepted,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct CloseWindow {
    pub window_id: u8,
}
impl Packet for CloseWindow {
    fn id(&self) -> u32 {
        20u32
    }
    fn name(&self) -> &'static str {
        "CloseWindow"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CloseWindowReader;
impl Default for CloseWindowReader {
    fn default() -> Self {
        CloseWindowReader
    }
}
impl PacketReader for CloseWindowReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let window_id = buf.try_get_u8()?;
        let packet = CloseWindow { window_id };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct WindowItems {
    pub window_id: u8,
    pub slots: Vec<Slot>,
}
impl Packet for WindowItems {
    fn id(&self) -> u32 {
        21u32
    }
    fn name(&self) -> &'static str {
        "WindowItems"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let count = self.slots.len() as i16;
        buf.put_i16(count as i16);
        let slots = self.slots;
        slots.into_iter().for_each(|x| buf.put_slot(x));
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WindowItemsReader;
impl Default for WindowItemsReader {
    fn default() -> Self {
        WindowItemsReader
    }
}
impl PacketReader for WindowItemsReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let window_id = buf.try_get_u8()?;
        let slots_len = buf.try_get_i16()?;
        let mut slots = vec![];
        for _ in 0..slots_len {
            let elem = buf.try_get_slot()?;
            slots.push(elem);
        }
        let packet = WindowItems { window_id, slots };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct WindowProperty {
    pub window_id: u8,
    pub property: i16,
    pub value: i16,
}
impl Packet for WindowProperty {
    fn id(&self) -> u32 {
        22u32
    }
    fn name(&self) -> &'static str {
        "WindowProperty"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let property = self.property;
        buf.put_i16(property as i16);
        let value = self.value;
        buf.put_i16(value as i16);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct WindowPropertyReader;
impl Default for WindowPropertyReader {
    fn default() -> Self {
        WindowPropertyReader
    }
}
impl PacketReader for WindowPropertyReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let window_id = buf.try_get_u8()?;
        let property = buf.try_get_i16()?;
        let value = buf.try_get_i16()?;
        let packet = WindowProperty {
            window_id,
            property,
            value,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct SetSlot {
    pub window_id: u8,
    pub slot_index: i16,
    pub slot: Slot,
}
impl Packet for SetSlot {
    fn id(&self) -> u32 {
        23u32
    }
    fn name(&self) -> &'static str {
        "SetSlot"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let slot_index = self.slot_index;
        buf.put_i16(slot_index as i16);
        let slot = self.slot;
        buf.put_slot(slot);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SetSlotReader;
impl Default for SetSlotReader {
    fn default() -> Self {
        SetSlotReader
    }
}
impl PacketReader for SetSlotReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let window_id = buf.try_get_u8()?;
        let slot_index = buf.try_get_i16()?;
        let slot = buf.try_get_slot()?;
        let packet = SetSlot {
            window_id,
            slot_index,
            slot,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct SetCooldown<P: Provider> {
    pub item: P::Item,
    pub cooldown_ticks: i32,
}
impl<P> Packet for SetCooldown<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        24u32
    }
    fn name(&self) -> &'static str {
        "SetCooldown"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let item = self.item;
        let item = P::item_id(item, version);
        buf.put_i32(item as i32);
        let cooldown_ticks = self.cooldown_ticks;
        buf.put_i32(cooldown_ticks as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SetCooldownReader<P>(std::marker::PhantomData<P>);
impl<P> Default for SetCooldownReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for SetCooldownReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let item = buf.try_get_i32()?;
        let item = P::item_from_id(item as u16, version)?;
        let cooldown_ticks = buf.try_get_i32()?;
        let packet = SetCooldown::<P> {
            item,
            cooldown_ticks,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct PluginMessage {
    pub channel: String,
    pub data: Vec<i8>,
}
impl Packet for PluginMessage {
    fn id(&self) -> u32 {
        25u32
    }
    fn name(&self) -> &'static str {
        "PluginMessage"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let channel = self.channel;
        buf.put_string(channel);
        let data = self.data;
        data.into_iter().for_each(|x| buf.put_i8(x as i8));
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PluginMessageReader;
impl Default for PluginMessageReader {
    fn default() -> Self {
        PluginMessageReader
    }
}
impl PacketReader for PluginMessageReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let channel = buf.try_get_string()?;
        let mut data = vec![];
        while buf.has_remaining() {
            let elem = buf.try_get_i8()?;
            data.push(elem);
        }
        let packet = PluginMessage { channel, data };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct NamedSoundEffect {
    pub name: String,
    pub category: i32,
    pub posx: i32,
    pub posy: i32,
    pub posz: i32,
    pub volume: f32,
    pub pitch: f32,
}
impl Packet for NamedSoundEffect {
    fn id(&self) -> u32 {
        26u32
    }
    fn name(&self) -> &'static str {
        "NamedSoundEffect"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let name = self.name;
        buf.put_string(name);
        let category = self.category;
        buf.put_i32(category as i32);
        let posx = self.posx;
        buf.put_i32(posx as i32);
        let posy = self.posy;
        buf.put_i32(posy as i32);
        let posz = self.posz;
        buf.put_i32(posz as i32);
        let volume = self.volume;
        buf.put_f32(volume as f32);
        let pitch = self.pitch;
        buf.put_f32(pitch as f32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct NamedSoundEffectReader;
impl Default for NamedSoundEffectReader {
    fn default() -> Self {
        NamedSoundEffectReader
    }
}
impl PacketReader for NamedSoundEffectReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let name = buf.try_get_string()?;
        let category = buf.try_get_i32()?;
        let posx = buf.try_get_i32()?;
        let posy = buf.try_get_i32()?;
        let posz = buf.try_get_i32()?;
        let volume = buf.try_get_f32()?;
        let pitch = buf.try_get_f32()?;
        let packet = NamedSoundEffect {
            name,
            category,
            posx,
            posy,
            posz,
            volume,
            pitch,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct Disconnect {
    pub reason: String,
}
impl Packet for Disconnect {
    fn id(&self) -> u32 {
        27u32
    }
    fn name(&self) -> &'static str {
        "Disconnect"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let reason = self.reason;
        buf.put_string(reason);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct DisconnectReader;
impl Default for DisconnectReader {
    fn default() -> Self {
        DisconnectReader
    }
}
impl PacketReader for DisconnectReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let reason = buf.try_get_string()?;
        let packet = Disconnect { reason };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct EntityStatus {
    pub entity: i32,
    pub status: i8,
}
impl Packet for EntityStatus {
    fn id(&self) -> u32 {
        28u32
    }
    fn name(&self) -> &'static str {
        "EntityStatus"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let status = self.status;
        buf.put_i8(status as i8);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EntityStatusReader;
impl Default for EntityStatusReader {
    fn default() -> Self {
        EntityStatusReader
    }
}
impl PacketReader for EntityStatusReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let status = buf.try_get_i8()?;
        let packet = EntityStatus { entity, status };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct Explosion<P: Provider> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub strength: f32,
    pub records: Vec<ExplosionRecord<P>>,
    pub player_dx: f32,
    pub player_dy: f32,
    pub player_dz: f32,
}
impl<P> Packet for Explosion<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        29u32
    }
    fn name(&self) -> &'static str {
        "Explosion"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let x = self.x;
        buf.put_f32(x as f32);
        let y = self.y;
        buf.put_f32(y as f32);
        let z = self.z;
        buf.put_f32(z as f32);
        let strength = self.strength;
        buf.put_f32(strength as f32);
        let record_count = self.records.len() as i32;
        buf.put_i32(record_count as i32);
        let records = self.records;
        records.into_iter().for_each(|x| x.write_to(buf));
        let player_dx = self.player_dx;
        buf.put_f32(player_dx as f32);
        let player_dy = self.player_dy;
        buf.put_f32(player_dy as f32);
        let player_dz = self.player_dz;
        buf.put_f32(player_dz as f32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct ExplosionReader<P>(std::marker::PhantomData<P>);
impl<P> Default for ExplosionReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for ExplosionReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let x = buf.try_get_f32()?;
        let y = buf.try_get_f32()?;
        let z = buf.try_get_f32()?;
        let strength = buf.try_get_f32()?;
        let records_len = buf.try_get_i32()?;
        let mut records = vec![];
        for _ in 0..records_len {
            let elem = ExplosionRecord::<P>::read_from(buf)?;
            records.push(elem);
        }
        let player_dx = buf.try_get_f32()?;
        let player_dy = buf.try_get_f32()?;
        let player_dz = buf.try_get_f32()?;
        let packet = Explosion::<P> {
            x,
            y,
            z,
            strength,
            records,
            player_dx,
            player_dy,
            player_dz,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct UnloadChunk {
    pub x: i32,
    pub z: i32,
}
impl Packet for UnloadChunk {
    fn id(&self) -> u32 {
        30u32
    }
    fn name(&self) -> &'static str {
        "UnloadChunk"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let x = self.x;
        buf.put_i32(x as i32);
        let z = self.z;
        buf.put_i32(z as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct UnloadChunkReader;
impl Default for UnloadChunkReader {
    fn default() -> Self {
        UnloadChunkReader
    }
}
impl PacketReader for UnloadChunkReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let x = buf.try_get_i32()?;
        let z = buf.try_get_i32()?;
        let packet = UnloadChunk { x, z };
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
    fn id(&self) -> u32 {
        31u32
    }
    fn name(&self) -> &'static str {
        "ChangeGameState"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
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
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let reason = buf.try_get_u8()?;
        let reason = ChangeGameStateReason::read_from(buf, reason as i64)?;
        let value = buf.try_get_f32()?;
        let packet = ChangeGameState::<P> { reason, value };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct OpenHorseWindow {
    pub window_id: u8,
    pub num_slots: i32,
    pub entity: i32,
}
impl Packet for OpenHorseWindow {
    fn id(&self) -> u32 {
        32u32
    }
    fn name(&self) -> &'static str {
        "OpenHorseWindow"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let num_slots = self.num_slots;
        buf.put_i32(num_slots as i32);
        let entity = self.entity;
        buf.put_i32(entity as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OpenHorseWindowReader;
impl Default for OpenHorseWindowReader {
    fn default() -> Self {
        OpenHorseWindowReader
    }
}
impl PacketReader for OpenHorseWindowReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let window_id = buf.try_get_u8()?;
        let num_slots = buf.try_get_i32()?;
        let entity = buf.try_get_i32()?;
        let packet = OpenHorseWindow {
            window_id,
            num_slots,
            entity,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct KeepAlive {
    pub id: i64,
}
impl Packet for KeepAlive {
    fn id(&self) -> u32 {
        33u32
    }
    fn name(&self) -> &'static str {
        "KeepAlive"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let id = self.id;
        buf.put_i64(id as i64);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct KeepAliveReader;
impl Default for KeepAliveReader {
    fn default() -> Self {
        KeepAliveReader
    }
}
impl PacketReader for KeepAliveReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let id = buf.try_get_i64()?;
        let packet = KeepAlive { id };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct Effect {
    pub id: i32,
    pub position: BlockPosition,
    pub data: i32,
    pub disable_relative_volume: bool,
}
impl Packet for Effect {
    fn id(&self) -> u32 {
        35u32
    }
    fn name(&self) -> &'static str {
        "Effect"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let id = self.id;
        buf.put_i32(id as i32);
        let position = self.position;
        buf.put_position(position);
        let data = self.data;
        buf.put_i32(data as i32);
        let disable_relative_volume = self.disable_relative_volume;
        buf.put_bool(disable_relative_volume as bool);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EffectReader;
impl Default for EffectReader {
    fn default() -> Self {
        EffectReader
    }
}
impl PacketReader for EffectReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let id = buf.try_get_i32()?;
        let position = buf.try_get_position()?;
        let data = buf.try_get_i32()?;
        let disable_relative_volume = buf.try_get_bool()?;
        let packet = Effect {
            id,
            position,
            data,
            disable_relative_volume,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct JoinGame {
    pub player_eid: i32,
    pub gamemode: u8,
    pub dimension: i32,
    pub hashed_seed: i64,
    pub max_players: u8,
    pub level_type: String,
    pub view_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
}
impl Packet for JoinGame {
    fn id(&self) -> u32 {
        38u32
    }
    fn name(&self) -> &'static str {
        "JoinGame"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let player_eid = self.player_eid;
        buf.put_i32(player_eid as i32);
        let gamemode = self.gamemode;
        buf.put_u8(gamemode as u8);
        let dimension = self.dimension;
        buf.put_i32(dimension as i32);
        let hashed_seed = self.hashed_seed;
        buf.put_i64(hashed_seed as i64);
        let max_players = self.max_players;
        buf.put_u8(max_players as u8);
        let level_type = self.level_type;
        buf.put_string(level_type);
        let view_distance = self.view_distance;
        buf.put_i32(view_distance as i32);
        let reduced_debug_info = self.reduced_debug_info;
        buf.put_bool(reduced_debug_info as bool);
        let enable_respawn_screen = self.enable_respawn_screen;
        buf.put_bool(enable_respawn_screen as bool);
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
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let player_eid = buf.try_get_i32()?;
        let gamemode = buf.try_get_u8()?;
        let dimension = buf.try_get_i32()?;
        let hashed_seed = buf.try_get_i64()?;
        let max_players = buf.try_get_u8()?;
        let level_type = buf.try_get_string()?;
        let view_distance = buf.try_get_i32()?;
        let reduced_debug_info = buf.try_get_bool()?;
        let enable_respawn_screen = buf.try_get_bool()?;
        let packet = JoinGame {
            player_eid,
            gamemode,
            dimension,
            hashed_seed,
            max_players,
            level_type,
            view_distance,
            reduced_debug_info,
            enable_respawn_screen,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct EntityPosition {
    pub entity: i32,
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
    pub on_ground: bool,
}
impl Packet for EntityPosition {
    fn id(&self) -> u32 {
        41u32
    }
    fn name(&self) -> &'static str {
        "EntityPosition"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let dx = self.dx;
        buf.put_i16(dx as i16);
        let dy = self.dy;
        buf.put_i16(dy as i16);
        let dz = self.dz;
        buf.put_i16(dz as i16);
        let on_ground = self.on_ground;
        buf.put_bool(on_ground as bool);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EntityPositionReader;
impl Default for EntityPositionReader {
    fn default() -> Self {
        EntityPositionReader
    }
}
impl PacketReader for EntityPositionReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let dx = buf.try_get_i16()?;
        let dy = buf.try_get_i16()?;
        let dz = buf.try_get_i16()?;
        let on_ground = buf.try_get_bool()?;
        let packet = EntityPosition {
            entity,
            dx,
            dy,
            dz,
            on_ground,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct EntityPositionAndRotation {
    pub entity: i32,
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}
impl Packet for EntityPositionAndRotation {
    fn id(&self) -> u32 {
        42u32
    }
    fn name(&self) -> &'static str {
        "EntityPositionAndRotation"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let dx = self.dx;
        buf.put_i16(dx as i16);
        let dy = self.dy;
        buf.put_i16(dy as i16);
        let dz = self.dz;
        buf.put_i16(dz as i16);
        let yaw = self.yaw;
        buf.put_u8(yaw as u8);
        let pitch = self.pitch;
        buf.put_u8(pitch as u8);
        let on_ground = self.on_ground;
        buf.put_bool(on_ground as bool);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EntityPositionAndRotationReader;
impl Default for EntityPositionAndRotationReader {
    fn default() -> Self {
        EntityPositionAndRotationReader
    }
}
impl PacketReader for EntityPositionAndRotationReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let dx = buf.try_get_i16()?;
        let dy = buf.try_get_i16()?;
        let dz = buf.try_get_i16()?;
        let yaw = buf.try_get_u8()?;
        let pitch = buf.try_get_u8()?;
        let on_ground = buf.try_get_bool()?;
        let packet = EntityPositionAndRotation {
            entity,
            dx,
            dy,
            dz,
            yaw,
            pitch,
            on_ground,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct EntityRotation {
    pub entity: i32,
    pub yaw: u8,
    pub pitch: u8,
    pub on_ground: bool,
}
impl Packet for EntityRotation {
    fn id(&self) -> u32 {
        43u32
    }
    fn name(&self) -> &'static str {
        "EntityRotation"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let yaw = self.yaw;
        buf.put_u8(yaw as u8);
        let pitch = self.pitch;
        buf.put_u8(pitch as u8);
        let on_ground = self.on_ground;
        buf.put_bool(on_ground as bool);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EntityRotationReader;
impl Default for EntityRotationReader {
    fn default() -> Self {
        EntityRotationReader
    }
}
impl PacketReader for EntityRotationReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let yaw = buf.try_get_u8()?;
        let pitch = buf.try_get_u8()?;
        let on_ground = buf.try_get_bool()?;
        let packet = EntityRotation {
            entity,
            yaw,
            pitch,
            on_ground,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct EntityMovement {
    pub entity: i32,
}
impl Packet for EntityMovement {
    fn id(&self) -> u32 {
        44u32
    }
    fn name(&self) -> &'static str {
        "EntityMovement"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let entity = self.entity;
        buf.put_i32(entity as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct EntityMovementReader;
impl Default for EntityMovementReader {
    fn default() -> Self {
        EntityMovementReader
    }
}
impl PacketReader for EntityMovementReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let entity = buf.try_get_i32()?;
        let packet = EntityMovement { entity };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct VehicleMove {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}
impl Packet for VehicleMove {
    fn id(&self) -> u32 {
        45u32
    }
    fn name(&self) -> &'static str {
        "VehicleMove"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let x = self.x;
        buf.put_f64(x as f64);
        let y = self.y;
        buf.put_f64(y as f64);
        let z = self.z;
        buf.put_f64(z as f64);
        let yaw = self.yaw;
        buf.put_f32(yaw as f32);
        let pitch = self.pitch;
        buf.put_f32(pitch as f32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct VehicleMoveReader;
impl Default for VehicleMoveReader {
    fn default() -> Self {
        VehicleMoveReader
    }
}
impl PacketReader for VehicleMoveReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let x = buf.try_get_f64()?;
        let y = buf.try_get_f64()?;
        let z = buf.try_get_f64()?;
        let yaw = buf.try_get_f32()?;
        let pitch = buf.try_get_f32()?;
        let packet = VehicleMove {
            x,
            y,
            z,
            yaw,
            pitch,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct OpenBook {
    pub hand: i32,
}
impl Packet for OpenBook {
    fn id(&self) -> u32 {
        46u32
    }
    fn name(&self) -> &'static str {
        "OpenBook"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let hand = self.hand;
        buf.put_i32(hand as i32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OpenBookReader;
impl Default for OpenBookReader {
    fn default() -> Self {
        OpenBookReader
    }
}
impl PacketReader for OpenBookReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let hand = buf.try_get_i32()?;
        let packet = OpenBook { hand };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct OpenWindow {
    pub window_id: i32,
    pub ty: i32,
    pub title: String,
}
impl Packet for OpenWindow {
    fn id(&self) -> u32 {
        47u32
    }
    fn name(&self) -> &'static str {
        "OpenWindow"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let window_id = self.window_id;
        buf.put_i32(window_id as i32);
        let ty = self.ty;
        buf.put_i32(ty as i32);
        let title = self.title;
        buf.put_string(title);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OpenWindowReader;
impl Default for OpenWindowReader {
    fn default() -> Self {
        OpenWindowReader
    }
}
impl PacketReader for OpenWindowReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let window_id = buf.try_get_i32()?;
        let ty = buf.try_get_i32()?;
        let title = buf.try_get_string()?;
        let packet = OpenWindow {
            window_id,
            ty,
            title,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct OpenSignEditor {
    pub position: BlockPosition,
}
impl Packet for OpenSignEditor {
    fn id(&self) -> u32 {
        48u32
    }
    fn name(&self) -> &'static str {
        "OpenSignEditor"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let position = self.position;
        buf.put_position(position);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct OpenSignEditorReader;
impl Default for OpenSignEditorReader {
    fn default() -> Self {
        OpenSignEditorReader
    }
}
impl PacketReader for OpenSignEditorReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let position = buf.try_get_position()?;
        let packet = OpenSignEditor { position };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct CraftRecipeResponse {
    pub window_id: u8,
    pub recipe: String,
}
impl Packet for CraftRecipeResponse {
    fn id(&self) -> u32 {
        49u32
    }
    fn name(&self) -> &'static str {
        "CraftRecipeResponse"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let recipe = self.recipe;
        buf.put_string(recipe);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CraftRecipeResponseReader;
impl Default for CraftRecipeResponseReader {
    fn default() -> Self {
        CraftRecipeResponseReader
    }
}
impl PacketReader for CraftRecipeResponseReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let window_id = buf.try_get_u8()?;
        let recipe = buf.try_get_string()?;
        let packet = CraftRecipeResponse { window_id, recipe };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct PlayerAbilities {
    pub flags: i8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}
impl Packet for PlayerAbilities {
    fn id(&self) -> u32 {
        50u32
    }
    fn name(&self) -> &'static str {
        "PlayerAbilities"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let flags = self.flags;
        buf.put_i8(flags as i8);
        let flying_speed = self.flying_speed;
        buf.put_f32(flying_speed as f32);
        let fov_modifier = self.fov_modifier;
        buf.put_f32(fov_modifier as f32);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct PlayerAbilitiesReader;
impl Default for PlayerAbilitiesReader {
    fn default() -> Self {
        PlayerAbilitiesReader
    }
}
impl PacketReader for PlayerAbilitiesReader {
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let flags = buf.try_get_i8()?;
        let flying_speed = buf.try_get_f32()?;
        let fov_modifier = buf.try_get_f32()?;
        let packet = PlayerAbilities {
            flags,
            flying_speed,
            fov_modifier,
        };
        Ok(smallbox::smallbox!(packet))
    }
}
#[derive(Debug, Clone)]
pub struct CombatEvent<P: Provider> {
    pub event: CombatEventType<P>,
}
impl<P> Packet for CombatEvent<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        51u32
    }
    fn name(&self) -> &'static str {
        "CombatEvent"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let version = VERSION;
        let event_id = self.event.repr() as i32;
        buf.put_i32(event_id as i32);
        let event = self.event;
        event.write_to(buf);
    }
}
#[derive(Debug, Clone, Copy)]
pub struct CombatEventReader<P>(std::marker::PhantomData<P>);
impl<P> Default for CombatEventReader<P>
where
    P: Provider,
{
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}
impl<P> PacketReader for CombatEventReader<P>
where
    P: Provider,
{
    fn read_from(buf: &mut Bytes) -> anyhow::Result<DynPacket> {
        let version = VERSION;
        let event_repr = buf.try_get_i32()?;
        let event = CombatEventType::<P>::read_from(buf, event_repr as i64)?;
        let packet = CombatEvent::<P> { event };
        Ok(smallbox::smallbox!(packet))
    }
}
