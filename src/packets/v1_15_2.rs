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
    category_id: i32,
    statistic_id: i32,
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
    horizontal_position: u8,
    vertical_position: u8,
    block: P::Block,
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
    name: String,
    tooltip: Option<String>,
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
    x: i8,
    y: i8,
    z: i8,
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
#[derive(Debug, Clone)]
pub struct SpawnObject {
    entity: i32,
    uuid: uuid::Uuid,
    ty: i32,
    x: f64,
    y: f64,
    z: f64,
    pitch: u8,
    yaw: u8,
    data: i32,
    dx: i16,
    dy: i16,
    dz: i16,
}
impl SpawnObject {
    fn id(&self) -> u32 {
        0u32
    }
    fn name(&self) -> &'static str {
        "SpawnObject"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct SpawnExperienceOrb {
    entity: i32,
    x: f64,
    y: f64,
    z: f64,
    count: i16,
}
impl SpawnExperienceOrb {
    fn id(&self) -> u32 {
        1u32
    }
    fn name(&self) -> &'static str {
        "SpawnExperienceOrb"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct SpawnGlobalEntity<P: Provider> {
    entity: i32,
    ty: SpawnGlobalEntityType<P>,
}
impl<P> SpawnGlobalEntity<P>
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
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let ty = self.ty;
        let ty = ty.repr();
        buf.put_u8(ty as u8);
    }
}
#[derive(Debug, Clone)]
pub struct SpawnMob {
    entity: i32,
    uuid: uuid::Uuid,
    ty: i32,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
    head_pitch: u8,
    dx: i16,
    dy: i16,
    dz: i16,
}
impl SpawnMob {
    fn id(&self) -> u32 {
        3u32
    }
    fn name(&self) -> &'static str {
        "SpawnMob"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct SpawnPainting<P: Provider> {
    entity: i32,
    uuid: uuid::Uuid,
    motive: i32,
    position: BlockPosition,
    direction: SpawnPaintingDirection<P>,
}
impl<P> SpawnPainting<P>
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
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let uuid = self.uuid;
        buf.put_uuid(uuid);
        let motive = self.motive;
        buf.put_i32(motive as i32);
        let position = self.position;
        buf.put_BlockPosition(position as BlockPosition);
        let direction = self.direction;
        let direction = direction.repr();
        buf.put_u8(direction as u8);
    }
}
#[derive(Debug, Clone)]
pub struct SpawnPlayer {
    entity: i32,
    uuid: uuid::Uuid,
    x: f64,
    y: f64,
    z: f64,
    yaw: u8,
    pitch: u8,
}
impl SpawnPlayer {
    fn id(&self) -> u32 {
        5u32
    }
    fn name(&self) -> &'static str {
        "SpawnPlayer"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct EntityAnimation<P: Provider> {
    entity: i32,
    animation: EntityAnimationType<P>,
}
impl<P> EntityAnimation<P>
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
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let animation = self.animation;
        let animation = animation.repr();
        buf.put_u8(animation as u8);
    }
}
#[derive(Debug, Clone)]
pub struct Statistics<P: Provider> {
    count: i32,
    statistics: Vec<Statistic<P>>,
    value: i32,
}
impl<P> Statistics<P>
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
        let count = self.statistics.len() as i32;
        buf.put_i32(count as i32);
        let statistics = self.statistics;
        statistics.iter().for_each(|x| x.write_to(buf));
        let value = self.value;
        buf.put_i32(value as i32);
    }
}
#[derive(Debug, Clone)]
pub struct AcknowledgePlayerDigging<P: Provider> {
    position: BlockPosition,
    block: P::Block,
    status: AcknowledgePlayerDiggingStatus<P>,
    successful: bool,
}
impl<P> AcknowledgePlayerDigging<P>
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
        let position = self.position;
        buf.put_BlockPosition(position as BlockPosition);
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
#[derive(Debug, Clone)]
pub struct BlockBreakAnimation {
    breaker: i32,
    position: BlockPosition,
    destroy_stage: u8,
}
impl BlockBreakAnimation {
    fn id(&self) -> u32 {
        9u32
    }
    fn name(&self) -> &'static str {
        "BlockBreakAnimation"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let breaker = self.breaker;
        buf.put_i32(breaker as i32);
        let position = self.position;
        buf.put_BlockPosition(position as BlockPosition);
        let destroy_stage = self.destroy_stage;
        buf.put_u8(destroy_stage as u8);
    }
}
#[derive(Debug, Clone)]
pub struct UpdateBlockEntity<P: Provider> {
    os: BlockPosition,
    action: UpdateBlockEntityAction<P>,
    data: nbt::Blob,
}
impl<P> UpdateBlockEntity<P>
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
        let os = self.os;
        buf.put_BlockPosition(os as BlockPosition);
        let action = self.action;
        let action = action.repr();
        buf.put_u8(action as u8);
        let data = self.data;
        buf.put_nbt(data);
    }
}
#[derive(Debug, Clone)]
pub struct BlockAction<P: Provider> {
    position: BlockPosition,
    action_id: u8,
    action_param: u8,
    block_type: P::Block,
}
impl<P> BlockAction<P>
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
        let position = self.position;
        buf.put_BlockPosition(position as BlockPosition);
        let action_id = self.action_id;
        buf.put_u8(action_id as u8);
        let action_param = self.action_param;
        buf.put_u8(action_param as u8);
        let block_type = self.block_type;
        let block_type = P::block_ty(block_type, version);
        buf.put_i32(block_type as i32);
    }
}
#[derive(Debug, Clone)]
pub struct BlockChange<P: Provider> {
    position: BlockPosition,
    block: P::Block,
}
impl<P> BlockChange<P>
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
        let position = self.position;
        buf.put_BlockPosition(position as BlockPosition);
        let block = self.block;
        let block = P::block_id(block, version);
        buf.put_i32(block as i32);
    }
}
#[derive(Debug, Clone)]
pub struct BossBar<P: Provider> {
    uuid: uuid::Uuid,
    action: i32,
    data: BossBarData<P>,
}
impl<P> BossBar<P>
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
        let uuid = self.uuid;
        buf.put_uuid(uuid);
        let action = self.data.repr() as i32;
        buf.put_i32(action as i32);
        let data = self.data;
        data.write_to(buf);
    }
}
#[derive(Debug, Clone)]
pub struct ServerDifficulty {
    difficulty: u8,
    difficulty_locked: bool,
}
impl ServerDifficulty {
    fn id(&self) -> u32 {
        14u32
    }
    fn name(&self) -> &'static str {
        "ServerDifficulty"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let difficulty = self.difficulty;
        buf.put_u8(difficulty as u8);
        let difficulty_locked = self.difficulty_locked;
        buf.put_bool(difficulty_locked as bool);
    }
}
#[derive(Debug, Clone)]
pub struct ChatMessage<P: Provider> {
    data: String,
    position: ChatMessagePosition<P>,
}
impl<P> ChatMessage<P>
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
        let data = self.data;
        buf.put_string(data);
        let position = self.position;
        let position = position.repr();
        buf.put_u8(position as u8);
    }
}
#[derive(Debug, Clone)]
pub struct MultiBlockChange<P: Provider> {
    chunk_x: i32,
    chunk_z: i32,
    count: i32,
    records: Vec<MultiBlockChangeRecord<P>>,
}
impl<P> MultiBlockChange<P>
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
        let chunk_x = self.chunk_x;
        buf.put_i32(chunk_x as i32);
        let chunk_z = self.chunk_z;
        buf.put_i32(chunk_z as i32);
        let count = self.records.len() as i32;
        buf.put_i32(count as i32);
        let records = self.records;
        records.iter().for_each(|x| x.write_to(buf));
    }
}
#[derive(Debug, Clone)]
pub struct TabComplete<P: Provider> {
    transaction_id: i32,
    start: i32,
    length: i32,
    count: i32,
    matches: Vec<TabCompleteMatch<P>>,
}
impl<P> TabComplete<P>
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
        let transaction_id = self.transaction_id;
        buf.put_i32(transaction_id as i32);
        let start = self.start;
        buf.put_i32(start as i32);
        let length = self.length;
        buf.put_i32(length as i32);
        let count = self.matches.len() as i32;
        buf.put_i32(count as i32);
        let matches = self.matches;
        matches.iter().for_each(|x| x.write_to(buf));
    }
}
#[derive(Debug, Clone)]
pub struct DeclareCommands {
    count: i32,
    nodes: Vec<Node>,
    root_index: i32,
}
impl DeclareCommands {
    fn id(&self) -> u32 {
        18u32
    }
    fn name(&self) -> &'static str {
        "DeclareCommands"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let count = self.nodes.len() as i32;
        buf.put_i32(count as i32);
        let nodes = self.nodes;
        nodes.iter().for_each(|x| buf.put_Node(x as Node));
        let root_index = self.root_index;
        buf.put_i32(root_index as i32);
    }
}
#[derive(Debug, Clone)]
pub struct WindowConfirmation {
    window_id: u8,
    action_number: i16,
    accepted: bool,
}
impl WindowConfirmation {
    fn id(&self) -> u32 {
        19u32
    }
    fn name(&self) -> &'static str {
        "WindowConfirmation"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let action_number = self.action_number;
        buf.put_i16(action_number as i16);
        let accepted = self.accepted;
        buf.put_bool(accepted as bool);
    }
}
#[derive(Debug, Clone)]
pub struct CloseWindow {
    window_id: u8,
}
impl CloseWindow {
    fn id(&self) -> u32 {
        20u32
    }
    fn name(&self) -> &'static str {
        "CloseWindow"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
    }
}
#[derive(Debug, Clone)]
pub struct WindowItems {
    window_id: u8,
    count: i16,
    slots: Vec<Slot>,
}
impl WindowItems {
    fn id(&self) -> u32 {
        21u32
    }
    fn name(&self) -> &'static str {
        "WindowItems"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let count = self.slots.len() as i16;
        buf.put_i16(count as i16);
        let slots = self.slots;
        slots.iter().for_each(|x| buf.put_Slot(x as Slot));
    }
}
#[derive(Debug, Clone)]
pub struct WindowProperty {
    window_id: u8,
    property: i16,
    value: i16,
}
impl WindowProperty {
    fn id(&self) -> u32 {
        22u32
    }
    fn name(&self) -> &'static str {
        "WindowProperty"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let property = self.property;
        buf.put_i16(property as i16);
        let value = self.value;
        buf.put_i16(value as i16);
    }
}
#[derive(Debug, Clone)]
pub struct SetSlot {
    window_id: u8,
    slot_index: i16,
    slot: Slot,
}
impl SetSlot {
    fn id(&self) -> u32 {
        23u32
    }
    fn name(&self) -> &'static str {
        "SetSlot"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let slot_index = self.slot_index;
        buf.put_i16(slot_index as i16);
        let slot = self.slot;
        buf.put_Slot(slot as Slot);
    }
}
#[derive(Debug, Clone)]
pub struct SetCooldown<P: Provider> {
    item: P::Item,
    cooldown_ticks: i32,
}
impl<P> SetCooldown<P>
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
        let item = self.item;
        let item = P::item_id(item, version);
        buf.put_i32(item as i32);
        let cooldown_ticks = self.cooldown_ticks;
        buf.put_i32(cooldown_ticks as i32);
    }
}
#[derive(Debug, Clone)]
pub struct PluginMessage {
    channel: String,
    data: Vec<i8>,
}
impl PluginMessage {
    fn id(&self) -> u32 {
        25u32
    }
    fn name(&self) -> &'static str {
        "PluginMessage"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let channel = self.channel;
        buf.put_string(channel);
        let data = self.data;
        data.iter().for_each(|x| buf.put_i8(x as i8));
    }
}
#[derive(Debug, Clone)]
pub struct NamedSoundEffect {
    name: String,
    category: i32,
    posx: i32,
    posy: i32,
    posz: i32,
    volume: f32,
    pitch: f32,
}
impl NamedSoundEffect {
    fn id(&self) -> u32 {
        26u32
    }
    fn name(&self) -> &'static str {
        "NamedSoundEffect"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct Disconnect {
    reason: String,
}
impl Disconnect {
    fn id(&self) -> u32 {
        27u32
    }
    fn name(&self) -> &'static str {
        "Disconnect"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let reason = self.reason;
        buf.put_string(reason);
    }
}
#[derive(Debug, Clone)]
pub struct EntityStatus {
    entity: i32,
    status: i8,
}
impl EntityStatus {
    fn id(&self) -> u32 {
        28u32
    }
    fn name(&self) -> &'static str {
        "EntityStatus"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let entity = self.entity;
        buf.put_i32(entity as i32);
        let status = self.status;
        buf.put_i8(status as i8);
    }
}
#[derive(Debug, Clone)]
pub struct Explosion<P: Provider> {
    x: f32,
    y: f32,
    z: f32,
    strength: f32,
    record_count: i32,
    records: Vec<ExplosionRecord<P>>,
    player_dx: f32,
    player_dy: f32,
    player_dz: f32,
}
impl<P> Explosion<P>
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
        records.iter().for_each(|x| x.write_to(buf));
        let player_dx = self.player_dx;
        buf.put_f32(player_dx as f32);
        let player_dy = self.player_dy;
        buf.put_f32(player_dy as f32);
        let player_dz = self.player_dz;
        buf.put_f32(player_dz as f32);
    }
}
#[derive(Debug, Clone)]
pub struct UnloadChunk {
    x: i32,
    z: i32,
}
impl UnloadChunk {
    fn id(&self) -> u32 {
        30u32
    }
    fn name(&self) -> &'static str {
        "UnloadChunk"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let x = self.x;
        buf.put_i32(x as i32);
        let z = self.z;
        buf.put_i32(z as i32);
    }
}
#[derive(Debug, Clone)]
pub struct ChangeGameState<P: Provider> {
    reason: ChangeGameStateReason<P>,
    value: f32,
}
impl<P> ChangeGameState<P>
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
        let reason = self.reason;
        let reason = reason.repr();
        buf.put_u8(reason as u8);
        let value = self.value;
        buf.put_f32(value as f32);
    }
}
#[derive(Debug, Clone)]
pub struct OpenHorseWindow {
    window_id: u8,
    num_slots: i32,
    entity: i32,
}
impl OpenHorseWindow {
    fn id(&self) -> u32 {
        32u32
    }
    fn name(&self) -> &'static str {
        "OpenHorseWindow"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let num_slots = self.num_slots;
        buf.put_i32(num_slots as i32);
        let entity = self.entity;
        buf.put_i32(entity as i32);
    }
}
#[derive(Debug, Clone)]
pub struct KeepAlive {
    id: i64,
}
impl KeepAlive {
    fn id(&self) -> u32 {
        33u32
    }
    fn name(&self) -> &'static str {
        "KeepAlive"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let id = self.id;
        buf.put_i64(id as i64);
    }
}
#[derive(Debug, Clone)]
pub struct ChunkData {}
impl ChunkData {
    fn id(&self) -> u32 {
        34u32
    }
    fn name(&self) -> &'static str {
        "ChunkData"
    }
    fn write_to(self, buf: &mut BytesMut) {}
}
#[derive(Debug, Clone)]
pub struct Effect {
    id: i32,
    position: BlockPosition,
    data: i32,
    disable_relative_volume: bool,
}
impl Effect {
    fn id(&self) -> u32 {
        35u32
    }
    fn name(&self) -> &'static str {
        "Effect"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let id = self.id;
        buf.put_i32(id as i32);
        let position = self.position;
        buf.put_BlockPosition(position as BlockPosition);
        let data = self.data;
        buf.put_i32(data as i32);
        let disable_relative_volume = self.disable_relative_volume;
        buf.put_bool(disable_relative_volume as bool);
    }
}
#[derive(Debug, Clone)]
pub struct UpdateLight {}
impl UpdateLight {
    fn id(&self) -> u32 {
        36u32
    }
    fn name(&self) -> &'static str {
        "UpdateLight"
    }
    fn write_to(self, buf: &mut BytesMut) {}
}
#[derive(Debug, Clone)]
pub struct JoinGame {
    player_eid: i32,
    gamemode: u8,
    dimension: i32,
    hashed_seed: i64,
    max_players: u8,
    level_type: String,
    view_distance: i32,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
}
impl JoinGame {
    fn id(&self) -> u32 {
        37u32
    }
    fn name(&self) -> &'static str {
        "JoinGame"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct EntityPosition {
    entity: i32,
    dx: i16,
    dy: i16,
    dz: i16,
    on_ground: bool,
}
impl EntityPosition {
    fn id(&self) -> u32 {
        38u32
    }
    fn name(&self) -> &'static str {
        "EntityPosition"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct EntityPositionAndRotation {
    entity: i32,
    dx: i16,
    dy: i16,
    dz: i16,
    yaw: u8,
    pitch: u8,
    on_ground: bool,
}
impl EntityPositionAndRotation {
    fn id(&self) -> u32 {
        39u32
    }
    fn name(&self) -> &'static str {
        "EntityPositionAndRotation"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct EntityRotation {
    entity: i32,
    yaw: u8,
    pitch: u8,
    on_ground: bool,
}
impl EntityRotation {
    fn id(&self) -> u32 {
        40u32
    }
    fn name(&self) -> &'static str {
        "EntityRotation"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct EntityMovement {
    entity: i32,
}
impl EntityMovement {
    fn id(&self) -> u32 {
        41u32
    }
    fn name(&self) -> &'static str {
        "EntityMovement"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let entity = self.entity;
        buf.put_i32(entity as i32);
    }
}
#[derive(Debug, Clone)]
pub struct VehicleMove {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
}
impl VehicleMove {
    fn id(&self) -> u32 {
        42u32
    }
    fn name(&self) -> &'static str {
        "VehicleMove"
    }
    fn write_to(self, buf: &mut BytesMut) {
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
#[derive(Debug, Clone)]
pub struct OpenBook {
    hand: i32,
}
impl OpenBook {
    fn id(&self) -> u32 {
        43u32
    }
    fn name(&self) -> &'static str {
        "OpenBook"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let hand = self.hand;
        buf.put_i32(hand as i32);
    }
}
#[derive(Debug, Clone)]
pub struct OpenWindow {
    window_id: i32,
    ty: i32,
    title: String,
}
impl OpenWindow {
    fn id(&self) -> u32 {
        44u32
    }
    fn name(&self) -> &'static str {
        "OpenWindow"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let window_id = self.window_id;
        buf.put_i32(window_id as i32);
        let ty = self.ty;
        buf.put_i32(ty as i32);
        let title = self.title;
        buf.put_string(title);
    }
}
#[derive(Debug, Clone)]
pub struct OpenSignEditor {
    position: BlockPosition,
}
impl OpenSignEditor {
    fn id(&self) -> u32 {
        45u32
    }
    fn name(&self) -> &'static str {
        "OpenSignEditor"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let position = self.position;
        buf.put_BlockPosition(position as BlockPosition);
    }
}
#[derive(Debug, Clone)]
pub struct CraftRecipeResponse {
    window_id: u8,
    recipe: String,
}
impl CraftRecipeResponse {
    fn id(&self) -> u32 {
        46u32
    }
    fn name(&self) -> &'static str {
        "CraftRecipeResponse"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let window_id = self.window_id;
        buf.put_u8(window_id as u8);
        let recipe = self.recipe;
        buf.put_string(recipe);
    }
}
#[derive(Debug, Clone)]
pub struct PlayerAbilities {
    flags: i8,
    flying_speed: f32,
    fov_modifier: f32,
}
impl PlayerAbilities {
    fn id(&self) -> u32 {
        47u32
    }
    fn name(&self) -> &'static str {
        "PlayerAbilities"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let flags = self.flags;
        buf.put_i8(flags as i8);
        let flying_speed = self.flying_speed;
        buf.put_f32(flying_speed as f32);
        let fov_modifier = self.fov_modifier;
        buf.put_f32(fov_modifier as f32);
    }
}
#[derive(Debug, Clone)]
pub struct CombatEvent<P: Provider> {
    event_id: i32,
    event: CombatEventType<P>,
}
impl<P> CombatEvent<P>
where
    P: Provider,
{
    fn id(&self) -> u32 {
        48u32
    }
    fn name(&self) -> &'static str {
        "CombatEvent"
    }
    fn write_to(self, buf: &mut BytesMut) {
        let event_id = self.event.repr() as i32;
        buf.put_i32(event_id as i32);
        let event = self.event;
        event.write_to(buf);
    }
}
#[derive(Debug, Clone)]
pub struct PlayerInfo {}
impl PlayerInfo {
    fn id(&self) -> u32 {
        49u32
    }
    fn name(&self) -> &'static str {
        "PlayerInfo"
    }
    fn write_to(self, buf: &mut BytesMut) {}
}
