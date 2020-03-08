// This is GENERATED CODE. Do not edit.
#![allow(warnings)]
use crate::{BytesExt, BytesMutExt, Error, ProtocolVersion, Provider};
use bytes::{Buf, BufMut, Bytes, BytesMut};
const VERSION: ProtocolVersion = ProtocolVersion::V1_15_2;
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
