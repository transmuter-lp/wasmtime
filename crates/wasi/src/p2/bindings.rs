//! Auto-generated bindings for WASI interfaces.
//!
//! This module contains the output of the [`bindgen!`] macro when run over
//! the `wasi:cli/command` world. That means this module has all the generated
//! types for WASI for all of its base interfaces used by the CLI world. This
//! module itself by default contains bindings for `async`-related traits. The
//! [`sync`] module contains bindings for a non-`async` version of types.
//!
//! [`bindgen!`]: https://docs.rs/wasmtime/latest/wasmtime/component/macro.bindgen.html
//!
//! # Examples
//!
//! If you have a WIT world which refers to WASI interfaces you probably want to
//! use this modules's bindings rather than generate fresh bindings. That can be
//! done using the `with` option to [`bindgen!`]:
//!
//! ```rust
//! use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};
//! use wasmtime::{Result, Engine, Config};
//! use wasmtime::component::{Linker, ResourceTable, HasSelf};
//!
//! wasmtime::component::bindgen!({
//!     inline: "
//!         package example:wasi;
//!
//!         // An example of extending the `wasi:cli/command` world with a
//!         // custom host interface.
//!         world my-world {
//!             include wasi:cli/command@0.2.6;
//!
//!             import custom-host;
//!         }
//!
//!         interface custom-host {
//!             my-custom-function: func();
//!         }
//!     ",
//!     path: "src/p2/wit",
//!     with: {
//!         "wasi": wasmtime_wasi::p2::bindings,
//!     },
//!     async: true,
//! });
//!
//! struct MyState {
//!     table: ResourceTable,
//!     ctx: WasiCtx,
//! }
//!
//! impl example::wasi::custom_host::Host for MyState {
//!     async fn my_custom_function(&mut self) {
//!         // ..
//!     }
//! }
//!
//! impl IoView for MyState {
//!     fn table(&mut self) -> &mut ResourceTable { &mut self.table }
//! }
//! impl WasiView for MyState {
//!     fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
//! }
//!
//! fn main() -> Result<()> {
//!     let mut config = Config::default();
//!     config.async_support(true);
//!     let engine = Engine::new(&config)?;
//!     let mut linker: Linker<MyState> = Linker::new(&engine);
//!     wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
//!     example::wasi::custom_host::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
//!
//!     // .. use `Linker` to instantiate component ...
//!
//!     Ok(())
//! }
//! ```

/// Synchronous-generated bindings for WASI interfaces.
///
/// This is the same as the top-level [`bindings`](crate::p2::bindings) submodule of
/// this module except that it's for synchronous calls.
///
/// # Examples
///
/// If you have a WIT world which refers to WASI interfaces you probably want to
/// use this modules's bindings rather than generate fresh bindings. That can be
/// done using the `with` option to `bindgen!`:
///
/// ```rust
/// use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView};
/// use wasmtime::{Result, Engine};
/// use wasmtime::component::{Linker, ResourceTable, HasSelf};
///
/// wasmtime::component::bindgen!({
///     inline: "
///         package example:wasi;
///
///         // An example of extending the `wasi:cli/command` world with a
///         // custom host interface.
///         world my-world {
///             include wasi:cli/command@0.2.6;
///
///             import custom-host;
///         }
///
///         interface custom-host {
///             my-custom-function: func();
///         }
///     ",
///     path: "src/p2/wit",
///     with: {
///         "wasi": wasmtime_wasi::p2::bindings::sync,
///     },
///     // This is required for bindings using `wasmtime-wasi` and it otherwise
///     // isn't the default for non-async bindings.
///     require_store_data_send: true,
/// });
///
/// struct MyState {
///     table: ResourceTable,
///     ctx: WasiCtx,
/// }
///
/// impl example::wasi::custom_host::Host for MyState {
///     fn my_custom_function(&mut self) {
///         // ..
///     }
/// }
///
/// impl IoView for MyState {
///     fn table(&mut self) -> &mut ResourceTable { &mut self.table }
/// }
/// impl WasiView for MyState {
///     fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
/// }
///
/// fn main() -> Result<()> {
///     let engine = Engine::default();
///     let mut linker: Linker<MyState> = Linker::new(&engine);
///     wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
///     example::wasi::custom_host::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;
///
///     // .. use `Linker` to instantiate component ...
///
///     Ok(())
/// }
/// ```
pub mod sync {
    mod generated {
        use crate::p2::{FsError, SocketError};
        use wasmtime_wasi_io::streams::StreamError;

