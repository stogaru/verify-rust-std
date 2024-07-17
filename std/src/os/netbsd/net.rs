//! NetBSD-specific networking functionality.

#![unstable(feature = "unix_socket_ancillary_data", issue = "76915")]

use crate::ffi::CStr;
use crate::io;
use crate::os::unix::net;
use crate::sealed::Sealed;
use crate::sys_common::AsInner;

/// NetBSD-specific functionality for `AF_UNIX` sockets [`UnixDatagram`]
/// and [`UnixStream`].
///
/// [`UnixDatagram`]: net::UnixDatagram
/// [`UnixStream`]: net::UnixStream
#[unstable(feature = "unix_socket_ancillary_data", issue = "76915")]
pub trait UnixSocketExt: Sealed {
    /// Query the current setting of socket option `LOCAL_CREDS`.
    #[unstable(feature = "unix_socket_ancillary_data", issue = "76915")]
    fn local_creds(&self) -> io::Result<bool>;

    /// Enable or disable socket option `LOCAL_CREDS`.
    ///
    /// This option enables the credentials of the sending process to be
    /// received as a control message in [`AncillaryData`].
    ///
    /// [`AncillaryData`]: net::AncillaryData
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(unix_socket_ancillary_data)]
    /// use std::os::netbsd::net::UnixSocketExt;
    /// use std::os::unix::net::UnixDatagram;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let sock = UnixDatagram::unbound()?;
    ///     sock.set_local_creds(true).expect("set_local_creds failed");
    ///     Ok(())
    /// }
    /// ```
    #[unstable(feature = "unix_socket_ancillary_data", issue = "76915")]
    fn set_local_creds(&self, local_creds: bool) -> io::Result<()>;

    /// Get a filter name if one had been set previously on the socket.
    #[unstable(feature = "acceptfilter", issue = "121891")]
    fn acceptfilter(&self) -> io::Result<&CStr>;

    /// Set or disable a filter on the socket to filter incoming connections
    /// to defer it before accept(2)
    #[unstable(feature = "acceptfilter", issue = "121891")]
    fn set_acceptfilter(&self, name: &CStr) -> io::Result<()>;
}

#[unstable(feature = "unix_socket_ancillary_data", issue = "76915")]
impl UnixSocketExt for net::UnixDatagram {
    fn local_creds(&self) -> io::Result<bool> {
        self.as_inner().local_creds()
    }

    fn set_local_creds(&self, local_creds: bool) -> io::Result<()> {
        self.as_inner().set_local_creds(local_creds)
    }

    fn acceptfilter(&self) -> io::Result<&CStr> {
        self.as_inner().acceptfilter()
    }

    fn set_acceptfilter(&self, name: &CStr) -> io::Result<()> {
        self.as_inner().set_acceptfilter(name)
    }
}

#[unstable(feature = "unix_socket_ancillary_data", issue = "76915")]
impl UnixSocketExt for net::UnixStream {
    fn local_creds(&self) -> io::Result<bool> {
        self.as_inner().local_creds()
    }

    fn set_local_creds(&self, local_creds: bool) -> io::Result<()> {
        self.as_inner().set_local_creds(local_creds)
    }

    fn acceptfilter(&self) -> io::Result<&CStr> {
        self.as_inner().acceptfilter()
    }

    fn set_acceptfilter(&self, name: &CStr) -> io::Result<()> {
        self.as_inner().set_acceptfilter(name)
    }
}
