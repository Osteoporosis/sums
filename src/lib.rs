#[inline]
pub fn for_sum(values: &[f64]) -> f64 {
    let mut s = 0.0;
    for i in values {
        s += i;
    }
    s
}

#[inline]
pub fn iter_sum(values: &[f64]) -> f64 {
    values.iter().sum()
}

pub const BLOCK: usize = 512;

pub fn fold_sum(values: &[f64]) -> f64 {
    let len = values.len();
    if len < BLOCK * BLOCK / 2 {
        return chunked_sum(values);
    }

    let capacity_current = (len + BLOCK - 1) / BLOCK;
    let mut buffer_current = Vec::with_capacity(capacity_current);
    let capacity_next = (capacity_current + BLOCK - 1) / BLOCK;
    let mut buffer_next = Vec::with_capacity(capacity_next);

    let (chunks, remainder) = values.as_chunks::<BLOCK>();
    buffer_current.extend(chunks.iter().map(|chunk| chunked_sum_512_to_1(chunk)));

    if !remainder.is_empty() {
        buffer_current.push(chunked_sum(remainder));
        // buffer_current.push(remainder.iter().sum());
    }

    while buffer_current.len() >= BLOCK * BLOCK / 2 {
        buffer_next.clear();

        let (chunks, remainder) = buffer_current.as_chunks::<BLOCK>();
        buffer_next.extend(chunks.iter().map(|chunk| chunked_sum_512_to_1(chunk)));

        if !remainder.is_empty() {
            buffer_next.push(chunked_sum(remainder));
            // buffer_next.push(remainder.iter().sum());
        }

        std::mem::swap(&mut buffer_current, &mut buffer_next);
    }

    chunked_sum(&buffer_current)
}

#[inline]
pub fn chunked_sum(values: &[f64]) -> f64 {
    let (remainder, mut s) = chunked_reduce(values);
    s[0] += remainder.iter().sum::<f64>();
    sum_8_to_1(&s)
}

#[inline]
fn sum_8_to_1(values: &[f64; 8]) -> f64 {
    let values = [
        values[0] + values[4],
        values[1] + values[5],
        values[2] + values[6],
        values[3] + values[7],
    ];
    (values[0] + values[2]) + (values[1] + values[3])
}

fn chunked_reduce(values: &[f64]) -> (&[f64], [f64; 8]) {
    let mut s0 = 0.0;
    let mut s1 = 0.0;
    let mut s2 = 0.0;
    let mut s3 = 0.0;
    let mut s4 = 0.0;
    let mut s5 = 0.0;
    let mut s6 = 0.0;
    let mut s7 = 0.0;
    let mut s8 = 0.0;
    let mut s9 = 0.0;
    let mut s10 = 0.0;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s13 = 0.0;
    let mut s14 = 0.0;
    let mut s15 = 0.0;

    let (chunks, remainder) = values.as_chunks::<16>();
    for &[
        c0,
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
        c8,
        c9,
        c10,
        c11,
        c12,
        c13,
        c14,
        c15,
    ] in chunks
    {
        s0 += c0;
        s1 += c1;
        s2 += c2;
        s3 += c3;
        s4 += c4;
        s5 += c5;
        s6 += c6;
        s7 += c7;
        s8 += c8;
        s9 += c9;
        s10 += c10;
        s11 += c11;
        s12 += c12;
        s13 += c13;
        s14 += c14;
        s15 += c15;
    }

    (
        remainder,
        [
            s0 + s8,
            s1 + s9,
            s2 + s10,
            s3 + s11,
            s4 + s12,
            s5 + s13,
            s6 + s14,
            s7 + s15,
        ],
    )
}

#[inline]
fn chunked_sum_512_to_1(values: &[f64; 512]) -> f64 {
    let values = chunked_reduce_512_to_8(values);
    sum_8_to_1(&values)
}

