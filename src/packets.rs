//! Defines all packet structs.

use feather_protocol_macros::packets;

pub use __packets::{recv, send};

#[packets("1.13.2", "1.14.4", "1.15.2", "1.16.1")]
mod __packets {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    #[tag_type = "varint"]
    pub enum Hand {
        Left,
        Right,
    }

    pub mod serverbound {
        pub mod handshake {
            #[derive(Debug, Clone)]
            #[packet(id = 0x00)]
            pub struct Handshake {
                #[varint]
                pub protocol_version: u32,
                #[max_len = 255]
                pub server_address: String,
                pub server_port: u16,
                pub next_state: HandshakeState,
            }

            #[derive(Debug, Clone)]
            #[tag_type = "varint"]
            pub enum HandshakeState {
                Status,
                Login,
            }
        }

        pub mod login {
            #[derive(Debug, Clone)]
            #[packet(id = 0x00)]
            pub struct LoginStart {
                #[max_len = "16"]
                pub username: String,
            }

            #[derive(Debug, Clone)]
            #[packet(id = 0x01)]
            pub struct EncryptionResponse {
                #[prefixed_len = "varint"]
                pub secret: Vec<u8>,
                #[prefixed_len = "varint"]
                pub verify_token: Vec<u8>,
            }
        }

        pub mod status {
            #[derive(Debug, Clone)]
            #[packet(id = 0x00)]
            pub struct Request;

            #[derive(Debug, Clone)]
            #[packet(id = 0x01)]
            pub struct Ping {
                pub payload: i64,
            }
        }

        pub mod play {
            #[derive(Debug, Clone)]
            #[packet(id = 0x00)]
            pub struct TeleportConfirm {
                #[varint]
                pub teleport_id: i32,
            }

            #[derive(Debug, Clone)]
            #[packet(id = 0x01)]
            pub struct QueryBlockNBT {
                #[varint]
                pub transaction_id: i32,
                pub location: BlockPosition,
            }

            #[derive(Debug, Clone)]
            #[packet(id = 0x03)]
            pub struct ChatMessage {
                /// Raw string, not a chat component
                #[max_len = 256]
                pub message: String,
            }

            #[derive(Debug, Clone)]
            #[packet(id = 0x04)]
            pub struct ClientStatus {
                pub kind: ClientStatusKind,
            }

            #[derive(Debug, Clone)]
            #[tag_type = "varint"]
            pub enum ClientStatusKind {
                PerformRespawn,
                RequestStats,
            }

            #[derive(Debug, Clone)]
            #[packet(id = 0x05)]
            pub struct ClientSettings {
                #[max_len = 16]
                pub locale: String,
                pub view_distance: i8,
                #[varint]
                pub chat_mode: i32,
                pub chat_colors: bool,
                pub displayed_skin_parts: u8,
                #[varint]
                pub main_hand: Hand,
            }

            #[derive(Debug, Clone)]
            #[tag_type = "varint"]
            pub enum ChatMode {
                Enabled,
                CommandsOnly,
                Hidden,
            }

            #[derive(Debug, Clone)]
            pub struct TabCompleteServerbound {
                #[varint]
                pub transaction_id: i32,
                pub text: String,
            }

            #[derive(Debug, Clone)]
            pub struct ConfirmTransactionServerbound {
                pub window_id: u8,
                pub action_number: u16,
                pub accepted: bool,
            }

            #[derive(Debug, Clone)]
            pub struct EnchantItem {
                pub window_id: u8,
                pub enchantment: u8,
            }

            #[derive(Debug, Clone)]
            pub struct ClickWindow {
                pub window_id: u8,
                pub slot: i16,
                pub button: u8,
                pub action_number: i16,
                #[varint]
                pub mode: i32,
                pub clicked_item: Slot,
            }

            #[derive(Debug, Clone)]
            pub struct CloseWindowServerbound {
                pub window_id: u8,
            }

            #[derive(Debug, Clone)]
            pub struct PluginMessageServerbound {
                pub channel: String,
                pub data: Vec<u8>,
            }