        wasmtime::component::bindgen!({
            path: "src/p2/wit",
            world: "wasi:cli/command",
            tracing: true,
            trappable_error_type: {
                "wasi:io/streams/stream-error" => StreamError,
                "wasi:filesystem/types/error-code" => FsError,
                "wasi:sockets/network/error-code" => SocketError,
            },
            trappable_imports: true,
            with: {
                // These interfaces contain only synchronous methods, so they
                // can be aliased directly
                "wasi:clocks": crate::p2::bindings::clocks,
                "wasi:random": crate::p2::bindings::random,
                "wasi:cli": crate::p2::bindings::cli,
                "wasi:filesystem/preopens": crate::p2::bindings::filesystem::preopens,
                "wasi:sockets/network": crate::p2::bindings::sockets::network,

                // Configure the resource types of the bound interfaces here
                // to be the same as the async versions of the resources, that
                // way everything has the same type.
                "wasi:filesystem/types/descriptor": super::super::filesystem::types::Descriptor,
                "wasi:filesystem/types/directory-entry-stream": super::super::filesystem::types::DirectoryEntryStream,
                "wasi:sockets/tcp/tcp-socket": super::super::sockets::tcp::TcpSocket,
                "wasi:sockets/udp/incoming-datagram-stream": super::super::sockets::udp::IncomingDatagramStream,
                "wasi:sockets/udp/outgoing-datagram-stream": super::super::sockets::udp::OutgoingDatagramStream,
                "wasi:sockets/udp/udp-socket": super::super::sockets::udp::UdpSocket,

                // Error host trait from wasmtime-wasi-io is synchronous, so we can alias it
                "wasi:io/error": wasmtime_wasi_io::bindings::wasi::io::error,
                // Configure the resource types from wasmtime-wasi-io, though
                // this bindgen will make a new synchronous Host traits
                "wasi:io/poll/pollable": wasmtime_wasi_io::poll::DynPollable,
                "wasi:io/streams/input-stream": wasmtime_wasi_io::streams::DynInputStream,
                "wasi:io/streams/output-stream": wasmtime_wasi_io::streams::DynOutputStream,

            },
            require_store_data_send: true,
        });
    }
    pub use self::generated::exports;
    pub use self::generated::wasi::*;

    /// Synchronous bindings to execute and run a `wasi:cli/command`.
    ///
    /// This structure is automatically generated by `bindgen!` and is intended
    /// to be used with [`Config::async_support(false)`][async]. For the
    /// asynchronous version see [`bindings::Command`](super::Command).
    ///
    /// This can be used for a more "typed" view of executing a command
    /// component through the [`Command::wasi_cli_run`] method plus
    /// [`Guest::call_run`](exports::wasi::cli::run::Guest::call_run).
    ///
    /// [async]: wasmtime::Config::async_support
    /// [`wasmtime_wasi::p2::add_to_linker_sync`]: crate::p2::add_to_linker_sync
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wasmtime::{Engine, Result, Store, Config};
    /// use wasmtime::component::{ResourceTable, Linker, Component};
    /// use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView, WasiCtxBuilder};
    /// use wasmtime_wasi::p2::bindings::sync::Command;
    ///
    /// // This example is an example shim of executing a component based on the
    /// // command line arguments provided to this program.
    /// fn main() -> Result<()> {
    ///     let args = std::env::args().skip(1).collect::<Vec<_>>();
    ///
    ///     // Configure and create `Engine`
    ///     let engine = Engine::default();
    ///
    ///     // Configure a `Linker` with WASI, compile a component based on
    ///     // command line arguments.
    ///     let mut linker = Linker::<MyState>::new(&engine);
    ///     wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    ///     let component = Component::from_file(&engine, &args[0])?;
    ///
    ///
    ///     // Configure a `WasiCtx` based on this program's environment. Then
    ///     // build a `Store` to instantiate into.
    ///     let mut builder = WasiCtxBuilder::new();
    ///     builder.inherit_stdio().inherit_env().args(&args[2..]);
    ///     let mut store = Store::new(
    ///         &engine,
    ///         MyState {
    ///             ctx: builder.build(),
    ///             table: ResourceTable::new(),
    ///         },
    ///     );
    ///
    ///     // Instantiate the component and we're off to the races.
    ///     let command = Command::instantiate(&mut store, &component, &linker)?;
    ///     let program_result = command.wasi_cli_run().call_run(&mut store)?;
    ///     match program_result {
    ///         Ok(()) => Ok(()),
    ///         Err(()) => std::process::exit(1),
    ///     }
    /// }
    ///
    /// struct MyState {
    ///     ctx: WasiCtx,
    ///     table: ResourceTable,
    /// }
    ///
    /// impl IoView for MyState {
    ///     fn table(&mut self) -> &mut ResourceTable { &mut self.table }
    /// }
    /// impl WasiView for MyState {
    ///     fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    /// }
    /// ```
    ///
    /// ---
    pub use self::generated::Command;