fn chunked_reduce_512_to_8(values: &[f64; 512]) -> [f64; 8] {
    let mut s0 = 0.0;
    let mut s1 = 0.0;
    let mut s2 = 0.0;
    let mut s3 = 0.0;
    let mut s4 = 0.0;
    let mut s5 = 0.0;
    let mut s6 = 0.0;
    let mut s7 = 0.0;
    let mut s8 = 0.0;
    let mut s9 = 0.0;
    let mut s10 = 0.0;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s13 = 0.0;
    let mut s14 = 0.0;
    let mut s15 = 0.0;

    let (chunks, _remainder) = values.as_chunks::<16>();
    for &[
        c0,
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
        c8,
        c9,
        c10,
        c11,
        c12,
        c13,
        c14,
        c15,
    ] in chunks
    {
        s0 += c0;
        s1 += c1;
        s2 += c2;
        s3 += c3;
        s4 += c4;
        s5 += c5;
        s6 += c6;
        s7 += c7;
        s8 += c8;
        s9 += c9;
        s10 += c10;
        s11 += c11;
        s12 += c12;
        s13 += c13;
        s14 += c14;
        s15 += c15;
    }

    [
        s0 + s8,
        s1 + s9,
        s2 + s10,
        s3 + s11,
        s4 + s12,
        s5 + s13,
        s6 + s14,
        s7 + s15,
    ]
}

pub fn wide_sum_fold0(values: &[f64]) -> f64 {
    let mut s0 = 0.0;
    let mut s1 = 0.0;
    let mut s2 = 0.0;
    let mut s3 = 0.0;
    let mut s4 = 0.0;
    let mut s5 = 0.0;
    let mut s6 = 0.0;
    let mut s7 = 0.0;
    let mut s8 = 0.0;
    let mut s9 = 0.0;
    let mut s10 = 0.0;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s13 = 0.0;
    let mut s14 = 0.0;
    let mut s15 = 0.0;

    let (chunks, remainder) = values.as_chunks::<16>();
    for &[
        c0,
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
        c8,
        c9,
        c10,
        c11,
        c12,
        c13,
        c14,
        c15,
    ] in chunks
    {
        s0 += c0;
        s1 += c1;
        s2 += c2;
        s3 += c3;
        s4 += c4;
        s5 += c5;
        s6 += c6;
        s7 += c7;
        s8 += c8;
        s9 += c9;
        s10 += c10;
        s11 += c11;
        s12 += c12;
        s13 += c13;
        s14 += c14;
        s15 += c15;
    }

    let r = remainder.iter().sum::<f64>();

    r + s0 + s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9 + s10 + s11 + s12 + s13 + s14 + s15
}

pub fn wide_sum_fold1(values: &[f64]) -> f64 {
    let mut s0 = 0.0;
    let mut s1 = 0.0;
    let mut s2 = 0.0;
    let mut s3 = 0.0;
    let mut s4 = 0.0;
    let mut s5 = 0.0;
    let mut s6 = 0.0;
    let mut s7 = 0.0;
    let mut s8 = 0.0;
    let mut s9 = 0.0;
    let mut s10 = 0.0;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s13 = 0.0;
    let mut s14 = 0.0;
    let mut s15 = 0.0;

    let (chunks, remainder) = values.as_chunks::<16>();
    for &[
        c0,
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
        c8,
        c9,
        c10,
        c11,
        c12,
        c13,
        c14,
        c15,
    ] in chunks
    {
        s0 += c0;
        s1 += c1;
        s2 += c2;
        s3 += c3;
        s4 += c4;
        s5 += c5;
        s6 += c6;
        s7 += c7;
        s8 += c8;
        s9 += c9;
        s10 += c10;
        s11 += c11;
        s12 += c12;
        s13 += c13;
        s14 += c14;
        s15 += c15;
    }

    let r = remainder.iter().sum::<f64>();

    let s = [
        s0 + s8,
        s1 + s9,
        s2 + s10,
        s3 + s11,
        s4 + s12,
        s5 + s13,
        s6 + s14,
        s7 + s15,
    ];
    r + s[0] + s[1] + s[2] + s[3] + s[4] + s[5] + s[6] + s[7]
}

