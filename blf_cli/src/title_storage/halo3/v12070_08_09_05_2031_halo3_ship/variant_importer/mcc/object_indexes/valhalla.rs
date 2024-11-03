use bimap::BiMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref valhalla_objects_map: BiMap<u32, u32> = BiMap::from_iter(vec![
        (0xEA3B08C5, 0x00010000),
        (0xED1F0BA9, 0x00010001),
        (0xF0B20F3C, 0x00010002),
        (0xF1260FB0, 0x00010003),
        (0xED970C21, 0x00010004),
        (0xEC120A9C, 0x00010005),
        (0xEFDC0E66, 0x00010006),
        (0xEED60D60, 0x00010007),
        (0xE26700F1, 0x00020000),
        (0xE53603C0, 0x00020001),
        (0xE7E60670, 0x00020002),
        (0xE8D3075D, 0x00020003),
        (0xE88E0718, 0x00020004),
        (0xF32211AC, 0x00020005),
        (0xE72705B1, 0x00020006),
        (0xF3D3125D, 0x00020007),
        (0xF35A11E4, 0x00020008),
        (0xF584140E, 0x00020009),
        (0xF4C61350, 0x0002000A),
        (0xE76305ED, 0x0002000B),
        (0xE57F0409, 0x0002000C),
        (0xF51913A3, 0x0002000D),
        (0xEEC20D4C, 0x0002000E),
        (0xF4F71381, 0x0002000F),
        (0xF49A1324, 0x00020010),
        (0xF5CA1454, 0x00020011),
        (0xF1AF1039, 0x00030000),
        (0xF1DD1067, 0x00030001),
        (0xF26910F3, 0x00030002),
        (0xF5F91483, 0x00030003),
        (0xF202108C, 0x00030004),
        (0xF2A81132, 0x00030005),
        (0xF24010CA, 0x00030006),
        (0xF6081492, 0x00030007),
        (0xF2FA1184, 0x00030008),
        (0xF63214BC, 0x00030009),
        (0xF64514CF, 0x0003000A),
        (0xF66114EB, 0x0003000B),
        (0xF2E5116F, 0x0003000C),
        (0xF2D4115E, 0x0003000D),
        (0xF2F3117D, 0x0003000E),
        (0xF6A3152D, 0x00040000),
        (0xF6CC1556, 0x00040001),
        (0xF6E3156D, 0x00040002),
        (0xF6EF1579, 0x00040003),
        (0xF6F4157E, 0x00040004),
        (0xF6FF1589, 0x00040005),
        (0xF712159C, 0x00040006),
        (0xF71E15A8, 0x00040007),
        (0xF72E15B8, 0x00040008),
        (0xF74315CD, 0x00040009),
        (0xF74915D3, 0x0004000A),
        (0xF74E15D8, 0x0004000B),
        (0xF75A15E4, 0x0004000C),
        (0xF76E15F8, 0x0004000D),
        (0xF77315FD, 0x0004000E),
        (0xF7891613, 0x00050000),
        (0xF79D1627, 0x00050001),
        (0xF7A4162E, 0x00050002),
        (0xF7A5162F, 0x00060000),
        (0xF7D81662, 0x00060001),
        (0xF7E1166B, 0x00060002),
        (0xF8081692, 0x00060003),
        (0xF8091693, 0x00060004),
        (0xF812169C, 0x00060005),
        (0xF813169D, 0x00060006),
        (0xF82316AD, 0x00060007),
        (0xF82516AF, 0x00060008),
        (0xE200008A, 0x00070000),
        (0xEA02088C, 0x00070001),
        (0xE23600C0, 0x00070002),
        (0xE23700C1, 0x00070003),
        (0xE21F00A9, 0x00070004),
        (0xE21E00A8, 0x00070005),
        (0xE9DA0864, 0x00070006),
        (0xEA10089A, 0x00070007),
        (0xEA03088D, 0x00070008),
        (0xE23500BF, 0x00070009),
        (0xE23400BE, 0x0007000A),
        (0xE23300BD, 0x0007000B),
        (0xF82616B0, 0x0007000C),
        (0xF82716B1, 0x0007000D),
        (0xE22000AA, 0x0007000E),
        (0xE23800C2, 0x0007000F),
        (0xEA11089B, 0x00070010),
        (0xF82816B2, 0x00070011),
        (0xF82916B3, 0x00070012),
        (0xE23900C3, 0x0008000A),
        (0xE9DB0865, 0x0008000C),
        (0xE9F80882, 0x0008000D),
        (0xEA04088E, 0x00080010),
        (0xEA12089C, 0x00080013),
        (0xEA2208AC, 0x00080014),
        (0xEA2708B1, 0x00080015),
        (0xEA2908B3, 0x00080016),
        (0xEA3608C0, 0x00080017),
        (0xEA3808C2, 0x00080018),
        (0xEA3A08C4, 0x00080019),
        (0xF84C16D6, 0x000C0000),
        (0xF85916E3, 0x000C0001),
        (0xF85E16E8, 0x000C0004),
        (0xF86D16F7, 0x000C0005),
        (0xF86F16F9, 0x000C0009),
        (0xF87F1709, 0x000C000B),
        (0xF6DD1567, 0x000C000D),
        (0xF885170F, 0x000C0011),
        (0xF88B1715, 0x000C0018),
        (0xF8A4172E, 0x000C0019),
        (0xF8A91733, 0x000C001C),
        (0xF8AE1738, 0x000C001D),
        (0xF8B3173D, 0x000C0021),
        (0xF8B81742, 0x000C0022),
        (0xF8BC1746, 0x000C0023),
        (0xF8C0174A, 0x000C0024),
        (0xF8C4174E, 0x000C0025),
        (0xF8C81752, 0x000C0026),
        (0xF8CC1756, 0x000C0027),
        (0xF8D0175A, 0x000C0028),
        (0xF8D4175E, 0x000C0029),
        (0xF8D81762, 0x000C002A),
    ]);
}

