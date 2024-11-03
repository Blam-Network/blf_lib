use bimap::BiMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref heretic_objects_map: BiMap<u32, u32> = BiMap::from_iter(vec![
        (0xEF230DAD, 0x00010000),
        (0xF0080E92, 0x00010001),
        (0xEC2A0AB4, 0x00020000),
        (0xE93F07C9, 0x00020001),
        (0xEE000C8A, 0x00020002),
        (0xE98F0819, 0x00020003),
        (0xE8D5075F, 0x00020004),
        (0xE81806A2, 0x00020005),
        (0xF0AF0F39, 0x00020006),
        (0xF0FE0F88, 0x00020007),
        (0xEC5D0AE7, 0x00020008),
        (0xF1490FD3, 0x00020009),
        (0xE2DA0164, 0x0002000A),
        (0xEC800B0A, 0x0002000B),
        (0xEB2809B2, 0x0002000C),
        (0xF19E1028, 0x0002000D),
        (0xE9FE0888, 0x0002000E),
        (0xEB970A21, 0x0002000F),
        (0xECF10B7B, 0x00020010),
        (0xE87006FA, 0x00020011),
        (0xECB20B3C, 0x00020012),
        (0xEE920D1C, 0x00020013),
        (0xED6D0BF7, 0x00020014),
        (0xEBE90A73, 0x00020015),
        (0xF1E0106A, 0x00020016),
        (0xE64B04D5, 0x00030000),
        (0xE583040D, 0x00030001),
        (0xE71905A3, 0x00030002),
        (0xF210109A, 0x00030003),
        (0xE681050B, 0x00030004),
        (0xE7CB0655, 0x00030005),
        (0xF21F10A9, 0x00030006),
        (0xF24710D1, 0x00030007),
        (0xF26B10F5, 0x00030008),
        (0xF292111C, 0x00030009),
        (0xF2A5112F, 0x0003000A),
        (0xF2C4114E, 0x0003000B),
        (0xE70A0594, 0x0003000C),
        (0xE6F90583, 0x0003000D),
        (0xF3061190, 0x0003000E),
        (0xF30D1197, 0x00040000),
        (0xF33711C1, 0x00040001),
        (0xF35711E1, 0x00040002),
        (0xF384120E, 0x00040003),
        (0xF3A4122E, 0x00040004),
        (0xF3B61240, 0x00040005),
        (0xF3BB1245, 0x00040006),
        (0xF3C81252, 0x00040007),
        (0xF3CD1257, 0x00040008),
        (0xF3DD1267, 0x00040009),
        (0xF3E2126C, 0x0004000A),
        (0xF3F4127E, 0x0004000B),
        (0xF40C1296, 0x0004000C),
        (0xF42012AA, 0x0004000D),
        (0xF42312AD, 0x0004000E),
        (0xF42712B1, 0x0004000F),
        (0xF42A12B4, 0x00040010),
        (0xF42D12B7, 0x00040011),
        (0xF43012BA, 0x00050000),
        (0xF44412CE, 0x00050001),
        (0xF44B12D5, 0x00050002),
        (0xF44C12D6, 0x00060000),
        (0xF45612E0, 0x00060001),
        (0xF47D1307, 0x00060002),
        (0xF47E1308, 0x00060003),
        (0xF4A71331, 0x00060004),
        (0xF4B0133A, 0x00060005),
        (0xF4B1133B, 0x00060006),
        (0xF4B3133D, 0x00060007),
        (0xF4B5133F, 0x00060008),
        (0xE24200CC, 0x00070000),
        (0xE24500CF, 0x00070001),
        (0xE24300CD, 0x00070002),
        (0xE24800D2, 0x00070003),
        (0xE24700D1, 0x00070004),
        (0xE22000AA, 0x00070005),
        (0xE24B00D5, 0x00070006),
        (0xE24D00D7, 0x00070007),
        (0xE24600D0, 0x00070008),
        (0xE24400CE, 0x00070009),
        (0xF4B61340, 0x0007000A),
        (0xF4B71341, 0x0007000B),
        (0xE24900D3, 0x0007000C),
        (0xE24A00D4, 0x0007000D),
        (0xE22F00B9, 0x0007000E),
        (0xE24C00D6, 0x0007000F),
        (0xF4B81342, 0x00070010),
        (0xE24E00D8, 0x00070011),
        (0xF4B91343, 0x00070012),
        (0xE24F00D9, 0x0008000F),
        (0xE2770101, 0x00080010),
        (0xE293011D, 0x00080011),
        (0xE2A60130, 0x00080012),
        (0xE2A70131, 0x00080013),
        (0xE2C3014D, 0x00080014),
        (0xE2C60150, 0x00080015),
        (0xF4BE1348, 0x00080016),
    ]);
}