pub fn wide_sum_fold2(values: &[f64]) -> f64 {
    let mut s0 = 0.0;
    let mut s1 = 0.0;
    let mut s2 = 0.0;
    let mut s3 = 0.0;
    let mut s4 = 0.0;
    let mut s5 = 0.0;
    let mut s6 = 0.0;
    let mut s7 = 0.0;
    let mut s8 = 0.0;
    let mut s9 = 0.0;
    let mut s10 = 0.0;
    let mut s11 = 0.0;
    let mut s12 = 0.0;
    let mut s13 = 0.0;
    let mut s14 = 0.0;
    let mut s15 = 0.0;

    let (chunks, remainder) = values.as_chunks::<16>();
    for &[
        c0,
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
        c8,
        c9,
        c10,
        c11,
        c12,
        c13,
        c14,
        c15,
    ] in chunks
    {
        s0 += c0;
        s1 += c1;
        s2 += c2;
        s3 += c3;
        s4 += c4;
        s5 += c5;
        s6 += c6;
        s7 += c7;
        s8 += c8;
        s9 += c9;
        s10 += c10;
        s11 += c11;
        s12 += c12;
        s13 += c13;
        s14 += c14;
        s15 += c15;
    }

    let r = remainder.iter().sum::<f64>();

    let s = [
        s0 + s8,
        s1 + s9,
        s2 + s10,
        s3 + s11,
        s4 + s12,
        s5 + s13,
        s6 + s14,
        s7 + s15,
    ];
    let s = [s[0] + s[4], s[1] + s[5], s[2] + s[6], s[3] + s[7]];
    r + s[0] + s[1] + s[2] + s[3]
}

pub fn expanded_fold_sum(values: &[f64]) -> f64 {
    let len = values.len();
    if len < BLOCK * BLOCK / 2 {
        return chunked_sum(values);
    }

    let capacity_current = (len + BLOCK - 1) / BLOCK;
    let mut buffer_current = Vec::with_capacity(capacity_current);
    let capacity_next = (capacity_current + BLOCK - 1) / BLOCK;
    let mut buffer_next = Vec::with_capacity(capacity_next);

    let (chunks, remainder) = values.as_chunks::<BLOCK>();
    buffer_current.extend(chunks.iter().map(|chunk| expanded_sum_512_to_1(chunk)));

    if !remainder.is_empty() {
        buffer_current.push(chunked_sum(remainder));
    }

    while buffer_current.len() >= BLOCK * BLOCK / 2 {
        buffer_next.clear();

        let (chunks, remainder) = buffer_current.as_chunks::<BLOCK>();
        buffer_next.extend(chunks.iter().map(|chunk| expanded_sum_512_to_1(chunk)));

        if !remainder.is_empty() {
            buffer_next.push(chunked_sum(remainder));
        }

        std::mem::swap(&mut buffer_current, &mut buffer_next);
    }

    chunked_sum(&buffer_current)
}

#[inline]
fn expanded_sum_512_to_1(values: &[f64; 512]) -> f64 {
    let values = expanded_reduce_512_to_8(values);
    sum_8_to_1(&values)
}

