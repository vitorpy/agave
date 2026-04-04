use {
    crate::instructions_frame::InstructionsIterator,
    core::fmt::{Debug, Formatter},
    solana_svm_transaction::instruction::SVMInstruction,
};

/// Instruction iterator for either legacy/v0 wire layout or decoded tx-v1.
#[derive(Clone)]
pub enum UnifiedInstructionsIter<'a> {
    Wire(InstructionsIterator<'a>),
    V1 {
        message: &'a solana_message::v1::Message,
        index: usize,
    },
}

impl<'a> Iterator for UnifiedInstructionsIter<'a> {
    type Item = SVMInstruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Wire(iter) => iter.next(),
            Self::V1 { message, index } => {
                let ix = message.instructions.get(*index)?;
                *index = index.saturating_add(1);
                Some(SVMInstruction {
                    program_id_index: ix.program_id_index,
                    accounts: ix.accounts.as_slice(),
                    data: ix.data.as_slice(),
                })
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for UnifiedInstructionsIter<'_> {
    fn len(&self) -> usize {
        match self {
            Self::Wire(iter) => iter.len(),
            Self::V1 { message, index } => message.instructions.len().saturating_sub(*index),
        }
    }
}

impl Debug for UnifiedInstructionsIter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Wire(iter) => f.debug_tuple("Wire").field(iter).finish(),
            Self::V1 { message, index } => f
                .debug_struct("V1")
                .field("remaining", &(message.instructions.len().saturating_sub(*index)))
                .finish(),
        }
    }
}
