use bimap::BiMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref citadel_objects_map: BiMap<u32, u32> = BiMap::from_iter(vec![
        (0xEF500DDA, 0x00010000),
        (0xE25100DB, 0x00010001),
        (0xEDB70C41, 0x00020000),
        (0xEE000C8A, 0x00020001),
        (0xEAF2097C, 0x00020002),
        (0xECA90B33, 0x00020003),
        (0xE6780502, 0x00020004),
        (0xE91807A2, 0x00020005),
        (0xE9F0087A, 0x00020006),
        (0xEA5208DC, 0x00020007),
        (0xEC660AF0, 0x00020008),
        (0xE83906C3, 0x00020009),
        (0xED680BF2, 0x0002000A),
        (0xEE3B0CC5, 0x0002000B),
        (0xEEB00D3A, 0x0002000C),
        (0xE980080A, 0x0002000D),
        (0xEFF70E81, 0x0002000E),
        (0xF04D0ED7, 0x0002000F),
        (0xE95607E0, 0x00020010),
        (0xF08F0F19, 0x00020011),
        (0xF0DA0F64, 0x00020012),
        (0xEB6909F3, 0x00020013),
        (0xF10A0F94, 0x00020014),
        (0xEC1E0AA8, 0x00020015),
        (0xF15A0FE4, 0x00020016),
        (0xF18A1014, 0x00020017),
        (0xE5F90483, 0x00030000),
        (0xE56B03F5, 0x00030001),
        (0xF1CF1059, 0x00030002),
        (0xF1F71081, 0x00030003),
        (0xE404028E, 0x00030004),
        (0xE52A03B4, 0x00030005),
        (0xF2061090, 0x00030006),
        (0xF23110BB, 0x00030007),
        (0xF23D10C7, 0x00030008),
        (0xF26410EE, 0x00030009),
        (0xF2771101, 0x0003000A),
        (0xF2961120, 0x0003000B),
        (0xE65804E2, 0x0003000C),
        (0xE66E04F8, 0x0003000D),
        (0xF2D81162, 0x0003000E),
        (0xF2DF1169, 0x00040000),
        (0xF3091193, 0x00040001),
        (0xF32A11B4, 0x00040002),
        (0xF33611C0, 0x00040003),
        (0xF33C11C6, 0x00040004),
        (0xF35111DB, 0x00040005),
        (0xF35611E0, 0x00040006),
        (0xF35B11E5, 0x00040007),
        (0xF36011EA, 0x00040008),
        (0xF37311FD, 0x00040009),
        (0xF3791203, 0x0004000A),
        (0xE61D04A7, 0x0004000B),
        (0xF38D1217, 0x0004000C),
        (0xF39D1227, 0x0004000D),
        (0xF3A2122C, 0x0004000E),
        (0xF3B4123E, 0x0004000F),
        (0xF3CC1256, 0x00040010),
        (0xF3E0126A, 0x00040011),
        (0xF3E3126D, 0x00040012),
        (0xF3E71271, 0x00040013),
        (0xF3EA1274, 0x00040014),
        (0xF3ED1277, 0x00040015),
        (0xF3F0127A, 0x00050000),
        (0xF4091293, 0x00050001),
        (0xF414129E, 0x00050002),
        (0xF415129F, 0x00060000),
        (0xF44B12D5, 0x00060001),
        (0xF45412DE, 0x00060002),
        (0xF47212FC, 0x00060003),
        (0xF47312FD, 0x00060004),
        (0xF47C1306, 0x00060005),
        (0xF47D1307, 0x00060006),
        (0xF47F1309, 0x00060007),
        (0xF481130B, 0x00060008),
        (0xE213009D, 0x00070000),
        (0xE24400CE, 0x00070001),
        (0xE24200CC, 0x00070002),
        (0xE24C00D6, 0x00070003),
        (0xE24F00D9, 0x00070004),
        (0xE24600D0, 0x00070005),
        (0xE24800D2, 0x00070006),
        (0xE24A00D4, 0x00070007),
        (0xE24500CF, 0x00070008),
        (0xE24300CD, 0x00070009),
        (0xE24100CB, 0x0007000A),
        (0xE22E00B8, 0x0007000B),
        (0xE24D00D7, 0x0007000C),
        (0xF482130C, 0x0007000D),
        (0xE24700D1, 0x0007000E),
        (0xE24900D3, 0x0007000F),
        (0xE24B00D5, 0x00070010),
        (0xE24E00D8, 0x00070011),
        (0xF483130D, 0x00070012),
        (0xE25000DA, 0x00080011),
        (0xEF360DC0, 0x000A0011),
        (0xF48B1315, 0x000C0004),
        (0xF49A1324, 0x000C000C),
        (0xF49F1329, 0x000C000D),
        (0xF4A3132D, 0x000C000E),
        (0xF4A71331, 0x000C000F),
        (0xF4AB1335, 0x000C0010),
        (0xF4AF1339, 0x000C0011),
        (0xF4B3133D, 0x000C0012),
        (0xF4B71341, 0x000C0013),
        (0xF4BB1345, 0x000C0014),
        (0xF4BF1349, 0x000C0015),
    ]);
}


