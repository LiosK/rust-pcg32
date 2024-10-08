//! # No-frills PCG32 random number generator implementation
//!
//! It implements the [PCG32 random number generator] (and really only that).
//!
//! [PCG32 random number generator]: https://www.pcg-random.org/download.html
//!
//! ```rust
//! let mut g = pcg32::Pcg32::new(0xff30_6525_39eb_eaa9, 0x315b_fae4_8ade_2146);
//!
//! assert_eq!(g.generate(), 0xf986_95e1);
//! assert_eq!(g.generate(), 0x7e39_20e2);
//! ```
//!
//! This crate is `no_std` compatible.

#![no_std]

/// PCG32 multiplier.
const MUL: u64 = 6364136223846793005;

/// A PCG32 random number generator.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    /// Initializes a PCG32 generator with two `u64` seeds.
    ///
    /// The arguments specify the starting state and the output sequence, respectively, for the
    /// constructed generator. You can pass any 64-bit value to each argument, though the most
    /// significant bit of `initseq` is ignored. See the [`pcg32_srandom_r`] documentation of the
    /// official library for further details.
    ///
    /// [`pcg32_srandom_r`]: https://www.pcg-random.org/using-pcg-c-basic.html#pcg32-srandom-r-rngptr-initstate-initseq
    #[inline]
    pub const fn new(initstate: u64, initseq: u64) -> Self {
        let inc = (initseq << 1) | 1;
        Self {
            state: inc
                .wrapping_add(initstate)
                .wrapping_mul(MUL)
                .wrapping_add(inc),
            inc,
        }
    }

    /// Generates a pseudorandom uniformly distributed 32-bit unsigned integer.
    #[inline]
    pub fn generate(&mut self) -> u32 {
        let s = self.state;
        self.state = s.wrapping_mul(MUL).wrapping_add(self.inc);
        let xorshifted = (((s >> 18) ^ s) >> 27) as u32;
        xorshifted.rotate_right((s >> 59) as u32)
    }

    /// Alias to `generate` for backward compatibility.
    #[doc(hidden)]
    #[inline]
    pub fn r#gen(&mut self) -> u32 {
        self.generate()
    }
}

