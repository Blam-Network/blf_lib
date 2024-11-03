use bimap::BiMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref the_pit_objects_map: BiMap<u32, u32> = BiMap::from_iter(vec![
        (0xE9DC0866, 0x00010000),
        (0xE914079E, 0x00010001),
        (0xEF650DEF, 0x00020000),
        (0xEFAF0E39, 0x00020001),
        (0xF0360EC0, 0x00020002),
        (0xF0C60F50, 0x00020003),
        (0xF0830F0D, 0x00020004),
        (0xF1200FAA, 0x00020005),
        (0xECD30B5D, 0x00020006),
        (0xEE110C9B, 0x00020007),
        (0xEFE80E72, 0x00020008),
        (0xED8E0C18, 0x00020009),
        (0xF24210CC, 0x0002000A),
        (0xF15D0FE7, 0x0002000B),
        (0xF1C4104E, 0x0002000C),
        (0xEF1D0DA7, 0x0002000D),
        (0xEA4308CD, 0x0002000E),
        (0xF27B1105, 0x0002000F),
        (0xEEE00D6A, 0x00020010),
        (0xF2E5116F, 0x00020011),
        (0xF29B1125, 0x00020012),
        (0xF3891213, 0x00020013),
        (0xF3CE1258, 0x00020014),
        (0xEAE90973, 0x00030000),
        (0xEA7C0906, 0x00030001),
        (0xEB7009FA, 0x00030002),
        (0xF3FD1287, 0x00030003),
        (0xEB11099B, 0x00030004),
        (0xEC160AA0, 0x00030005),
        (0xEC6B0AF5, 0x00030006),
        (0xF40C1296, 0x00030007),
        (0xEC980B22, 0x00030008),
        (0xECC00B4A, 0x00030009),
        (0xF43012BA, 0x0003000A),
        (0xF44F12D9, 0x0003000B),
        (0xEC550ADF, 0x0003000C),
        (0xEC440ACE, 0x0003000D),
        (0xEC640AEE, 0x0003000E),
        (0xF491131B, 0x00040000),
        (0xF4B91343, 0x00040001),
        (0xF4C2134C, 0x00040002),
        (0xF4D4135E, 0x00040003),
        (0xF4DA1364, 0x00040004),
        (0xF4E71371, 0x00040005),
        (0xF4FC1386, 0x00040006),
        (0xF514139E, 0x00040007),
        (0xF52913B3, 0x00040008),
        (0xF53313BD, 0x00040009),
        (0xF54013CA, 0x0004000A),
        (0xF56813F2, 0x0004000B),
        (0xF5971421, 0x0004000C),
        (0xF59C1426, 0x0004000D),
        (0xF5A1142B, 0x0004000E),
        (0xF5D2145C, 0x0004000F),
        (0xF5ED1477, 0x00040010),
        (0xF5FD1487, 0x00040011),
        (0xF60F1499, 0x00040012),
        (0xF61714A1, 0x00040013),
        (0xF62014AA, 0x00040014),
        (0xF63614C0, 0x00040015),
        (0xF64B14D5, 0x00040016),
        (0xF65014DA, 0x00050000),
        (0xF66414EE, 0x00050001),
        (0xF66B14F5, 0x00050002),
        (0xF66C14F6, 0x00060000),
        (0xF6A0152A, 0x00060001),
        (0xF6A91533, 0x00060002),
        (0xF6D0155A, 0x00060003),
        (0xF6D1155B, 0x00060004),
        (0xF6DA1564, 0x00060005),
        (0xF6DB1565, 0x00060006),
        (0xF6EB1575, 0x00060007),
        (0xF6ED1577, 0x00060008),
        (0xE21A00A4, 0x00070000),
        (0xE21D00A7, 0x00070001),
        (0xE1E5006F, 0x00070002),
        (0xE22200AC, 0x00070003),
        (0xE21800A2, 0x00070004),
        (0xE21900A3, 0x00070005),
        (0xE22100AB, 0x00070006),
        (0xE21F00A9, 0x00070007),
        (0xE21E00A8, 0x00070008),
        (0xE203008D, 0x00070009),
        (0xE21600A0, 0x0007000A),
        (0xE21700A1, 0x0007000B),
        (0xE22400AE, 0x0007000C),
        (0xE22300AD, 0x0007000D),
        (0xE21B00A5, 0x0007000E),
        (0xE21C00A6, 0x0007000F),
        (0xE22000AA, 0x00070010),
        (0xE49C0326, 0x00070011),
        (0xE49D0327, 0x00070012),
        (0xE22500AF, 0x00080013),
        (0xE23A00C4, 0x00080014),
        (0xE24900D3, 0x00080015),
        (0xE25600E0, 0x00080016),
        (0xE26100EB, 0x00080017),
        (0xE26700F1, 0x00080018),
        (0xE2B5013F, 0x00080019),
        (0xE2D60160, 0x0008001A),
        (0xE2EF0179, 0x0008001B),
        (0xE2F4017E, 0x0008001C),
        (0xE300018A, 0x0008001D),
        (0xE310019A, 0x0008001E),
        (0xE31901A3, 0x0008001F),
        (0xE32D01B7, 0x00080020),
        (0xE491031B, 0x00080021),
        (0xE49B0325, 0x00080022),
        (0xE49E0328, 0x00090000),
        (0xE87F0709, 0x00090001),
        (0xF74115CB, 0x000C0000),
        (0xF74315CD, 0x000C000E),
        (0xF76215EC, 0x000C0015),
        (0xF76C15F6, 0x000C0016),
        (0xF7781602, 0x000C0017),
        (0xF784160E, 0x000C0018),
        (0xF7891613, 0x000C0019),
        (0xF792161C, 0x000C001C),
        (0xF7A81632, 0x000C0024),
        (0xF7B2163C, 0x000C0025),
        (0xF7BA1644, 0x000C0028),
        (0xF7ED1677, 0x000C002A),
        (0xF7F2167C, 0x000C002B),
        (0xF7F61680, 0x000C002C),
        (0xF7FA1684, 0x000C002D),
        (0xF7FE1688, 0x000C002E),
        (0xF802168C, 0x000C002F),
        (0xF8061690, 0x000C0030),
        (0xF80A1694, 0x000C0031),
        (0xF80E1698, 0x000C0032),
        (0xF812169C, 0x000C0033),
    ]);
}

