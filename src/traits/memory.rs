use crate::prelude::*;

pub mod m_pos {
    use super::Pos;

    pub const GM7: Pos = -21;
    pub const GM6: Pos = -18;
    pub const GM5: Pos = -15;
    pub const GM4: Pos = -12;
    pub const GM3: Pos = -9;
    pub const GM2: Pos = -6;
    pub const GM1: Pos = -3;
    pub const MU: Pos = -2;
    pub const ML: Pos = -1;
    pub const G0: Pos = 0;
    pub const G1: Pos = 3;
}

pub trait Memory: Arith {
    fn read_memory(&mut self) -> anyhow::Result<&mut Self> {
        // load A
        self.copy_word(
            word::A,
            &[(pos::G0 + m_pos::GM2, pos::G0 + m_pos::GM1)],
            pos::VU,
        )?;
        // go forward
        // memory layout: 1 - - T - - Au - - Al - - F
        // traversed gap cells are set to 1
        self.seek(pos::G0)?
            .set_pos(m_pos::G0)?
            .while_cond(
                m_pos::G0,
                |s| {
                    s.is_nonzero(
                        (m_pos::GM2, m_pos::GM1),
                        m_pos::G0,
                        [m_pos::GM4, m_pos::GM3],
                    )
                },
                |s| {
                    s.clear_cell(&[m_pos::G0])?
                        .move_cell(m_pos::GM1, &[m_pos::G0])?
                        .move_cell(m_pos::GM2, &[m_pos::GM1])?
                        .dec_word((m_pos::GM1, m_pos::G0), [m_pos::GM3, m_pos::GM2])?
                        .seek(m_pos::GM4)?
                        .inc_val()?
                        .seek(m_pos::G1)?
                        .set_pos(m_pos::G0)
                },
            )?
            .seek(m_pos::GM4)?
            .inc_val()?;
        // read M
        self.copy_word(
            (m_pos::MU, m_pos::ML),
            &[(m_pos::GM3, m_pos::GM2)],
            m_pos::GM1,
        )?;
        // go backward
        // memory layout: F - - Mu - - Ml - - T - - T
        self.while_(m_pos::GM4, |s| {
            s.clear_cell(&[m_pos::GM4])?
                .move_cell(m_pos::GM3, &[m_pos::GM4])?
                .move_cell(m_pos::GM2, &[m_pos::GM3])?
                .seek(m_pos::GM5)?
                .set_pos(m_pos::GM4)
        })?
        .seek(m_pos::G0)?
        .set_pos(pos::G0 + m_pos::GM1)?;
        // unload M
        self.move_cell(pos::G0 + m_pos::GM4, &[pos::MU])?
            .move_cell(pos::G0 + m_pos::GM3, &[pos::ML])
        // self.move_word((pos::G0 + m_pos::GM4, pos::G0 + m_pos::GM3), &[word::M])
    }

    fn write_memory(&mut self) -> anyhow::Result<&mut Self> {
        // load A and M
        self.copy_word(
            word::M,
            &[(pos::G0 + m_pos::GM4, pos::G0 + m_pos::GM3)],
            pos::VU,
        )?
        .copy_word(
            word::A,
            &[(pos::G0 + m_pos::GM2, pos::G0 + m_pos::GM1)],
            pos::VU,
        )?;
        // go forward
        // memory layout: 1 - - T - - Mu - - Ml - - Au - - Al - - F
        // traversed gap cells are set to 1
        self.seek(pos::G0)?
            .set_pos(m_pos::G0)?
            .while_cond(
                m_pos::G0,
                |s| {
                    s.is_nonzero(
                        (m_pos::GM2, m_pos::GM1),
                        m_pos::G0,
                        [m_pos::GM6, m_pos::GM5],
                    )
                },
                |s| {
                    s.clear_cell(&[m_pos::G0])?
                        .move_cell(m_pos::GM1, &[m_pos::G0])?
                        .move_cell(m_pos::GM2, &[m_pos::GM1])?
                        .move_cell(m_pos::GM3, &[m_pos::GM2])?
                        .move_cell(m_pos::GM4, &[m_pos::GM3])?
                        .dec_word((m_pos::GM1, m_pos::G0), [m_pos::GM5, m_pos::GM4])?
                        .seek(m_pos::GM6)?
                        .inc_val()?
                        .seek(m_pos::G1)?
                        .set_pos(m_pos::G0)
                },
            )?
            .seek(m_pos::GM6)?
            .inc_val()?;
        // write M
        self.clear_cell(&[m_pos::MU, m_pos::ML])?
            .move_word((m_pos::GM4, m_pos::GM3), &[(m_pos::MU, m_pos::ML)])?;
        // go backward
        // memory layout: F - - T - - T - - T - - T - - T - - T
        self.while_(m_pos::GM6, |s| {
            s.clear_cell(&[m_pos::GM6])?
                .seek(m_pos::GM7)?
                .set_pos(m_pos::GM6)
        })?
        .seek(m_pos::G0)?
        .set_pos(pos::G0 + m_pos::GM1)
    }
}
impl<T: Arith> Memory for T {}

