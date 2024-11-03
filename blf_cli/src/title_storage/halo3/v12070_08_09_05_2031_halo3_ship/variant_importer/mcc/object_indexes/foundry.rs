use bimap::BiMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref foundry_objects_map: BiMap<u32, u32> = BiMap::from_iter(vec![
        (0xE6E80572, 0x00010000),
        (0xE95007DA, 0x00010001),
        (0xE62E04B8, 0x00010002),
        (0xE28B0115, 0x00010003),
        (0xF2E71171, 0x00010004),
        (0xF36D11F7, 0x00010005),
        (0xEEDE0D68, 0x00020000),
        (0xEF280DB2, 0x00020001),
        (0xEFB20E3C, 0x00020002),
        (0xF0420ECC, 0x00020003),
        (0xEFFF0E89, 0x00020004),
        (0xF09C0F26, 0x00020005),
        (0xEC4C0AD6, 0x00020006),
        (0xED890C13, 0x00020007),
        (0xEF610DEB, 0x00020008),
        (0xED070B91, 0x00020009),
        (0xF1BC1046, 0x0002000A),
        (0xF0D40F5E, 0x0002000B),
        (0xF13E0FC8, 0x0002000C),
        (0xEE930D1D, 0x0002000D),
        (0xF484130E, 0x0002000E),
        (0xF1F4107E, 0x0002000F),
        (0xF4C1134B, 0x00020010),
        (0xEE560CE0, 0x00020011),
        (0xF214109E, 0x00020012),
        (0xF5081392, 0x00020013),
        (0xF54B13D5, 0x00020014),
        (0xE95107DB, 0x00020015),
        (0xF25E10E8, 0x00020016),
        (0xE9FB0885, 0x00030000),
        (0xE98E0818, 0x00030001),
        (0xEA2308AD, 0x00030002),
        (0xEC3D0AC7, 0x00030003),
        (0xEAC90953, 0x00030004),
        (0xEB5709E1, 0x00030005),
        (0xEB2809B2, 0x00030006),
        (0xEBD00A5A, 0x00030007),
        (0xF57A1404, 0x00030008),
        (0xF5A1142B, 0x00030009),
        (0xEC1D0AA7, 0x0003000A),
        (0xF5B4143E, 0x0003000B),
        (0xEC070A91, 0x0003000C),
        (0xEBF60A80, 0x0003000D),
        (0xEC160AA0, 0x0003000E),
        (0xF5B61440, 0x00040000),
        (0xF5C71451, 0x00040001),
        (0xF5D0145A, 0x00040002),
        (0xF5D5145F, 0x00040003),
        (0xF5DA1464, 0x00040004),
        (0xF5EB1475, 0x00040005),
        (0xF5F0147A, 0x00040006),
        (0xF604148E, 0x00040007),
        (0xF61E14A8, 0x00040008),
        (0xF62314AD, 0x00040009),
        (0xF62814B2, 0x0004000A),
        (0xF63614C0, 0x0004000B),
        (0xF64A14D4, 0x0004000C),
        (0xF64F14D9, 0x0004000D),
        (0xF6881512, 0x0004000E),
        (0xF6D1155B, 0x0004000F),
        (0xF6E2156C, 0x00040010),
        (0xF6EA1574, 0x00040011),
        (0xF6F3157D, 0x00040012),
        (0xF6FC1586, 0x00040013),
        (0xF703158D, 0x00040014),
        (0xF72A15B4, 0x00040015),
        (0xF73A15C4, 0x00040016),
        (0xF74415CE, 0x00040017),
        (0xF76C15F6, 0x00040018),
        (0xF79B1625, 0x00040019),
        (0xF7CF1659, 0x0004001A),
        (0xF7EA1674, 0x0004001B),
        (0xF7FA1684, 0x0004001C),
        (0xF811169B, 0x0004001D),
        (0xF82416AE, 0x0004001E),
        (0xF82916B3, 0x0004001F),
        (0xF84216CC, 0x00040020),
        (0xF84E16D8, 0x00040021),
        (0xF85316DD, 0x00040022),
        (0xF86016EA, 0x00050000),
        (0xF87416FE, 0x00050001),
        (0xF87B1705, 0x00050002),
        (0xF87C1706, 0x00060000),
        (0xF8AF1739, 0x00060001),
        (0xF8B81742, 0x00060002),
        (0xF8DF1769, 0x00060003),
        (0xF8E0176A, 0x00060004),
        (0xF8E91773, 0x00060005),
        (0xF8EA1774, 0x00060006),
        (0xF8FA1784, 0x00060007),
        (0xF8FC1786, 0x00060008),
        (0xE200008A, 0x00070000),
        (0xE21F00A9, 0x00070001),
        (0xE23500BF, 0x00070002),
        (0xE23900C3, 0x00070003),
        (0xE23B00C5, 0x00070004),
        (0xE23D00C7, 0x00070005),
        (0xE23F00C9, 0x00070006),
        (0xE24100CB, 0x00070007),
        (0xE22000AA, 0x00070008),
        (0xE23600C0, 0x00070009),
        (0xE23300BD, 0x0007000A),
        (0xE23400BE, 0x0007000B),
        (0xE23A00C4, 0x0007000C),
        (0xE23C00C6, 0x0007000D),
        (0xE23E00C8, 0x0007000E),
        (0xE24000CA, 0x0007000F),
        (0xE24200CC, 0x00070010),
        (0xE23700C1, 0x00070011),
        (0xE23800C2, 0x00070012),
        (0xE21E00A8, 0x00080001),
        (0xE24300CD, 0x00080014),
        (0xE26400EE, 0x00080015),
        (0xE26E00F8, 0x00080016),
        (0xE27100FB, 0x00080017),
        (0xEB830A0D, 0x000B0006),
        (0xF9B4183E, 0x000C0000),
        (0xF9C5184F, 0x000C0023),
        (0xF9CA1854, 0x000C0024),
        (0xF9CE1858, 0x000C0025),
        (0xF9D2185C, 0x000C0026),
        (0xF9D61860, 0x000C0027),
        (0xF9DA1864, 0x000C0028),
        (0xF9DE1868, 0x000C0029),
        (0xF9E2186C, 0x000C002A),
        (0xF9E61870, 0x000C002B),
        (0xF9EA1874, 0x000C002C),
    ]);
}