            impl Packet for PluginMessageServerbound {
                fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
                    self.channel = buf.try_get_string()?;

                    let mut data = Vec::with_capacity(buf.remaining());
                    buf.read(&mut data)
                        .map_err(|_| Error::InsufficientArrayLength)?;
                    self.data = data;

                    Ok(())
                }

                fn write_to(&self, buf: &mut BytesMut) {
                    buf.push_string(&self.channel);

                    buf.put(self.data.as_slice());
                }

                fn ty(&self) -> PacketType {
                    PacketType::PluginMessageServerbound
                }

                fn ty_sized() -> PacketType
                where
                    Self: Sized,
                {
                    PacketType::PluginMessageServerbound
                }

                fn box_clone(&self) -> Box<dyn Packet> {
                    box_clone_impl!(self);
                }
            }

            #[derive(Debug, Clone)]
            pub struct EditBook {
                pub new_book: Slot,
                pub is_signing: bool,
                #[varint]
                pub hand: i32,
            }

            #[derive(Debug, Clone)]
            pub struct QueryEntityNBT {
                #[varint]
                pub transaction_id: i32,
                #[varint]
                pub entity_id: i32,
            }

            #[derive(Debug, Clone)]
            pub struct UseEntity {
                #[varint]
                pub target: i32,
                pub ty: UseEntityType,
            }

            impl Packet for UseEntity {
                fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
                    self.target = buf.try_get_var_int()?;

                    let ty_id = buf.try_get_var_int()?;
                    self.ty = match ty_id {
                        0 => UseEntityType::Interact,
                        1 => UseEntityType::Attack,
                        2 => {
                            let x = buf.try_get_f32()?;
                            let y = buf.try_get_f32()?;
                            let z = buf.try_get_f32()?;
                            let hand = buf.try_get_var_int()?;
                            UseEntityType::InteractAt(x, y, z, hand)
                        }
                        i => return Err(Error::InvalidUseEntity(i).into()), // Invalid type
                    };

                    Ok(())
                }

                fn write_to(&self, buf: &mut BytesMut) {
                    buf.push_var_int(self.target);

                    let ty_id = match self.ty {
                        UseEntityType::Interact => 0,
                        UseEntityType::Attack => 1,
                        UseEntityType::InteractAt(_, _, _, _) => 2,
                    };
                    buf.push_var_int(ty_id);

                    if let UseEntityType::InteractAt(x, y, z, hand) = self.ty {
                        buf.push_f32(x);
                        buf.push_f32(y);
                        buf.push_f32(z);
                        buf.push_var_int(hand);
                    }
                }

                fn ty(&self) -> PacketType {
                    PacketType::UseEntity
                }

                fn ty_sized() -> PacketType
                where
                    Self: Sized,
                {
                    PacketType::UseEntity
                }

                fn box_clone(&self) -> Box<dyn Packet> {
                    box_clone_impl!(self);
                }
            }

            #[derive(Debug, Clone)]
            pub enum UseEntityType {
                Interact,
                Attack,
                InteractAt(f32, f32, f32, VarInt),
            }

            impl Default for UseEntityType {
                fn default() -> Self {
                    UseEntityType::Interact
                }
            }

            #[derive(Debug, Clone)]
            pub struct KeepAliveServerbound {
                pub id: i64,
            }

            #[derive(Debug, Clone)]
            pub struct Player {
                pub on_ground: bool,
            }

            #[derive(Debug, Clone)]
            pub struct PlayerPosition {
                pub x: f64,
                pub feet_y: f64,
                pub z: f64,
                pub on_ground: bool,
            }

            #[derive(Debug, Clone)]
            pub struct PlayerPositionAndLookServerbound {
                pub x: f64,
                pub feet_y: f64,
                pub z: f64,
                pub yaw: f32,
                pub pitch: f32,
                pub on_ground: bool,
            }

            #[derive(Debug, Clone)]
            pub struct PlayerLook {
                pub yaw: f32,
                pub pitch: f32,
                pub on_ground: bool,
            }