    /// Pre-instantiated analogue of [`Command`].
    ///
    /// This works the same as [`Command`] but enables front-loading work such
    /// as export lookup to before instantiation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wasmtime::{Engine, Result, Store, Config};
    /// use wasmtime::component::{ResourceTable, Linker, Component};
    /// use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView, WasiCtxBuilder};
    /// use wasmtime_wasi::p2::bindings::sync::CommandPre;
    ///
    /// // This example is an example shim of executing a component based on the
    /// // command line arguments provided to this program.
    /// fn main() -> Result<()> {
    ///     let args = std::env::args().skip(1).collect::<Vec<_>>();
    ///
    ///     // Configure and create `Engine`
    ///     let engine = Engine::default();
    ///
    ///     // Configure a `Linker` with WASI, compile a component based on
    ///     // command line arguments, and then pre-instantiate it.
    ///     let mut linker = Linker::<MyState>::new(&engine);
    ///     wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    ///     let component = Component::from_file(&engine, &args[0])?;
    ///     let pre = CommandPre::new(linker.instantiate_pre(&component)?)?;
    ///
    ///
    ///     // Configure a `WasiCtx` based on this program's environment. Then
    ///     // build a `Store` to instantiate into.
    ///     let mut builder = WasiCtxBuilder::new();
    ///     builder.inherit_stdio().inherit_env().args(&args);
    ///     let mut store = Store::new(
    ///         &engine,
    ///         MyState {
    ///             ctx: builder.build(),
    ///             table: ResourceTable::new(),
    ///         },
    ///     );
    ///
    ///     // Instantiate the component and we're off to the races.
    ///     let command = pre.instantiate(&mut store)?;
    ///     let program_result = command.wasi_cli_run().call_run(&mut store)?;
    ///     match program_result {
    ///         Ok(()) => Ok(()),
    ///         Err(()) => std::process::exit(1),
    ///     }
    /// }
    ///
    /// struct MyState {
    ///     ctx: WasiCtx,
    ///     table: ResourceTable,
    /// }
    ///
    /// impl IoView for MyState {
    ///     fn table(&mut self) -> &mut ResourceTable { &mut self.table }
    /// }
    /// impl WasiView for MyState {
    ///     fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    /// }
    /// ```
    ///
    /// ---
    pub use self::generated::CommandPre;

    pub use self::generated::CommandIndices;

    pub use self::generated::LinkOptions;
}