impl Default for Pcg32 {
    #[inline]
    fn default() -> Self {
        // adapted from `PCG32_INITIALIZER` of the official library
        Self {
            state: 0x853c49e6748fea9b,
            inc: 0xda3e39cb94b95bdb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pcg32;

    /// Compares the generated sequence with the one obtained from the [official library].
    ///
    /// [official library]: https://www.pcg-random.org/download.html
    #[test]
    fn compare_with_official_library() {
        let cases = [
            (
                Pcg32::default(),
                [
                    0x152ca78d, 0x027c6003, 0xcb07bbf3, 0xf98befee, 0x1cd777e3, 0xa4e29590,
                    0x661e4b6d, 0x093b9e0e, 0xb7e9851d, 0xe71f2e4d, 0xbdb2a071, 0x469753f2,
                    0xd4195b44, 0x8d5b2e0a, 0xe749bf46, 0x7370bb1c, 0xb9ad21f8, 0xcfad21e0,
                    0x843fa922, 0xf16b535e, 0x8be6e048, 0xdd7e3483, 0xd136c7ea, 0x7886b716,
                    0xdeafd023, 0xa56eeebd, 0x449dff2a, 0x30a8f133, 0x5fb4f0ef, 0x0e8c4479,
                    0x1b2326a7, 0xab7f98df, 0x12423bb8, 0xbc693c36, 0x6a3430a1, 0x53aeb48e,
                    0xd0b0846f, 0x07b30dc1, 0x3daa400e, 0xee475503, 0xcbd06115, 0x6b442912,
                    0xa21b7bf2, 0xa1497036, 0xdbaa7d4c, 0xee844a19, 0x1242149f, 0x9b7f2319,
                    0x13b5574a, 0xdacbbda7, 0x6e6f51ac, 0xbb2ce758, 0xa40b4c79, 0x52a17060,
                    0x82810ae9, 0xba62b903, 0x216bcb52, 0x0c78819d, 0x586ebe6f, 0xe539ce35,
                    0x2bf68cef, 0x2aca379a, 0x249ca1dd, 0x9823ce15, 0x40faab65, 0xe382c24e,
                    0x35636845, 0xd2e38084, 0x914b5c23, 0x755bfb5c, 0xefc5eada, 0x752a8073,
                    0x55a2c490, 0xae755d8d, 0xf6295e62, 0xe066a750, 0xdc6fcd8b, 0x269948c6,
                    0x0c34ddff, 0xe95a401c, 0xf90e404a, 0x4d9e2ed2, 0x31146cd4, 0x85d595dd,
                    0x2671f802, 0x01039001, 0x9696a286, 0x0833f03d, 0xd132f08d, 0xaa8f5d48,
                    0xf4cdd3ec, 0x3d9f75d5, 0xe9cb0fa5, 0x0333d581, 0x26f5cbf2, 0xe6e318a5,
                    0xc1b495a7, 0x2c165c7b, 0x8ef4a460, 0x2fb3b822, 0xded1f339, 0xbb0f2779,
                    0x993a456c, 0xaf4adfc5, 0x81befafc, 0xd2782e01, 0xa31969a4, 0xd162454b,
                    0xaeb32e05, 0x2b574d96, 0x457594d4, 0x5c6b9dae, 0x58aed378, 0x957f1712,
                    0x456acaf8, 0x04e34857, 0x5c5fe2b4, 0xfce85f57, 0x1579d5ba, 0xcb84f4fa,
                    0xd60e4d1b, 0x12bf8237, 0x9dacac42, 0x39c33b82, 0x2ea83e2f, 0x06305065,
                    0x2c09559e, 0x7069564b, 0x0388ada6, 0x13bf868e, 0x3856f6d1, 0x6f306183,
                    0x0f4974e3, 0x1c56c0f0, 0x499e5d63, 0x15423dbd, 0x407fc8a0, 0xa9c97b23,
                    0xdfdffdb1, 0x74b65c7c, 0x11efa393, 0x4bf1609f, 0x24666240, 0xd5abb7da,
                    0xf6ff5afb, 0x4ce224b4, 0x07bfbf6d, 0xf92e8326, 0xec098605, 0xa64df396,
                    0x365a5867, 0x0e2d8454, 0xaf98eae6, 0x03f6076d, 0x55c3bd38, 0x0007c9bb,
                    0x9b8fc18f, 0x52667654, 0xa505ec95, 0x14e76502, 0xc56f9a27, 0xa1c0d691,
                    0xd1be0215, 0x87fd6765, 0x38488a79, 0xb0e92730, 0xc7b7991a, 0xaec5501c,
                    0x8a30014f, 0xad0f78ab, 0x5b55ca17, 0x7d534328, 0x24d4bf4b, 0xe4a0a4ea,
                    0xd3477948, 0x5091bbca, 0xd5652ace, 0xb7ae7ff5, 0xc8286a8d, 0x11f06d6f,
                    0x16c2fcbe, 0x1b056dee, 0x8682ad52, 0xd8ed7ce4, 0xd3baa41f, 0xc512730b,
                    0x06e98ce6, 0x1ebb80d8, 0x1fc324a2, 0x3ae73691, 0x31c92de0, 0x74c190d2,
                    0xbd01a22c, 0xd7853911, 0x4b6c61d6, 0x617f2bf4, 0x7fec94a2, 0x23b4df61,
                    0x6e313ca3, 0xa581a91f, 0x865e3640, 0x46d33a4a, 0x3b69032f, 0x4e5c79b8,
                    0x119fb6db, 0x12e9ec15, 0xf58379dc, 0xb8050454, 0x0a17d9be, 0x7f772c04,
                    0xe11068e9, 0x859fb1de, 0x66915631, 0x566194b8, 0x0e9bc96a, 0x25f0ec0a,
                    0x068a4b0d, 0x812aca2b, 0x96099ea7, 0x1280bac3, 0x9d90e17f, 0x23479d99,
                    0xf4a59874, 0xa640945f, 0x6e386ccd, 0x8ae7965c, 0x9623da01, 0x8d878907,
                    0x3f52e398, 0x237673b0, 0x99de2c25, 0x03a32d0c, 0x647cd5f4, 0x2f3a418e,
                    0x70e415f5, 0xbb5054ee, 0x97135f89, 0xbea5f514, 0xcaecd59f, 0x102724ab,
                    0xcd597253, 0xce46fb98, 0xbc55f6fa, 0xdd3188d6, 0x9528a70e, 0x641ac279,
                    0xcf4f0ce5, 0x1f8a509d, 0xcce7797c, 0x1aff28ca, 0xef7d31c7, 0xe9512931,
                    0x9f5f01d8, 0x94a3faf9, 0x28f9d8bd, 0xd2bb5c90,
                ],
            ),
            (
                Pcg32::new(0x99a93b4a325d9348, 0xebee5b2aa08119cb),
                [
                    0x127519df, 0xc4864313, 0xfe9cb540, 0x7eb42740, 0xff3dd1fc, 0xfe1390d7,
                    0xdd743788, 0x7e3259ac, 0x5c6f5c57, 0x1f669aa5, 0xe9ba2fd8, 0xebb5c6e9,
                    0xabefa4a7, 0x13e764db, 0x7888662f, 0x536d9c73, 0x527406f1, 0x0e155178,
                    0xab544c6b, 0xa64049a3, 0x3344a835, 0x813e7aef, 0xc6e5f719, 0x1fec7bbb,
                    0xe31fd2c6, 0x4370a7c8, 0x2a07edf2, 0xef7fe8a0, 0x12dc1a70, 0x765e35ec,
                    0x1bdaff08, 0x7c85d60c, 0x932ffce8, 0x6ea81ad6, 0x3baca8ff, 0xb8ab1996,
                    0x23c8ba5c, 0xf23469ce, 0xfb4e4dfa, 0xd32bd679, 0xdae372eb, 0xd9fb8f2e,
                    0x48979891, 0x4e2260e8, 0xdd3a6dc5, 0x7caf0479, 0x84d4ca3c, 0xa25f187e,
                    0x2e520b95, 0xa5854594, 0xfc1850c6, 0x255b8042, 0x8aafb9d2, 0xf9d41768,
                    0x2b8cd2b9, 0x476e1e26, 0x8d1ecf7d, 0xd5cca8ae, 0x10354dca, 0x0801a9af,
                    0xbc7a60a9, 0x78cf57d4, 0x38d76918, 0x887a292d, 0xd84098e4, 0xdda354af,
                    0xf2bb4777, 0xefb16f30, 0x500a1721, 0x2e4710f8, 0xa303de3c, 0x5567f9a9,
                    0x42821552, 0x7ce6271f, 0x2a265775, 0x3c2aebe0, 0x1abfe20d, 0x10f1853b,
                    0x4135907a, 0x072d509e, 0x63ec8b26, 0xe27776ac, 0x32afb6d4, 0xf33d2ae3,
                    0x2e46fb37, 0xa856a0f7, 0xbc081038, 0x13b665f5, 0xb8b43606, 0xa63cb402,
                    0x94258220, 0xe4d3751c, 0xb1b08201, 0xee11bae5, 0x0d67cc05, 0x39b1d832,
                    0xe8cadfbc, 0x91ad4597, 0x282a4b15, 0x1b003297, 0x7d33e847, 0x3bee5826,
                    0xf33879e3, 0x2d9e0fc8, 0x2c8b05bd, 0xa8fad28f, 0x4f4b2aaa, 0x9072e672,
                    0x156b3da9, 0x21d3e3b4, 0x7be201f5, 0xd19e2a13, 0x0fba2113, 0x65644711,
                    0xddec2b7e, 0xaf1f1155, 0xcd7a8ea7, 0x0dda527a, 0x2f0d0ea7, 0x98d27af2,
                    0x5033f447, 0x0c0c7c5f, 0xfa104646, 0xb34ef41d, 0xa7daa81d, 0x6fa17c09,
                    0x90abecbc, 0x6cf337f1, 0xe4dcb409, 0x5bd48e94, 0xd208f126, 0x47f2dcda,
                    0xbb9ff68f, 0x6541d0c0, 0x0011b056, 0xedcca0db, 0x21504f60, 0xb3647dd6,
                    0x41da032a, 0xd568afc8, 0x80db77bc, 0xe95d0a53, 0x4a89dbaa, 0xadf61638,
                    0x504d6a2d, 0xe36eea0d, 0x7d5bd50f, 0x8b1b3b4b, 0x72ac565a, 0x2d7a189f,
                    0x68108354, 0x2dda2e32, 0x068fb3e1, 0x4d49599a, 0xd1ec4ab9, 0x9f26b75e,
                    0xcfff3a0d, 0x8820913f, 0x127a8666, 0xe70681bd, 0x1e58c9db, 0xd6612547,
                    0xfd1096c7, 0x7e136426, 0x1b1189eb, 0xfc332fdc, 0xb64100f5, 0x0b07901b,
                    0x7de3c9ac, 0x60dcabdc, 0x5243a4a3, 0x8ee09c40, 0x7590fa9b, 0x46400789,
                    0xbeef2091, 0x10490378, 0xa57d8203, 0xbb63ce85, 0x25d608f3, 0x51c4115e,
                    0xd89cf0d1, 0x1af3bbff, 0xa036410e, 0x7d7a39e7, 0xedaa6edf, 0xaffff2ea,
                    0x2959f568, 0xa1edea20, 0x8c1c1140, 0x7b7192c1, 0xe7c5ae94, 0x430b44d5,
                    0x710a13be, 0x8906f53c, 0x040c1c36, 0x207763c9, 0x323eb969, 0x3020a602,
                    0x7903af49, 0x5759c42e, 0xdfee2340, 0x69f09e9a, 0x24f6abe3, 0x02219f3a,
                    0x3939a751, 0x63610783, 0xfa2bb039, 0xbc5fae01, 0x7f0461f0, 0xfc2d0ffe,
                    0x22fb8646, 0x23595f9b, 0xb7114a6a, 0xe9aad913, 0x046805ab, 0xd862f9a2,
                    0x621b4ceb, 0xc6495c46, 0x96b01c24, 0x8c2a220c, 0x0590595f, 0x999ad813,
                    0xdd0b10b4, 0x899ebb07, 0x9e58adef, 0x5763d185, 0xfc1acea7, 0xc124202f,
                    0x8a9370c2, 0x7b08b727, 0x8331a196, 0xd2233ae0, 0x79a51c3f, 0x5af9c785,
                    0x56603424, 0xe6db5909, 0x812bebf9, 0x4028d726, 0x3c26dfb9, 0x716d3dc0,
                    0xa0ee13b1, 0xe1ecabc7, 0xdfac265b, 0x55a572c7, 0x56d4c82b, 0x8bbadf08,
                    0x62694c6d, 0x02277a99, 0xc6131c4a, 0xaed3b06a, 0xefe9e882, 0xe5e535ae,
                    0x4793c535, 0x341985fa, 0xcf68a90d, 0xca21f98c,
                ],
            ),
            (
                Pcg32::new(0x01f125a59ffb5a04, 0x70f7e17e846603e5),
                [
                    0x6da86395, 0xdbcec1c7, 0x80a14a91, 0xb72b2429, 0x172258a7, 0x15c5824e,
                    0xc5a6692a, 0xf82abdd0, 0x3ec281f8, 0x2b9e265d, 0x1f837482, 0x07e8e84f,
                    0x35a74011, 0xa86e1e5b, 0x7da2c981, 0xc5ff4ab2, 0x471e7ec4, 0xfe286c32,
                    0x59a0b4b0, 0xa9f1b3f6, 0x3e60dc2c, 0xf18edcf2, 0xbf1f58b2, 0x47d9b1f0,
                    0xc1192f37, 0x74e3e2e2, 0xc796fc4b, 0xe2e53c87, 0x31610955, 0x270be31e,
                    0xeac2b347, 0x65fc8092, 0xc144842a, 0x9352b7e5, 0xa12e9ff1, 0x799a3e2b,
                    0xa9d12580, 0xa9efffb1, 0x5b9857f8, 0x361384d6, 0x5d02341f, 0x233fb6f1,
                    0x4defa255, 0x21f5d4e9, 0x2579098c, 0x134b3a31, 0xd27e7cbe, 0x7ff06e0a,
                    0xd8e83dc6, 0xfa2b6a32, 0xefbc290d, 0x02cd5e7c, 0x71fe175a, 0xaba0d75d,
                    0x7f81c080, 0xca300ac0, 0x5593c949, 0xb59a8770, 0x0caa34d1, 0xa0e01f3e,
                    0x9c5537b6, 0xc10f3029, 0xcfe159c4, 0x03baced2, 0xcf8288f5, 0xee26070c,
                    0xf75b3cb8, 0x03feff08, 0xa6fb2a58, 0x37f0d5f0, 0x82ae6470, 0xf40da398,
                    0x762fd3d9, 0xcc772b6c, 0xd68dff2a, 0xef4c85a4, 0x89fe05f5, 0xce2b63bd,
                    0x2c4b7daa, 0xfb065b10, 0xe2f3bb80, 0x857ea2af, 0xef2f893f, 0xf0776270,
                    0x015c23a9, 0xcbd8766e, 0xbefbadde, 0x20a6cfcc, 0xf3102219, 0x7947813b,
                    0xe38c7f4c, 0x9b7275b0, 0xfe8187e3, 0x69636183, 0x8f372c41, 0x818de7c4,
                    0xc6cd34ca, 0x0a797f1c, 0x22b1b17a, 0x1d32fde9, 0xe9363cb9, 0x71645380,
                    0xea8894f4, 0xa8012657, 0xe5733f83, 0xbb514879, 0x3ce4ebde, 0x81d469c1,
                    0x7413c35d, 0x6cf7101e, 0x486f8ebc, 0x4d395cb6, 0x26583b90, 0x0d2b01a0,
                    0x0d52bc1a, 0x143e4102, 0x0cd29bae, 0xdd49f88c, 0x2cc2dbc1, 0x513ca6d1,
                    0x097d00ed, 0x03d338ba, 0x1c5f53b8, 0x13c3abbb, 0xd3319566, 0xd6e649f3,
                    0x70071842, 0x76606b21, 0xa1a344a5, 0xae7b2294, 0x1d7f6801, 0x9c85d2c5,
                    0xbb4cb9db, 0x6d3bf2c9, 0x74cc9c3e, 0xe14788c1, 0x2132e74a, 0xa00d46f1,
                    0xdb89f582, 0x07638255, 0x94681e09, 0x82c13cab, 0x905ebaf1, 0x110d98a1,
                    0xf4cee4e1, 0x79643224, 0x6da94d53, 0xbed4ed73, 0xad4fb32f, 0xebcc0810,
                    0x1d7ec279, 0xec0c8563, 0x4c4a0d78, 0xa722d1d1, 0xcba0c856, 0x914fe28f,
                    0x43efd9a9, 0x6ad1ec6b, 0x2c8697b1, 0x2780e20b, 0x02c4d4ae, 0x9b38c6e8,
                    0xea38274f, 0x3e563245, 0xe2faa953, 0x14666831, 0x7e3a92ea, 0xaa8a534f,
                    0x92751ea0, 0x6509ad89, 0x3ad8c88b, 0xba2d4ce6, 0x9ecc2298, 0x212c9950,
                    0xf3c25cf2, 0x1f09f156, 0x7623e2d5, 0x4158d47f, 0xe226285b, 0x76b2b631,
                    0x2f5ec08c, 0xd8885fe6, 0x218e2281, 0x69e3308e, 0x5399d2c1, 0xc603dd48,
                    0xc935c4ab, 0x2db929e7, 0x88b7b33d, 0x749078a5, 0x2257cd08, 0x87e321c3,
                    0x3aa3c54b, 0x62d5bce1, 0x174327c9, 0x06ff80d4, 0xa0365d3b, 0xdcd591e7,
                    0x55904f86, 0x117f093c, 0x58d68d12, 0xf62c9394, 0x5d485492, 0x8957690e,
                    0x379aeab7, 0x8c84b453, 0x22cdfc5f, 0xb66c2131, 0x3f2902ee, 0xdf340005,
                    0x710d7938, 0x77877090, 0x0ed4fae1, 0xe2b2b9ac, 0xbaeff8b9, 0x8e549f03,
                    0xa0308304, 0xa89f543b, 0x8740fa6b, 0x3da9c31e, 0x205f369d, 0x0cdc098d,
                    0xf4ad76b7, 0xde4b4973, 0xbeb490ae, 0xf242300f, 0xafbddf8d, 0x3eefe236,
                    0x2f9ef632, 0xeac35736, 0xba6b525f, 0x6d033969, 0x97983d13, 0xc735d515,
                    0x9c27a0f7, 0x82b5e8c2, 0xbbc8e3ca, 0x7c08587b, 0x157a1c08, 0x04e294d0,
                    0x3a8cca0a, 0x8f7a7e7b, 0x64d1c010, 0xaa2b985b, 0x2b0bb0fb, 0x24efefbb,
                    0xd6ce4d86, 0x7010b232, 0x0777fc39, 0x169957bf, 0xa951ed0e, 0xec211445,
                    0xb0f452ef, 0xebecb6f2, 0xe6ef9e33, 0x951a73db,
                ],
            ),
        ];

        for (mut rng, expected_sequence) in cases {
            for expected in expected_sequence {
                assert_eq!(rng.generate(), expected);
            }
        }
    }

    /// Compares the generated sequence with the one obtained from the [rand_pcg] crate.
    ///
    /// [rand_pcg]: https://crates.io/crates/rand_pcg
    #[test]
    fn compare_with_rand_pcg_crate() {
        use rand_core::RngCore;

        let seeds = [
            (0xf930cb32d6de10de, 0x9235e8751c5b6654),
            (0x15a055655308631b, 0xd7c57990690d1a4e),
            (0xb7c9d5d05bf40106, 0xc82c2e11fb716d16),
            (0xea8027b445dec2b1, 0x7081640a24ccaada),
            (0x49d01b4c727eed8a, 0x03980cb5fe1fdbd4),
            (0xa8ea425ece6cd7df, 0xe16021284e459fc4),
            (0x3365ed708a774db9, 0xe90e267fe44789c9),
            (0x37ef056e3220f002, 0xeee9c87acef4211b),
            (0xda8526447ac03f86, 0x6b57163e0ace8e22),
            (0x4b4d034eff74b3fe, 0x3087cfde49085b6f),
            (0xb71cdda7777030fe, 0x2e40223adaa402ab),
            (0x93802b691874d1a6, 0x084aec86255c2f12),
            (0x4dd3c81cd769e6fe, 0xc7d90b7c9c5aeaf8),
            (0x697beaef9a920157, 0x7fdb4cbff7f5364e),
            (0x80ca79a58db8fc94, 0x17d77e0abf68e0de),
            (0x5de9a076d26b168d, 0x34788f3110ac0ed5),
        ];

        for (initstate, initseq) in seeds {
            let mut ours = Pcg32::new(initstate, initseq);
            let mut theirs = rand_pcg::Pcg32::new(initstate, initseq);
            for _ in 0..0x1_0000 {
                assert_eq!(ours.generate(), theirs.next_u32());
            }
        }
    }
}