            #[derive(Debug, Clone)]
            pub struct VehicleMoveServerbound {
                pub x: f64,
                pub y: f64,
                pub z: f64,
                pub yaw: f32,
                pub pitch: f32,
            }

            #[derive(Debug, Clone)]
            pub struct SteerBoat {
                pub left_paddle_turning: bool,
                pub right_paddle_turning: bool,
            }

            #[derive(Debug, Clone)]
            pub struct PickItem {
                #[varint]
                pub slot_to_use: i32,
            }

            #[derive(Debug, Clone)]
            pub struct CraftRecipeRequest {
                pub window_id: i8,
                pub recipe: String,
                pub make_all: bool,
            }

            #[derive(Debug, Clone)]
            pub struct PlayerAbilitiesServerbound {
                pub flags: u8,
                pub flying_speed: f32,
                pub walking_speed: f32,
            }

            #[derive(Debug, Clone)]
            pub struct PlayerDigging {
                pub status: PlayerDiggingStatus,
                pub location: BlockPosition,
                pub face: i8,
            }

            impl Packet for PlayerDigging {
                fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
                    self.status = {
                        let id = buf.try_get_var_int()?;
                        match id {
                            0 => PlayerDiggingStatus::StartedDigging,
                            1 => PlayerDiggingStatus::CancelledDigging,
                            2 => PlayerDiggingStatus::FinishedDigging,
                            3 => PlayerDiggingStatus::DropItemStack,
                            4 => PlayerDiggingStatus::DropItem,
                            5 => PlayerDiggingStatus::ConsumeItem,
                            6 => PlayerDiggingStatus::SwapItemInHand,
                            i => return Err(Error::InvalidPlayerDiggingStatus(i).into()),
                        }
                    };

                    self.location = buf.try_get_position()?;
                    self.face = buf.try_get_i8()?;

                    Ok(())
                }

                fn write_to(&self, buf: &mut BytesMut) {
                    let id = self.status as i32;
                    buf.push_var_int(id);
                    buf.push_position(&self.location);
                    buf.push_i8(self.face);
                }

                fn ty(&self) -> PacketType {
                    PacketType::PlayerDigging
                }

                fn ty_sized() -> PacketType
                where
                    Self: Sized,
                {
                    PacketType::PlayerDigging
                }