fn expanded_reduce_512_to_8(values: &[f64; 512]) -> [f64; 8] {
    let values = {
        let mut s0 = 0.0;
        let mut s1 = 0.0;
        let mut s2 = 0.0;
        let mut s3 = 0.0;
        let mut s4 = 0.0;
        let mut s5 = 0.0;
        let mut s6 = 0.0;
        let mut s7 = 0.0;
        let mut s8 = 0.0;
        let mut s9 = 0.0;
        let mut s10 = 0.0;
        let mut s11 = 0.0;
        let mut s12 = 0.0;
        let mut s13 = 0.0;
        let mut s14 = 0.0;
        let mut s15 = 0.0;
        let mut s16 = 0.0;
        let mut s17 = 0.0;
        let mut s18 = 0.0;
        let mut s19 = 0.0;
        let mut s20 = 0.0;
        let mut s21 = 0.0;
        let mut s22 = 0.0;
        let mut s23 = 0.0;
        let mut s24 = 0.0;
        let mut s25 = 0.0;
        let mut s26 = 0.0;
        let mut s27 = 0.0;
        let mut s28 = 0.0;
        let mut s29 = 0.0;
        let mut s30 = 0.0;
        let mut s31 = 0.0;
        let mut s32 = 0.0;
        let mut s33 = 0.0;
        let mut s34 = 0.0;
        let mut s35 = 0.0;
        let mut s36 = 0.0;
        let mut s37 = 0.0;
        let mut s38 = 0.0;
        let mut s39 = 0.0;
        let mut s40 = 0.0;
        let mut s41 = 0.0;
        let mut s42 = 0.0;
        let mut s43 = 0.0;
        let mut s44 = 0.0;
        let mut s45 = 0.0;
        let mut s46 = 0.0;
        let mut s47 = 0.0;
        let mut s48 = 0.0;
        let mut s49 = 0.0;
        let mut s50 = 0.0;
        let mut s51 = 0.0;
        let mut s52 = 0.0;
        let mut s53 = 0.0;
        let mut s54 = 0.0;
        let mut s55 = 0.0;
        let mut s56 = 0.0;
        let mut s57 = 0.0;
        let mut s58 = 0.0;
        let mut s59 = 0.0;
        let mut s60 = 0.0;
        let mut s61 = 0.0;
        let mut s62 = 0.0;
        let mut s63 = 0.0;
        let mut s64 = 0.0;
        let mut s65 = 0.0;
        let mut s66 = 0.0;
        let mut s67 = 0.0;
        let mut s68 = 0.0;
        let mut s69 = 0.0;
        let mut s70 = 0.0;
        let mut s71 = 0.0;
        let mut s72 = 0.0;
        let mut s73 = 0.0;
        let mut s74 = 0.0;
        let mut s75 = 0.0;
        let mut s76 = 0.0;
        let mut s77 = 0.0;
        let mut s78 = 0.0;
        let mut s79 = 0.0;
        let mut s80 = 0.0;
        let mut s81 = 0.0;
        let mut s82 = 0.0;
        let mut s83 = 0.0;
        let mut s84 = 0.0;
        let mut s85 = 0.0;
        let mut s86 = 0.0;
        let mut s87 = 0.0;
        let mut s88 = 0.0;
        let mut s89 = 0.0;
        let mut s90 = 0.0;
        let mut s91 = 0.0;
        let mut s92 = 0.0;
        let mut s93 = 0.0;
        let mut s94 = 0.0;
        let mut s95 = 0.0;
        let mut s96 = 0.0;
        let mut s97 = 0.0;
        let mut s98 = 0.0;
        let mut s99 = 0.0;
        let mut s100 = 0.0;
        let mut s101 = 0.0;
        let mut s102 = 0.0;
        let mut s103 = 0.0;
        let mut s104 = 0.0;
        let mut s105 = 0.0;
        let mut s106 = 0.0;
        let mut s107 = 0.0;
        let mut s108 = 0.0;
        let mut s109 = 0.0;
        let mut s110 = 0.0;
        let mut s111 = 0.0;
        let mut s112 = 0.0;
        let mut s113 = 0.0;
        let mut s114 = 0.0;
        let mut s115 = 0.0;
        let mut s116 = 0.0;
        let mut s117 = 0.0;
        let mut s118 = 0.0;
        let mut s119 = 0.0;
        let mut s120 = 0.0;
        let mut s121 = 0.0;
        let mut s122 = 0.0;
        let mut s123 = 0.0;
        let mut s124 = 0.0;
        let mut s125 = 0.0;
        let mut s126 = 0.0;
        let mut s127 = 0.0;

        let (chunks, _remainder) = values.as_chunks::<128>();
        for &[
            c0,
            c1,
            c2,
            c3,
            c4,
            c5,
            c6,
            c7,
            c8,
            c9,
            c10,
            c11,
            c12,
            c13,
            c14,
            c15,
            c16,
            c17,
            c18,
            c19,
            c20,
            c21,
            c22,
            c23,
            c24,
            c25,
            c26,
            c27,
            c28,
            c29,
            c30,
            c31,
            c32,
            c33,
            c34,
            c35,
            c36,
            c37,
            c38,
            c39,
            c40,
            c41,
            c42,
            c43,
            c44,
            c45,
            c46,
            c47,
            c48,
            c49,
            c50,
            c51,
            c52,
            c53,
            c54,
            c55,
            c56,
            c57,
            c58,
            c59,
            c60,
            c61,
            c62,
            c63,
            c64,
            c65,
            c66,
            c67,
            c68,
            c69,
            c70,
            c71,
            c72,
            c73,
            c74,
            c75,
            c76,
            c77,
            c78,
            c79,
            c80,
            c81,
            c82,
            c83,
            c84,
            c85,
            c86,
            c87,
            c88,
            c89,
            c90,
            c91,
            c92,
            c93,
            c94,
            c95,
            c96,
            c97,
            c98,
            c99,
            c100,
            c101,
            c102,
            c103,
            c104,
            c105,
            c106,
            c107,
            c108,
            c109,
            c110,
            c111,
            c112,
            c113,
            c114,
            c115,
            c116,
            c117,
            c118,
            c119,
            c120,
            c121,
            c122,
            c123,
            c124,
            c125,
            c126,
            c127,
        ] in chunks
        {
            s0 += c0;
            s1 += c1;
            s2 += c2;
            s3 += c3;
            s4 += c4;
            s5 += c5;
            s6 += c6;
            s7 += c7;
            s8 += c8;
            s9 += c9;
            s10 += c10;
            s11 += c11;
            s12 += c12;
            s13 += c13;
            s14 += c14;
            s15 += c15;
            s16 += c16;
            s17 += c17;
            s18 += c18;
            s19 += c19;
            s20 += c20;
            s21 += c21;
            s22 += c22;
            s23 += c23;
            s24 += c24;
            s25 += c25;
            s26 += c26;
            s27 += c27;
            s28 += c28;
            s29 += c29;
            s30 += c30;
            s31 += c31;
            s32 += c32;
            s33 += c33;
            s34 += c34;
            s35 += c35;
            s36 += c36;
            s37 += c37;
            s38 += c38;
            s39 += c39;
            s40 += c40;
            s41 += c41;
            s42 += c42;
            s43 += c43;
            s44 += c44;
            s45 += c45;
            s46 += c46;
            s47 += c47;
            s48 += c48;
            s49 += c49;
            s50 += c50;
            s51 += c51;
            s52 += c52;
            s53 += c53;
            s54 += c54;
            s55 += c55;
            s56 += c56;
            s57 += c57;
            s58 += c58;
            s59 += c59;
            s60 += c60;
            s61 += c61;
            s62 += c62;
            s63 += c63;
            s64 += c64;
            s65 += c65;
            s66 += c66;
            s67 += c67;
            s68 += c68;
            s69 += c69;
            s70 += c70;
            s71 += c71;
            s72 += c72;
            s73 += c73;
            s74 += c74;
            s75 += c75;
            s76 += c76;
            s77 += c77;
            s78 += c78;
            s79 += c79;
            s80 += c80;
            s81 += c81;
            s82 += c82;
            s83 += c83;
            s84 += c84;
            s85 += c85;
            s86 += c86;
            s87 += c87;
            s88 += c88;
            s89 += c89;
            s90 += c90;
            s91 += c91;
            s92 += c92;
            s93 += c93;
            s94 += c94;
            s95 += c95;
            s96 += c96;
            s97 += c97;
            s98 += c98;
            s99 += c99;
            s100 += c100;
            s101 += c101;
            s102 += c102;
            s103 += c103;
            s104 += c104;
            s105 += c105;
            s106 += c106;
            s107 += c107;
            s108 += c108;
            s109 += c109;
            s110 += c110;
            s111 += c111;
            s112 += c112;
            s113 += c113;
            s114 += c114;
            s115 += c115;
            s116 += c116;
            s117 += c117;
            s118 += c118;
            s119 += c119;
            s120 += c120;
            s121 += c121;
            s122 += c122;
            s123 += c123;
            s124 += c124;
            s125 += c125;
            s126 += c126;
            s127 += c127;
        }

        [
            s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16, s17, s18,
            s19, s20, s21, s22, s23, s24, s25, s26, s27, s28, s29, s30, s31, s32, s33, s34, s35,
            s36, s37, s38, s39, s40, s41, s42, s43, s44, s45, s46, s47, s48, s49, s50, s51, s52,
            s53, s54, s55, s56, s57, s58, s59, s60, s61, s62, s63, s64, s65, s66, s67, s68, s69,
            s70, s71, s72, s73, s74, s75, s76, s77, s78, s79, s80, s81, s82, s83, s84, s85, s86,
            s87, s88, s89, s90, s91, s92, s93, s94, s95, s96, s97, s98, s99, s100, s101, s102,
            s103, s104, s105, s106, s107, s108, s109, s110, s111, s112, s113, s114, s115, s116,
            s117, s118, s119, s120, s121, s122, s123, s124, s125, s126, s127,
        ]
    };

    let values = {
        let mut s0 = 0.0;
        let mut s1 = 0.0;
        let mut s2 = 0.0;
        let mut s3 = 0.0;
        let mut s4 = 0.0;
        let mut s5 = 0.0;
        let mut s6 = 0.0;
        let mut s7 = 0.0;
        let mut s8 = 0.0;
        let mut s9 = 0.0;
        let mut s10 = 0.0;
        let mut s11 = 0.0;
        let mut s12 = 0.0;
        let mut s13 = 0.0;
        let mut s14 = 0.0;
        let mut s15 = 0.0;
        let mut s16 = 0.0;
        let mut s17 = 0.0;
        let mut s18 = 0.0;
        let mut s19 = 0.0;
        let mut s20 = 0.0;
        let mut s21 = 0.0;
        let mut s22 = 0.0;
        let mut s23 = 0.0;
        let mut s24 = 0.0;
        let mut s25 = 0.0;
        let mut s26 = 0.0;
        let mut s27 = 0.0;
        let mut s28 = 0.0;
        let mut s29 = 0.0;
        let mut s30 = 0.0;
        let mut s31 = 0.0;

        let (chunks, _remainder) = values.as_chunks::<32>();
        for &[
            c0,
            c1,
            c2,
            c3,
            c4,
            c5,
            c6,
            c7,
            c8,
            c9,
            c10,
            c11,
            c12,
            c13,
            c14,
            c15,
            c16,
            c17,
            c18,
            c19,
            c20,
            c21,
            c22,
            c23,
            c24,
            c25,
            c26,
            c27,
            c28,
            c29,
            c30,
            c31,
        ] in chunks
        {
            s0 += c0;
            s1 += c1;
            s2 += c2;
            s3 += c3;
            s4 += c4;
            s5 += c5;
            s6 += c6;
            s7 += c7;
            s8 += c8;
            s9 += c9;
            s10 += c10;
            s11 += c11;
            s12 += c12;
            s13 += c13;
            s14 += c14;
            s15 += c15;
            s16 += c16;
            s17 += c17;
            s18 += c18;
            s19 += c19;
            s20 += c20;
            s21 += c21;
            s22 += c22;
            s23 += c23;
            s24 += c24;
            s25 += c25;
            s26 += c26;
            s27 += c27;
            s28 += c28;
            s29 += c29;
            s30 += c30;
            s31 += c31;
        }

        [
            s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16, s17, s18,
            s19, s20, s21, s22, s23, s24, s25, s26, s27, s28, s29, s30, s31,
        ]
    };

    let values = {
        let mut s0 = 0.0;
        let mut s1 = 0.0;
        let mut s2 = 0.0;
        let mut s3 = 0.0;
        let mut s4 = 0.0;
        let mut s5 = 0.0;
        let mut s6 = 0.0;
        let mut s7 = 0.0;

        let (chunks, _remainder) = values.as_chunks::<8>();
        for &[c0, c1, c2, c3, c4, c5, c6, c7] in chunks {
            s0 += c0;
            s1 += c1;
            s2 += c2;
            s3 += c3;
            s4 += c4;
            s5 += c5;
            s6 += c6;
            s7 += c7;
        }

        [s0, s1, s2, s3, s4, s5, s6, s7]
    };

    values
}