#[cfg(test)]
mod tests {
    use {super::*, crate::test};

    #[test]
    fn read_memory() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.read_memory()?.seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[
                0, 1, 0, // A
                0, 0, 0, // D
                0, 0, 0, // M
                0, 0, 0, // P
                0, 0, 0, // Q
                0, 0, 0, // R
                0, 0, 0, // F
                0, 0, 0, // V
                0, 0, 0, // W
                1, 0, 0, 1, 1, 0, 1, 2, 0, 1, 3, 0, 1, 4, 0,
            ],
            0,
            &[
                0, 1, 0, // A
                0, 0, 0, // D
                1, 1, 0, // M
                0, 0, 0, // P
                0, 0, 0, // Q
                0, 0, 0, // R
                0, 0, 0, // F
                0, 0, 0, // V
                0, 0, 0, // W
                1, 0, 0, 1, 1, 0, 1, 2, 0, 1, 3, 0, 1, 4, 0,
            ],
            0,
        );

        let au = 2_u8;
        let al = 3_u8;
        let a = usize::from(au) * 256 + usize::from(al);

        let mut initial_tape = vec![0; 26 + 3 * (a + 3)];
        initial_tape[0] = au;
        initial_tape[1] = al;
        for i in 0..(a + 3) {
            initial_tape[27 + 3 * i] = (i / 256).try_into()?;
            initial_tape[28 + 3 * i] = (i % 256).try_into()?;
        }

        let mut final_tape = initial_tape.clone();
        final_tape[6] = au;
        final_tape[7] = al;

        test::compare_tape(coder.writer(), &initial_tape, 0, &final_tape, 0);

        Ok(())
    }

    #[test]
    fn write_memory() -> anyhow::Result<()> {
        let mut coder = Coder::new(vec![]);
        coder.write_memory()?.seek(0)?;

        test::compare_tape(
            coder.writer(),
            &[
                0, 1, 0, // A
                0, 0, 0, // D
                3, 5, 0, // M
                0, 0, 0, // P
                0, 0, 0, // Q
                0, 0, 0, // R
                0, 0, 0, // F
                0, 0, 0, // V
                0, 0, 0, // W
                1, 0, 0, 1, 1, 0, 1, 2, 0, 1, 3, 0, 1, 4, 0,
            ],
            0,
            &[
                0, 1, 0, // A
                0, 0, 0, // D
                3, 5, 0, // M
                0, 0, 0, // P
                0, 0, 0, // Q
                0, 0, 0, // R
                0, 0, 0, // F
                0, 0, 0, // V
                0, 0, 0, // W
                1, 0, 0, 3, 5, 0, 1, 2, 0, 1, 3, 0, 1, 4, 0,
            ],
            0,
        );

        let au = 2_u8;
        let al = 3_u8;
        let a = usize::from(au) * 256 + usize::from(al);

        let mut initial_tape = vec![0; 26 + 3 * (a + 3)];
        initial_tape[0] = au;
        initial_tape[1] = al;
        initial_tape[6] = 5;
        initial_tape[7] = 6;

        let mut final_tape = initial_tape.clone();
        final_tape[27 + 3 * a] = 5;
        final_tape[28 + 3 * a] = 6;

        test::compare_tape(coder.writer(), &initial_tape, 0, &final_tape, 0);

        Ok(())
    }
}