                fn box_clone(&self) -> Box<dyn Packet> {
                    box_clone_impl!(self);
                }
            }

            #[derive(Debug, Clone)]
            #[repr(i32)]
            pub enum PlayerDiggingStatus {
                StartedDigging = 0,
                CancelledDigging = 1,
                FinishedDigging = 2,
                DropItemStack = 3,
                DropItem = 4,
                ConsumeItem = 5,
                SwapItemInHand = 6,
            }

            impl Default for PlayerDiggingStatus {
                fn default() -> Self {
                    PlayerDiggingStatus::StartedDigging
                }
            }

            #[derive(Debug, Clone)]
            pub struct EntityAction {
                #[varint]
                pub entity_id: i32,
                pub action_id: EntityActionType,
                #[varint]
                pub jump_boost: i32,
            }

            impl Packet for EntityAction {
                fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
                    self.entity_id = buf.try_get_var_int()?;
                    let action_id = buf.try_get_var_int()?;
                    self.action_id = EntityActionType::from_i32(action_id)
                        .ok_or(Error::InvalidEntityAction(action_id))?;
                    self.jump_boost = buf.try_get_var_int()?;

                    Ok(())
                }

                fn write_to(&self, buf: &mut BytesMut) {
                    buf.push_var_int(self.entity_id);
                    let action_id = self.action_id.to_i32().unwrap();
                    buf.push_var_int(action_id);
                    buf.push_var_int(self.jump_boost);
                }

                fn ty(&self) -> PacketType {
                    PacketType::EntityAction
                }

                fn ty_sized() -> PacketType
                where
                    Self: Sized,
                {
                    PacketType::EntityAction
                }

                fn box_clone(&self) -> Box<dyn Packet> {
                    box_clone_impl!(self);
                }
            }

            #[derive(Debug, Clone)]
            pub enum EntityActionType {
                StartSneaking,
                StopSneaking,
                LeaveBed,
                StartSprinting,
                StopSprinting,
                StartJumpWithHorse,
                StopJumpWithHorse,
                OpenHorseInventory,
                StartFlyingWithElytra,
            }

            impl Default for EntityActionType {
                fn default() -> Self {
                    EntityActionType::StartSneaking
                }
            }

            #[derive(Debug, Clone)]
            pub struct SteerVehicle {
                pub sideways: f32,
                pub forward: f32,
                pub flags: u8,
            }

            #[derive(Debug, Clone)]
            pub struct RecipeBookData {
                #[varint]
                pub ty: i32,
                // TODO
            }

            #[derive(Debug, Clone)]
            pub struct NameItem {
                pub item_name: String,
            }

            #[derive(Debug, Clone)]
            pub struct ResourcePackStatus {
                #[varint]
                pub result: i32,
            }

            #[derive(Debug, Clone)]
            pub struct AdvancementTab {
                #[varint]
                pub action: i32,
                pub tab_id: String,
            }

            #[derive(Debug, Clone)]
            pub struct SelectTrade {
                #[varint]
                pub selected_slot: i32,
            }

            #[derive(Debug, Clone)]
            pub struct SetBeaconEffect {
                #[varint]
                pub primary_effect: i32,
                #[varint]
                pub secondary_effect: i32,
            }

            #[derive(Debug, Clone)]
            pub struct HeldItemChangeServerbound {
                pub slot: i16,
            }

            #[derive(Debug, Clone)]
            pub struct UpdateCommandBlock {
                pub location: BlockPosition,
                pub command: String,
                #[varint]
                pub mode: i32,
                pub flags: u8,
            }

            #[derive(Debug, Clone)]
            pub struct UpdateCommandBlockMinecart {
                #[varint]
                pub entity_id: i32,
                pub command: String,
                pub track_output: bool,
            }

            #[derive(Debug, Clone)]
            pub struct CreativeInventoryAction {
                pub slot: i16,
                pub clicked_item: Slot,
            }

            #[allow(clippy::too_many_arguments)]
            #[derive(Debug, Clone)]
            pub struct UpdateStructureBlock {
                pub location: BlockPosition,
                #[varint]
                pub action: i32,
                #[varint]
                pub mode: i32,
                pub name: String,
                pub offset_x: i8,
                pub offset_y: i8,
                pub offset_z: i8,
                pub size_x: i8,
                pub size_y: i8,
                pub size_z: i8,
                #[varint]
                pub mirror: i32,
                #[varint]
                pub rotation: i32,
                pub metadata: String,
                pub integrity: f32,
                // TODO seed: VarLong,
                pub flags: u8,
            }

            #[derive(Debug, Clone)]
            pub struct UpdateSign {
                pub location: BlockPosition,
                pub line_1: String,
                pub line_2: String,
                pub line_3: String,
                pub line_4: String,
            }

            #[derive(Debug, Clone)]
            pub struct AnimationServerbound {
                pub hand: Hand,
            }

            impl Packet for AnimationServerbound {
                fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
                    let hand_id = buf.try_get_var_int()?;
                    self.hand = match Hand::from_i32(hand_id) {
                        Some(hand) => hand,
                        None => return Err(Error::InvalidHand(hand_id).into()),
                    };

                    Ok(())
                }

                fn write_to(&self, buf: &mut BytesMut) {
                    buf.push_var_int(self.hand.to_i32().unwrap());
                }

                fn ty(&self) -> PacketType {
                    PacketType::AnimationServerbound
                }

                fn ty_sized() -> PacketType
                where
                    Self: Sized,
                {
                    PacketType::AnimationServerbound
                }

                fn box_clone(&self) -> Box<dyn Packet> {
                    box_clone_impl!(self);
                }
            }

            #[derive(Debug, Clone)]
            pub struct Spectate {
                pub target_player: Uuid,
            }

            #[derive(Debug, Clone)]
            pub enum Face {
                Bottom,
                Top,
                North,
                South,
                West,
                East,
            }

            impl Face {
                pub fn placement_offset(self) -> BlockPosition {
                    match self {
                        Face::Bottom => BlockPosition::new(0, -1, 0),
                        Face::Top => BlockPosition::new(0, 1, 0),
                        Face::North => BlockPosition::new(0, 0, -1),
                        Face::South => BlockPosition::new(0, 0, 1),
                        Face::West => BlockPosition::new(-1, 0, 0),
                        Face::East => BlockPosition::new(1, 0, 0),
                    }
                }
            }

            impl Face {
                pub fn face(self) -> BlockFace {
                    match self {
                        Face::Bottom => BlockFace::Ceiling,
                        Face::Top => BlockFace::Floor,
                        Face::North => BlockFace::Wall,
                        Face::South => BlockFace::Wall,
                        Face::West => BlockFace::Wall,
                        Face::East => BlockFace::Wall,
                    }
                }

                pub fn facing_cardinal(self) -> FacingCardinal {
                    match self {
                        Face::North => FacingCardinal::North,
                        Face::South => FacingCardinal::South,
                        Face::West => FacingCardinal::West,
                        Face::East => FacingCardinal::East,
                        Face::Top => panic!("Face::Top cannot be converted to FacingCardinal"),
                        Face::Bottom => {
                            panic!("Face::Bottom cannot be converted to FacingCardinal")
                        }
                    }
                }

                pub fn facing_cardinal_and_down(self) -> FacingCardinalAndDown {
                    match self {
                        Face::North => FacingCardinalAndDown::North,
                        Face::South => FacingCardinalAndDown::South,
                        Face::West => FacingCardinalAndDown::West,
                        Face::East => FacingCardinalAndDown::East,
                        Face::Bottom => FacingCardinalAndDown::Down,
                        Face::Top => {
                            panic!("Face::Top cannot be converted to FacingCardinalAndDown")
                        }
                    }
                }

                pub fn facing_cubic(self) -> FacingCubic {
                    match self {
                        Face::North => FacingCubic::North,
                        Face::South => FacingCubic::South,
                        Face::West => FacingCubic::West,
                        Face::East => FacingCubic::East,
                        Face::Top => FacingCubic::Up,
                        Face::Bottom => FacingCubic::Down,
                    }
                }
            }

            impl Default for Face {
                fn default() -> Self {
                    Face::Bottom
                }
            }

            #[derive(Debug, Clone)]
            pub struct PlayerBlockPlacement {
                pub location: BlockPosition,
                pub face: Face,
                #[varint]
                pub hand: i32,
                pub cursor_position_x: f32,
                pub cursor_position_y: f32,
                pub cursor_position_z: f32,
            }

            impl Packet for PlayerBlockPlacement {
                fn read_from(&mut self, buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
                    self.location = buf.try_get_position()?;
                    let face_id = buf.try_get_var_int()?;
                    self.face = Face::from_i32(face_id).ok_or(Error::InvalidFace(face_id))?;
                    self.hand = buf.try_get_var_int()?;
                    self.cursor_position_x = buf.try_get_f32()?;
                    self.cursor_position_y = buf.try_get_f32()?;
                    self.cursor_position_z = buf.try_get_f32()?;
                    Ok(())
                }

                fn write_to(&self, buf: &mut BytesMut) {
                    buf.push_position(&self.location);
                    buf.push_var_int(self.face.to_i32().unwrap());
                    buf.push_var_int(self.hand);
                    buf.push_f32(self.cursor_position_x);
                    buf.push_f32(self.cursor_position_y);
                    buf.push_f32(self.cursor_position_z);
                }

                fn ty(&self) -> PacketType {
                    PacketType::PlayerBlockPlacement
                }

                fn ty_sized() -> PacketType
                where
                    Self: Sized,
                {
                    PacketType::PlayerBlockPlacement
                }

                fn box_clone(&self) -> Box<dyn Packet> {
                    box_clone_impl!(self);
                }
            }

            #[derive(Debug, Clone)]
            pub struct UseItem {
                #[varint]
                pub hand: i32,
            }
        }
    }
}
