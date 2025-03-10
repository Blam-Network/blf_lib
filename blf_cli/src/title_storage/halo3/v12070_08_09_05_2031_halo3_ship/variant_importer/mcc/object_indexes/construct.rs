use bimap::BiMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref construct_objects_map: BiMap<u32, u32> = BiMap::from_iter(vec![
        (0xE69F0529, 0x00010000),
        (0xE24C00D6, 0x00010001),
        (0xE75705E1, 0x00010002),
        (0xEC100A9A, 0x00020000),
        (0xEC7B0B05, 0x00020001),
        (0xECB30B3D, 0x00020002),
        (0xED990C23, 0x00020003),
        (0xED550BDF, 0x00020004),
        (0xEDF60C80, 0x00020005),
        (0xF0850F0F, 0x00020006),
        (0xEF9E0E28, 0x00020007),
        (0xF13B0FC5, 0x00020008),
        (0xEB4109CB, 0x00020009),
        (0xF1891013, 0x0002000A),
        (0xEE450CCF, 0x0002000B),
        (0xF0B80F42, 0x0002000C),
        (0xEA9C0926, 0x0002000D),
        (0xEEA80D32, 0x0002000E),
        (0xE90B0795, 0x0002000F),
        (0xF0640EEE, 0x00020010),
        (0xEEF00D7A, 0x00020011),
        (0xF1CB1055, 0x00020012),
        (0xEF650DEF, 0x00020013),
        (0xF21710A1, 0x00020014),
        (0xEF140D9E, 0x00020015),
        (0xE96507EF, 0x00030000),
        (0xE9AA0834, 0x00030001),
        (0xF24710D1, 0x00030002),
        (0xF26F10F9, 0x00030003),
        (0xEA4408CE, 0x00030004),
        (0xE9D2085C, 0x00030005),
        (0xF27E1108, 0x00030006),
        (0xEA10089A, 0x00030007),
        (0xF2A61130, 0x00030008),
        (0xF2CD1157, 0x00030009),
        (0xF2E0116A, 0x0003000A),
        (0xF2FF1189, 0x0003000B),
        (0xEA7508FF, 0x0003000C),
        (0xEA8B0915, 0x0003000D),
        (0xEA95091F, 0x0003000E),
        (0xF34111CB, 0x00040000),
        (0xF36D11F7, 0x00040001),
        (0xF384120E, 0x00040002),
        (0xF390121A, 0x00040003),
        (0xF395121F, 0x00040004),
        (0xF3A0122A, 0x00040005),
        (0xF3B3123D, 0x00040006),
        (0xF3BF1249, 0x00040007),
        (0xF3D71261, 0x00040008),
        (0xF3ED1277, 0x00040009),
        (0xF3FA1284, 0x0004000A),
        (0xF3FF1289, 0x0004000B),
        (0xF40B1295, 0x0004000C),
        (0xF41F12A9, 0x0004000D),
        (0xF43C12C6, 0x0004000E),
        (0xF44812D2, 0x0004000F),
        (0xF44E12D8, 0x00040010),
        (0xF45D12E7, 0x00040011),
        (0xF4891313, 0x00040012),
        (0xF48E1318, 0x00040013),
        (0xF4AE1338, 0x00040014),
        (0xF4C4134E, 0x00050000),
        (0xF4D81362, 0x00050001),
        (0xF4DF1369, 0x00050002),
        (0xF4E0136A, 0x00060000),
        (0xF513139D, 0x00060001),
        (0xF51C13A6, 0x00060002),
        (0xF54313CD, 0x00060003),
        (0xF54413CE, 0x00060004),
        (0xF54D13D7, 0x00060005),
        (0xF54E13D8, 0x00060006),
        (0xF55E13E8, 0x00060007),
        (0xF56013EA, 0x00060008),
        (0xE1ED0077, 0x00070000),
        (0xE22200AC, 0x00070001),
        (0xE22400AE, 0x00070002),
        (0xE22700B1, 0x00070003),
        (0xE20C0096, 0x00070004),
        (0xE20B0095, 0x00070005),
        (0xE22600B0, 0x00070006),
        (0xE22100AB, 0x00070007),
        (0xE22300AD, 0x00070008),
        (0xE20D0097, 0x00070009),
        (0xF56113EB, 0x0007000A),
        (0xF56213EC, 0x0007000B),
        (0xF56313ED, 0x0007000C),
        (0xF56413EE, 0x0007000D),
        (0xF56513EF, 0x0007000E),
        (0xE22500AF, 0x0007000F),
        (0xE22000AA, 0x00070010),
        (0xF56613F0, 0x00070011),
        (0xF56713F1, 0x00070012),
        (0xE22800B2, 0x0008000C),
        (0xE24B00D5, 0x0008000D),
        (0xF5A3142D, 0x000C0000),
        (0xF5AF1439, 0x000C0001),
        (0xF5BE1448, 0x000C000E),
        (0xF5C3144D, 0x000C0010),
        (0xF5C91453, 0x000C0018),
        (0xF5D1145B, 0x000C001A),
        (0xF5D61460, 0x000C001B),
        (0xF5DA1464, 0x000C001C),
        (0xF5DE1468, 0x000C001D),
        (0xF5E2146C, 0x000C001E),
        (0xF5E61470, 0x000C001F),
        (0xF5EA1474, 0x000C0020),
        (0xF5EE1478, 0x000C0021),
        (0xF5F2147C, 0x000C0022),
        (0xF5F61480, 0x000C0023),
    ]);
}


