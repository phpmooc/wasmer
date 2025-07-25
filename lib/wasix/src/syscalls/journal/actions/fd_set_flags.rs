use super::*;

impl JournalSyscallPlayer<'_, '_> {
    #[allow(clippy::result_large_err)]
    pub(crate) unsafe fn action_fd_set_flags(
        &mut self,
        fd: Fd,
        flags: Fdflags,
    ) -> Result<(), WasiRuntimeError> {
        tracing::trace!(%fd, ?flags, "Replay journal - FdSetFlags");
        JournalEffector::apply_fd_set_flags(&mut self.ctx, fd, flags)
            .map_err(anyhow_err_to_runtime_err)?;
        Ok(())
    }
}