mod async_io {
    wasmtime::component::bindgen!({
        path: "src/p2/wit",
        world: "wasi:cli/command",
        tracing: true,
        trappable_imports: true,
        async: {
            // Only these functions are `async` and everything else is sync
            // meaning that it basically doesn't need to block. These functions
            // are the only ones that need to block.
            //
            // Note that at this time `only_imports` works on function names
            // which in theory can be shared across interfaces, so this may
            // need fancier syntax in the future.
            only_imports: [
                "[method]descriptor.access-at",
                "[method]descriptor.advise",
                "[method]descriptor.change-directory-permissions-at",
                "[method]descriptor.change-file-permissions-at",
                "[method]descriptor.create-directory-at",
                "[method]descriptor.get-flags",
                "[method]descriptor.get-type",
                "[method]descriptor.is-same-object",
                "[method]descriptor.link-at",
                "[method]descriptor.lock-exclusive",
                "[method]descriptor.lock-shared",
                "[method]descriptor.metadata-hash",
                "[method]descriptor.metadata-hash-at",
                "[method]descriptor.open-at",
                "[method]descriptor.read",
                "[method]descriptor.read-directory",
                "[method]descriptor.readlink-at",
                "[method]descriptor.remove-directory-at",
                "[method]descriptor.rename-at",
                "[method]descriptor.set-size",
                "[method]descriptor.set-times",
                "[method]descriptor.set-times-at",
                "[method]descriptor.stat",
                "[method]descriptor.stat-at",
                "[method]descriptor.symlink-at",
                "[method]descriptor.sync",
                "[method]descriptor.sync-data",
                "[method]descriptor.try-lock-exclusive",
                "[method]descriptor.try-lock-shared",
                "[method]descriptor.unlink-file-at",
                "[method]descriptor.unlock",
                "[method]descriptor.write",
                "[method]input-stream.blocking-read",
                "[method]input-stream.blocking-skip",
                "[drop]input-stream",
                "[method]output-stream.blocking-splice",
                "[method]output-stream.blocking-flush",
                "[method]output-stream.blocking-write",
                "[method]output-stream.blocking-write-and-flush",
                "[method]output-stream.blocking-write-zeroes-and-flush",
                "[drop]output-stream",
                "[method]directory-entry-stream.read-directory-entry",
                "poll",
                "[method]pollable.block",
                "[method]pollable.ready",
                "[method]tcp-socket.start-bind",
                "[method]tcp-socket.start-connect",
                "[method]udp-socket.start-bind",
                "[method]udp-socket.stream",
                "[method]outgoing-datagram-stream.send",
            ],
        },
        trappable_error_type: {
            "wasi:io/streams/stream-error" => wasmtime_wasi_io::streams::StreamError,
            "wasi:filesystem/types/error-code" => crate::p2::FsError,
            "wasi:sockets/network/error-code" => crate::p2::SocketError,
        },
        with: {
            // All interfaces in the wasi:io package should be aliased to
            // the wasmtime-wasi-io generated code. Note that this will also
            // map the resource types to those defined in that crate as well.
            "wasi:io/poll": wasmtime_wasi_io::bindings::wasi::io::poll,
            "wasi:io/streams": wasmtime_wasi_io::bindings::wasi::io::streams,
            "wasi:io/error": wasmtime_wasi_io::bindings::wasi::io::error,

            // Configure all other resources to be concrete types defined in
            // this crate
            "wasi:sockets/network/network": crate::net::Network,
            "wasi:sockets/tcp/tcp-socket": crate::p2::tcp::TcpSocket,
            "wasi:sockets/udp/udp-socket": crate::p2::udp::UdpSocket,
            "wasi:sockets/udp/incoming-datagram-stream": crate::p2::udp::IncomingDatagramStream,
            "wasi:sockets/udp/outgoing-datagram-stream": crate::p2::udp::OutgoingDatagramStream,
            "wasi:sockets/ip-name-lookup/resolve-address-stream": crate::p2::ip_name_lookup::ResolveAddressStream,
            "wasi:filesystem/types/directory-entry-stream": crate::p2::filesystem::ReaddirIterator,
            "wasi:filesystem/types/descriptor": crate::p2::filesystem::Descriptor,
            "wasi:cli/terminal-input/terminal-input": crate::p2::stdio::TerminalInput,
            "wasi:cli/terminal-output/terminal-output": crate::p2::stdio::TerminalOutput,
        },
    });
}

pub use self::async_io::LinkOptions;
pub use self::async_io::exports;
pub use self::async_io::wasi::*;

