// This is GENERATED CODE. Do not edit.
use crate::{BytesExt, BytesMutExt, Provider};
use bytes::{Buf, BufMut, Bytes, BytesMut};
const VERSION: crate::ProtocolVersion = ProtocolVersion::V1_15_2;
pub enum SpawnGlobalEntityType<P: Provider> {
    Thunderbolt {},
}
pub enum SpawnPaintingDirection<P: Provider> {
    North {},
    South {},
    West {},
    East {},
}
pub enum EntityAnimationType<P: Provider> {
    SwingMainArm {},
    TakeDamage {},
    LeaveBed {},
    SwingOffhand {},
    CriticalEffect {},
    MagicCriticalEffect {},
}
pub struct Statistic<P: Provider> {
    category_id: i32,
    statistic_id: i32,
}
impl<P> Statistic<P>
where
    P: Provider,
{
    pub fn read_from(buf: &mut bytes::Bytes) -> anyhow::Result<Self> {
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
pub enum BossBarData<P: Provider> {
    Add {
        title: String,
        health: f32,
        color: BossBarColor,
        division: BossBarDivision,
        flags: BossBarFlags,
    },
    Remove {},
    UpdateHealth {
        health: f32,
    },
    UpdateTitle {
        title: String,
    },
    UpdateStyle {
        color: BossBarColor,
        division: BossBarDivision,
    },
    UpdateFlags {
        flags: BossBarFlags,
    },
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
pub enum BossBarDivision<P: Provider> {
    None {},
    Six {},
    Ten {},
    Twelve {},
    Twenty {},
}
pub enum ChatMessagePosition<P: Provider> {
    Chat {},
    SystemMessage {},
    GameInfo {},
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
    pub fn read_from(buf: &mut bytes::Bytes) -> anyhow::Result<Self> {
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
    pub fn read_from(buf: &mut bytes::Bytes) -> anyhow::Result<Self> {
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
    pub fn read_from(buf: &mut bytes::Bytes) -> anyhow::Result<Self> {
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
pub enum JoinGameLevelType<P: Provider> {
    Default {},
    Flat {},
    LargeBiomes {},
    Amplified {},
    Customized {},
    Buffet {},
    Default11 {},
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
