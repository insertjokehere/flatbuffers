// automatically generated by the FlatBuffers compiler, do not modify


#![allow(unused_imports, dead_code)]

use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod my_game {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
#[allow(unused_imports, dead_code)]
pub mod sample {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_COLOR: i8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_COLOR: i8 = 2;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_COLOR: [Color; 3] = [
  Color::Red,
  Color::Green,
  Color::Blue,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(pub i8);
#[allow(non_upper_case_globals)]
impl Color {
  pub const Red: Self = Self(0);
  pub const Green: Self = Self(1);
  pub const Blue: Self = Self(2);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::Red,
    Self::Green,
    Self::Blue,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::Red => Some("Red"),
      Self::Green => Some("Green"),
      Self::Blue => Some("Blue"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Color {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Color {
    type Output = Color;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Color {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = i8::to_le(self.0);
    Self(b)
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let b = i8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for Color {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Color {}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_EQUIPMENT: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_EQUIPMENT: u8 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_EQUIPMENT: [Equipment; 2] = [
  Equipment::NONE,
  Equipment::Weapon,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Equipment(pub u8);
#[allow(non_upper_case_globals)]
impl Equipment {
  pub const NONE: Self = Self(0);
  pub const Weapon: Self = Self(1);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 1;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::Weapon,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::Weapon => Some("Weapon"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for Equipment {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
pub struct EquipmentUnionTableOffset {}
impl<'a> flatbuffers::Follow<'a> for Equipment {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Equipment {
    type Output = Equipment;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Equipment {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for Equipment {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Equipment {}
// struct Vec3, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3(pub [u8; 12]);
impl std::fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Vec3")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Vec3 {}
impl flatbuffers::SafeSliceAccess for Vec3 {}
impl<'a> flatbuffers::Follow<'a> for Vec3 {
  type Inner = &'a Vec3;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Vec3>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vec3 {
  type Inner = &'a Vec3;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Vec3>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Vec3 {
    type Output = Vec3;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Vec3 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Vec3 {
    type Output = Vec3;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Vec3 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Vec3 {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl Vec3 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    x: f32,
    y: f32,
    z: f32,
  ) -> Self {
    let mut s = Self([0; 12]);
    s.set_x(x);
    s.set_y(y);
    s.set_z(z);
    s
  }

  pub fn x(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<f32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_x(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f32 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<f32>(),
      );
    }
  }

  pub fn y(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<f32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_y(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f32 as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<f32>(),
      );
    }
  }

  pub fn z(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<f32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_z(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f32 as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<f32>(),
      );
    }
  }

}

pub enum MonsterOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Monster<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Monster<'a> {
    type Inner = Monster<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Monster<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Monster {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MonsterArgs<'args>) -> flatbuffers::WIPOffset<Monster<'bldr>> {
      let mut builder = MonsterBuilder::new(_fbb);
      if let Some(x) = args.path { builder.add_path(x); }
      if let Some(x) = args.equipped { builder.add_equipped(x); }
      if let Some(x) = args.weapons { builder.add_weapons(x); }
      if let Some(x) = args.inventory { builder.add_inventory(x); }
      if let Some(x) = args.name { builder.add_name(x); }
      if let Some(x) = args.pos { builder.add_pos(x); }
      builder.add_hp(args.hp);
      builder.add_mana(args.mana);
      builder.add_equipped_type(args.equipped_type);
      builder.add_color(args.color);
      builder.finish()
    }

    pub const VT_POS: flatbuffers::VOffsetT = 4;
    pub const VT_MANA: flatbuffers::VOffsetT = 6;
    pub const VT_HP: flatbuffers::VOffsetT = 8;
    pub const VT_NAME: flatbuffers::VOffsetT = 10;
    pub const VT_INVENTORY: flatbuffers::VOffsetT = 14;
    pub const VT_COLOR: flatbuffers::VOffsetT = 16;
    pub const VT_WEAPONS: flatbuffers::VOffsetT = 18;
    pub const VT_EQUIPPED_TYPE: flatbuffers::VOffsetT = 20;
    pub const VT_EQUIPPED: flatbuffers::VOffsetT = 22;
    pub const VT_PATH: flatbuffers::VOffsetT = 24;

  #[inline]
  pub fn pos(&self) -> Option<&'a Vec3> {
    self._tab.get::<Vec3>(Monster::VT_POS, None)
  }
  #[inline]
  pub fn mana(&self) -> i16 {
    self._tab.get::<i16>(Monster::VT_MANA, Some(150)).unwrap()
  }
  #[inline]
  pub fn hp(&self) -> i16 {
    self._tab.get::<i16>(Monster::VT_HP, Some(100)).unwrap()
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Monster::VT_NAME, None)
  }
  #[inline]
  pub fn inventory(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Monster::VT_INVENTORY, None).map(|v| v.safe_slice())
  }
  #[inline]
  pub fn color(&self) -> Color {
    self._tab.get::<Color>(Monster::VT_COLOR, Some(Color::Blue)).unwrap()
  }
  #[inline]
  pub fn weapons(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Weapon<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Weapon>>>>(Monster::VT_WEAPONS, None)
  }
  #[inline]
  pub fn equipped_type(&self) -> Equipment {
    self._tab.get::<Equipment>(Monster::VT_EQUIPPED_TYPE, Some(Equipment::NONE)).unwrap()
  }
  #[inline]
  pub fn equipped(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Monster::VT_EQUIPPED, None)
  }
  #[inline]
  pub fn path(&self) -> Option<&'a [Vec3]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Vec3>>>(Monster::VT_PATH, None).map(|v| v.safe_slice())
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn equipped_as_weapon(&self) -> Option<Weapon<'a>> {
    if self.equipped_type() == Equipment::Weapon {
      self.equipped().map(Weapon::init_from_table)
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Monster<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Vec3>(&"pos", Self::VT_POS, false)?
     .visit_field::<i16>(&"mana", Self::VT_MANA, false)?
     .visit_field::<i16>(&"hp", Self::VT_HP, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"name", Self::VT_NAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>(&"inventory", Self::VT_INVENTORY, false)?
     .visit_field::<Color>(&"color", Self::VT_COLOR, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Weapon>>>>(&"weapons", Self::VT_WEAPONS, false)?
     .visit_union::<Equipment, _>(&"equipped_type", Self::VT_EQUIPPED_TYPE, &"equipped", Self::VT_EQUIPPED, false, |key, v, pos| {
        match key {
          Equipment::Weapon => v.verify_union_variant::<flatbuffers::ForwardsUOffset<Weapon>>("Equipment::Weapon", pos),
          _ => Ok(()),
        }
     })?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, Vec3>>>(&"path", Self::VT_PATH, false)?
     .finish();
    Ok(())
  }
}
pub struct MonsterArgs<'a> {
    pub pos: Option<&'a Vec3>,
    pub mana: i16,
    pub hp: i16,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub inventory: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
    pub color: Color,
    pub weapons: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Weapon<'a>>>>>,
    pub equipped_type: Equipment,
    pub equipped: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
    pub path: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Vec3>>>,
}
impl<'a> Default for MonsterArgs<'a> {
    #[inline]
    fn default() -> Self {
        MonsterArgs {
            pos: None,
            mana: 150,
            hp: 100,
            name: None,
            inventory: None,
            color: Color::Blue,
            weapons: None,
            equipped_type: Equipment::NONE,
            equipped: None,
            path: None,
        }
    }
}
pub struct MonsterBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MonsterBuilder<'a, 'b> {
  #[inline]
  pub fn add_pos(&mut self, pos: &Vec3) {
    self.fbb_.push_slot_always::<&Vec3>(Monster::VT_POS, pos);
  }
  #[inline]
  pub fn add_mana(&mut self, mana: i16) {
    self.fbb_.push_slot::<i16>(Monster::VT_MANA, mana, 150);
  }
  #[inline]
  pub fn add_hp(&mut self, hp: i16) {
    self.fbb_.push_slot::<i16>(Monster::VT_HP, hp, 100);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_NAME, name);
  }
  #[inline]
  pub fn add_inventory(&mut self, inventory: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_INVENTORY, inventory);
  }
  #[inline]
  pub fn add_color(&mut self, color: Color) {
    self.fbb_.push_slot::<Color>(Monster::VT_COLOR, color, Color::Blue);
  }
  #[inline]
  pub fn add_weapons(&mut self, weapons: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Weapon<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_WEAPONS, weapons);
  }
  #[inline]
  pub fn add_equipped_type(&mut self, equipped_type: Equipment) {
    self.fbb_.push_slot::<Equipment>(Monster::VT_EQUIPPED_TYPE, equipped_type, Equipment::NONE);
  }
  #[inline]
  pub fn add_equipped(&mut self, equipped: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_EQUIPPED, equipped);
  }
  #[inline]
  pub fn add_path(&mut self, path: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Vec3>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_PATH, path);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MonsterBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MonsterBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Monster<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Monster<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Monster");
      ds.field("pos", &self.pos());
      ds.field("mana", &self.mana());
      ds.field("hp", &self.hp());
      ds.field("name", &self.name());
      ds.field("inventory", &self.inventory());
      ds.field("color", &self.color());
      ds.field("weapons", &self.weapons());
      ds.field("equipped_type", &self.equipped_type());
      match self.equipped_type() {
        Equipment::Weapon => {
          if let Some(x) = self.equipped_as_weapon() {
            ds.field("equipped", &x)
          } else {
            ds.field("equipped", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => { 
          let x: Option<()> = None;
          ds.field("equipped", &x)
        },
      };
      ds.field("path", &self.path());
      ds.finish()
  }
}
pub enum WeaponOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Weapon<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Weapon<'a> {
    type Inner = Weapon<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Weapon<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Weapon {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args WeaponArgs<'args>) -> flatbuffers::WIPOffset<Weapon<'bldr>> {
      let mut builder = WeaponBuilder::new(_fbb);
      if let Some(x) = args.name { builder.add_name(x); }
      builder.add_damage(args.damage);
      builder.finish()
    }

    pub const VT_NAME: flatbuffers::VOffsetT = 4;
    pub const VT_DAMAGE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Weapon::VT_NAME, None)
  }
  #[inline]
  pub fn damage(&self) -> i16 {
    self._tab.get::<i16>(Weapon::VT_DAMAGE, Some(0)).unwrap()
  }
}

impl flatbuffers::Verifiable for Weapon<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"name", Self::VT_NAME, false)?
     .visit_field::<i16>(&"damage", Self::VT_DAMAGE, false)?
     .finish();
    Ok(())
  }
}
pub struct WeaponArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub damage: i16,
}
impl<'a> Default for WeaponArgs<'a> {
    #[inline]
    fn default() -> Self {
        WeaponArgs {
            name: None,
            damage: 0,
        }
    }
}
pub struct WeaponBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> WeaponBuilder<'a, 'b> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Weapon::VT_NAME, name);
  }
  #[inline]
  pub fn add_damage(&mut self, damage: i16) {
    self.fbb_.push_slot::<i16>(Weapon::VT_DAMAGE, damage, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WeaponBuilder<'a, 'b> {
    let start = _fbb.start_table();
    WeaponBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Weapon<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Weapon<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Weapon");
      ds.field("name", &self.name());
      ds.field("damage", &self.damage());
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_monster<'a>(buf: &'a [u8]) -> Monster<'a> {
  unsafe { flatbuffers::root_unchecked::<Monster<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_monster<'a>(buf: &'a [u8]) -> Monster<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<Monster<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Monster`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_monster_unchecked`.
pub fn root_as_monster(buf: &[u8]) -> Result<Monster, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Monster>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Monster` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_monster_unchecked`.
pub fn size_prefixed_root_as_monster(buf: &[u8]) -> Result<Monster, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Monster>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Monster` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_monster_unchecked`.
pub fn root_as_monster_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Monster<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Monster<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Monster` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_monster_unchecked`.
pub fn size_prefixed_root_as_monster_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Monster<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Monster<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Monster and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Monster`.
pub unsafe fn root_as_monster_unchecked(buf: &[u8]) -> Monster {
  flatbuffers::root_unchecked::<Monster>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Monster and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Monster`.
pub unsafe fn size_prefixed_root_as_monster_unchecked(buf: &[u8]) -> Monster {
  flatbuffers::size_prefixed_root_unchecked::<Monster>(buf)
}
#[inline]
pub fn finish_monster_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Monster<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_monster_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Monster<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Sample
}  // pub mod MyGame