/// Asynchronous bindings to execute and run a `wasi:cli/command`.
///
/// This structure is automatically generated by `bindgen!` and is intended to
/// be used with [`Config::async_support(true)`][async]. For the synchronous
/// version see [`bindings::sync::Command`](sync::Command).
///
/// This can be used for a more "typed" view of executing a command component
/// through the [`Command::wasi_cli_run`] method plus
/// [`Guest::call_run`](exports::wasi::cli::run::Guest::call_run).
///
/// [async]: wasmtime::Config::async_support
/// [`wasmtime_wasi::p2::add_to_linker_async`]: crate::p2::add_to_linker_async
///
/// # Examples
///
/// ```no_run
/// use wasmtime::{Engine, Result, Store, Config};
/// use wasmtime::component::{ResourceTable, Linker, Component};
/// use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView, WasiCtxBuilder};
/// use wasmtime_wasi::p2::bindings::Command;
///
/// // This example is an example shim of executing a component based on the
/// // command line arguments provided to this program.
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let args = std::env::args().skip(1).collect::<Vec<_>>();
///
///     // Configure and create `Engine`
///     let mut config = Config::new();
///     config.async_support(true);
///     let engine = Engine::new(&config)?;
///
///     // Configure a `Linker` with WASI, compile a component based on
///     // command line arguments, and then pre-instantiate it.
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
///     let component = Component::from_file(&engine, &args[0])?;
///
///
///     // Configure a `WasiCtx` based on this program's environment. Then
///     // build a `Store` to instantiate into.
///     let mut builder = WasiCtxBuilder::new();
///     builder.inherit_stdio().inherit_env().args(&args);
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             ctx: builder.build(),
///             table: ResourceTable::new(),
///         },
///     );
///
///     // Instantiate the component and we're off to the races.
///     let command = Command::instantiate_async(&mut store, &component, &linker).await?;
///     let program_result = command.wasi_cli_run().call_run(&mut store).await?;
///     match program_result {
///         Ok(()) => Ok(()),
///         Err(()) => std::process::exit(1),
///     }
/// }
///
/// struct MyState {
///     ctx: WasiCtx,
///     table: ResourceTable,
/// }
///
/// impl IoView for MyState {
///     fn table(&mut self) -> &mut ResourceTable { &mut self.table }
/// }
/// impl WasiView for MyState {
///     fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
/// }
/// ```
///
/// ---
pub use self::async_io::Command;

/// Pre-instantiated analog of [`Command`]
///
/// This can be used to front-load work such as export lookup before
/// instantiation.
///
/// # Examples
///
/// ```no_run
/// use wasmtime::{Engine, Result, Store, Config};
/// use wasmtime::component::{ResourceTable, Linker, Component};
/// use wasmtime_wasi::p2::{IoView, WasiCtx, WasiView, WasiCtxBuilder};
/// use wasmtime_wasi::p2::bindings::CommandPre;
///
/// // This example is an example shim of executing a component based on the
/// // command line arguments provided to this program.
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let args = std::env::args().skip(1).collect::<Vec<_>>();
///
///     // Configure and create `Engine`
///     let mut config = Config::new();
///     config.async_support(true);
///     let engine = Engine::new(&config)?;
///
///     // Configure a `Linker` with WASI, compile a component based on
///     // command line arguments, and then pre-instantiate it.
///     let mut linker = Linker::<MyState>::new(&engine);
///     wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
///     let component = Component::from_file(&engine, &args[0])?;
///     let pre = CommandPre::new(linker.instantiate_pre(&component)?)?;
///
///
///     // Configure a `WasiCtx` based on this program's environment. Then
///     // build a `Store` to instantiate into.
///     let mut builder = WasiCtxBuilder::new();
///     builder.inherit_stdio().inherit_env().args(&args);
///     let mut store = Store::new(
///         &engine,
///         MyState {
///             ctx: builder.build(),
///             table: ResourceTable::new(),
///         },
///     );
///
///     // Instantiate the component and we're off to the races.
///     let command = pre.instantiate_async(&mut store).await?;
///     let program_result = command.wasi_cli_run().call_run(&mut store).await?;
///     match program_result {
///         Ok(()) => Ok(()),
///         Err(()) => std::process::exit(1),
///     }
/// }
///
/// struct MyState {
///     ctx: WasiCtx,
///     table: ResourceTable,
/// }
///
/// impl IoView for MyState {
///     fn table(&mut self) -> &mut ResourceTable { &mut self.table }
/// }
/// impl WasiView for MyState {
///     fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
/// }
/// ```
///
/// ---
pub use self::async_io::CommandPre;

pub use self::async_io::CommandIndices;
