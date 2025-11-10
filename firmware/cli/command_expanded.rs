pub mod command {
    use embedded_cli::Command;
    pub enum LedCommand {
        /// Get current LED value
        Get,
        /// Set LED value
        Set {
            /// LED brightness
            value: u8,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LedCommand {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                LedCommand::Get => ::core::fmt::Formatter::write_str(f, "Get"),
                LedCommand::Set { value: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Set",
                        "value",
                        &__self_0,
                    )
                }
            }
        }
    }
    const _: () = {
        extern crate embedded_cli as _cli;
        use _cli::__private::io as _io;
        impl _cli::service::Autocomplete for LedCommand {
            fn autocomplete(
                request: _cli::autocomplete::Request<'_>,
                autocompletion: &mut _cli::autocomplete::Autocompletion<'_>,
            ) {
                const NAMES: &[&str; 2usize] = &["get", "set"];
                if let _cli::autocomplete::Request::CommandName(name) = request {
                    NAMES
                        .iter()
                        .skip_while(|n| !n.starts_with(name))
                        .take_while(|n| n.starts_with(name))
                        .for_each(|n| {
                            let autocompleted = unsafe { n.get_unchecked(name.len()..) };
                            autocompletion.merge_autocompletion(autocompleted)
                        });
                }
            }
        }
        impl _cli::service::Help for LedCommand {
            fn command_count() -> usize {
                2usize
            }
            fn list_commands<W: _io::Write<Error = E>, E: _io::Error>(
                writer: &mut _cli::writer::Writer<'_, W, E>,
            ) -> Result<(), E> {
                writer.write_title("Commands:")?;
                writer.writeln_str("")?;
                writer.write_list_element("get", "Get current LED value", 3usize)?;
                writer.write_list_element("set", "Set LED value", 3usize)?;
                Ok(())
            }
            fn command_help<
                W: _io::Write<Error = E>,
                E: _io::Error,
                F: FnMut(&mut _cli::writer::Writer<'_, W, E>) -> Result<(), E>,
            >(
                parent: &mut F,
                command: _cli::command::RawCommand<'_>,
                writer: &mut _cli::writer::Writer<'_, W, E>,
            ) -> Result<(), _cli::service::HelpError<E>> {
                match command.name() {
                    "get" => {
                        writer.writeln_str("Get current LED value")?;
                        writer.writeln_str("")?;
                        writer.write_title("Usage:")?;
                        writer.write_str(" ")?;
                        parent(writer)?;
                        writer.write_str("get")?;
                        writer.writeln_str("")?;
                        writer.writeln_str("")?;
                        writer.write_title("Options:")?;
                        writer.writeln_str("")?;
                        writer.write_list_element("-h, --help", "Print help", 10usize)?;
                    }
                    "set" => {
                        writer.writeln_str("Set LED value")?;
                        writer.writeln_str("")?;
                        writer.write_title("Usage:")?;
                        writer.write_str(" ")?;
                        parent(writer)?;
                        writer.write_str("set")?;
                        writer.write_str(" ")?;
                        writer.write_str("<VALUE>")?;
                        writer.writeln_str("")?;
                        writer.writeln_str("")?;
                        writer.write_title("Arguments:\n")?;
                        writer.write_list_element("<VALUE>", "LED brightness", 7usize)?;
                        writer.writeln_str("")?;
                        writer.write_title("Options:")?;
                        writer.writeln_str("")?;
                        writer.write_list_element("-h, --help", "Print help", 10usize)?;
                    }
                    _ => return Err(_cli::service::HelpError::UnknownCommand),
                }
                Ok(())
            }
        }
        impl<'a> _cli::service::FromRaw<'a> for LedCommand {
            fn parse(
                command: _cli::command::RawCommand<'a>,
            ) -> Result<Self, _cli::service::ParseError<'a>> {
                let command = match command.name() {
                    "get" => LedCommand::Get,
                    "set" => {
                        let mut arg_value = None;
                        enum States {
                            Normal,
                        }
                        #[automatically_derived]
                        impl ::core::cmp::Eq for States {
                            #[inline]
                            #[doc(hidden)]
                            #[coverage(off)]
                            fn assert_receiver_is_total_eq(&self) -> () {}
                        }
                        #[automatically_derived]
                        impl ::core::marker::StructuralPartialEq for States {}
                        #[automatically_derived]
                        impl ::core::cmp::PartialEq for States {
                            #[inline]
                            fn eq(&self, other: &States) -> bool {
                                true
                            }
                        }
                        let mut state = States::Normal;
                        let mut positional = 0;
                        let mut args = command.args().args();
                        while let Some(arg) = args.next() {
                            let arg = arg
                                .map_err(|err| match err {
                                    _cli::arguments::ArgError::NonAsciiShortOption => {
                                        _cli::service::ParseError::NonAsciiShortOption
                                    }
                                })?;
                            match arg {
                                _cli::arguments::Arg::Value(
                                    val,
                                ) if state == States::Normal => {
                                    match positional {
                                        0usize => {
                                            arg_value = Some(
                                                <u8 as _cli::arguments::FromArgument>::from_arg(val)?,
                                            );
                                        }
                                        _ => {
                                            return Err(_cli::service::ParseError::UnexpectedArgument {
                                                value: val,
                                            });
                                        }
                                    }
                                    positional += 1;
                                }
                                _cli::arguments::Arg::Value(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _cli::arguments::Arg::LongOption(option) => {
                                    return Err(_cli::service::ParseError::UnexpectedLongOption {
                                        name: option,
                                    });
                                }
                                _cli::arguments::Arg::ShortOption(option) => {
                                    return Err(_cli::service::ParseError::UnexpectedShortOption {
                                        name: option,
                                    });
                                }
                                _cli::arguments::Arg::DoubleDash => {}
                            }
                        }
                        LedCommand::Set {
                            value: arg_value
                                .ok_or(_cli::service::ParseError::MissingRequiredArgument {
                                    name: "<VALUE>",
                                })?,
                        }
                    }
                    cmd => return Err(_cli::service::ParseError::UnknownCommand),
                };
                Ok(command)
            }
        }
        impl LedCommand {
            fn processor<
                W: _io::Write<Error = E>,
                E: _io::Error,
                F: FnMut(
                        &mut _cli::cli::CliHandle<'_, W, E>,
                        LedCommand,
                    ) -> Result<(), E>,
            >(f: F) -> impl _cli::service::CommandProcessor<W, E> {
                struct Processor<
                    W: _io::Write<Error = E>,
                    E: _io::Error,
                    F: FnMut(
                            &mut _cli::cli::CliHandle<'_, W, E>,
                            LedCommand,
                        ) -> Result<(), E>,
                > {
                    f: F,
                    _ph: core::marker::PhantomData<(W, E)>,
                }
                impl<
                    W: _io::Write<Error = E>,
                    E: _io::Error,
                    F: FnMut(
                            &mut _cli::cli::CliHandle<'_, W, E>,
                            LedCommand,
                        ) -> Result<(), E>,
                > _cli::service::CommandProcessor<W, E> for Processor<W, E, F> {
                    fn process<'a>(
                        &mut self,
                        cli: &mut _cli::cli::CliHandle<'_, W, E>,
                        raw: _cli::command::RawCommand<'a>,
                    ) -> Result<(), _cli::service::ProcessError<'a, E>> {
                        let cmd = <LedCommand as _cli::service::FromRaw>::parse(raw)?;
                        (self.f)(cli, cmd)?;
                        Ok(())
                    }
                }
                Processor {
                    f,
                    _ph: core::marker::PhantomData,
                }
            }
        }
    };
    pub enum AdcCommand<'a> {
        /// Read ADC value
        Read {
            /// Print extra info
            #[arg(short = 'V', long)]
            verbose: bool,
            /// Sample count (16 by default)
            #[arg(long)]
            samples: Option<u8>,
            #[arg(long)]
            sampler: &'a str,
        },
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for AdcCommand<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AdcCommand::Read {
                    verbose: __self_0,
                    samples: __self_1,
                    sampler: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Read",
                        "verbose",
                        __self_0,
                        "samples",
                        __self_1,
                        "sampler",
                        &__self_2,
                    )
                }
            }
        }
    }
    const _: () = {
        extern crate embedded_cli as _cli;
        use _cli::__private::io as _io;
        impl<'a> _cli::service::Autocomplete for AdcCommand<'a> {
            fn autocomplete(
                request: _cli::autocomplete::Request<'_>,
                autocompletion: &mut _cli::autocomplete::Autocompletion<'_>,
            ) {
                const NAMES: &[&str; 1usize] = &["read"];
                if let _cli::autocomplete::Request::CommandName(name) = request {
                    NAMES
                        .iter()
                        .skip_while(|n| !n.starts_with(name))
                        .take_while(|n| n.starts_with(name))
                        .for_each(|n| {
                            let autocompleted = unsafe { n.get_unchecked(name.len()..) };
                            autocompletion.merge_autocompletion(autocompleted)
                        });
                }
            }
        }
        impl<'a> _cli::service::Help for AdcCommand<'a> {
            fn command_count() -> usize {
                1usize
            }
            fn list_commands<W: _io::Write<Error = E>, E: _io::Error>(
                writer: &mut _cli::writer::Writer<'_, W, E>,
            ) -> Result<(), E> {
                writer.write_title("Commands:")?;
                writer.writeln_str("")?;
                writer.write_list_element("read", "Read ADC value", 4usize)?;
                Ok(())
            }
            fn command_help<
                W: _io::Write<Error = E>,
                E: _io::Error,
                F: FnMut(&mut _cli::writer::Writer<'_, W, E>) -> Result<(), E>,
            >(
                parent: &mut F,
                command: _cli::command::RawCommand<'_>,
                writer: &mut _cli::writer::Writer<'_, W, E>,
            ) -> Result<(), _cli::service::HelpError<E>> {
                match command.name() {
                    "read" => {
                        writer.writeln_str("Read ADC value")?;
                        writer.writeln_str("")?;
                        writer.write_title("Usage:")?;
                        writer.write_str(" ")?;
                        parent(writer)?;
                        writer.write_str("read")?;
                        writer.write_str(" [OPTIONS]")?;
                        writer.writeln_str("")?;
                        writer.writeln_str("")?;
                        writer.write_title("Options:")?;
                        writer.writeln_str("")?;
                        writer
                            .write_list_element(
                                "-V, --verbose",
                                "Print extra info",
                                19usize,
                            )?;
                        writer
                            .write_list_element(
                                "--samples [SAMPLES]",
                                "Sample count (16 by default)",
                                19usize,
                            )?;
                        writer.write_list_element("--sampler <SAMPLER>", "", 19usize)?;
                        writer.write_list_element("-h, --help", "Print help", 19usize)?;
                    }
                    _ => return Err(_cli::service::HelpError::UnknownCommand),
                }
                Ok(())
            }
        }
        impl<'a> _cli::service::FromRaw<'a> for AdcCommand<'a> {
            fn parse(
                command: _cli::command::RawCommand<'a>,
            ) -> Result<Self, _cli::service::ParseError<'a>> {
                let command = match command.name() {
                    "read" => {
                        let mut arg_verbose = None;
                        let mut arg_samples = None;
                        let mut arg_sampler = None;
                        enum States {
                            Normal,
                            ExpectSamples,
                            ExpectSampler,
                        }
                        #[automatically_derived]
                        impl ::core::cmp::Eq for States {
                            #[inline]
                            #[doc(hidden)]
                            #[coverage(off)]
                            fn assert_receiver_is_total_eq(&self) -> () {}
                        }
                        #[automatically_derived]
                        impl ::core::marker::StructuralPartialEq for States {}
                        #[automatically_derived]
                        impl ::core::cmp::PartialEq for States {
                            #[inline]
                            fn eq(&self, other: &States) -> bool {
                                let __self_discr = ::core::intrinsics::discriminant_value(
                                    self,
                                );
                                let __arg1_discr = ::core::intrinsics::discriminant_value(
                                    other,
                                );
                                __self_discr == __arg1_discr
                            }
                        }
                        let mut state = States::Normal;
                        let mut positional = 0;
                        let mut args = command.args().args();
                        while let Some(arg) = args.next() {
                            let arg = arg
                                .map_err(|err| match err {
                                    _cli::arguments::ArgError::NonAsciiShortOption => {
                                        _cli::service::ParseError::NonAsciiShortOption
                                    }
                                })?;
                            match arg {
                                _cli::arguments::Arg::LongOption("verbose")
                                | _cli::arguments::Arg::ShortOption('V') => {
                                    arg_verbose = Some(true);
                                    state = States::Normal;
                                }
                                _cli::arguments::Arg::LongOption("samples") => {
                                    state = States::ExpectSamples;
                                }
                                _cli::arguments::Arg::LongOption("sampler") => {
                                    state = States::ExpectSampler;
                                }
                                _cli::arguments::Arg::Value(
                                    val,
                                ) if state == States::ExpectSamples => {
                                    arg_samples = Some(
                                        <u8 as _cli::arguments::FromArgument>::from_arg(val)?,
                                    );
                                    state = States::Normal;
                                }
                                _cli::arguments::Arg::Value(
                                    val,
                                ) if state == States::ExpectSampler => {
                                    arg_sampler = Some(
                                        <&'a str as _cli::arguments::FromArgument>::from_arg(val)?,
                                    );
                                    state = States::Normal;
                                }
                                _cli::arguments::Arg::Value(
                                    value,
                                ) if state == States::Normal => {
                                    return Err(_cli::service::ParseError::UnexpectedArgument {
                                        value,
                                    });
                                }
                                _cli::arguments::Arg::Value(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _cli::arguments::Arg::LongOption(option) => {
                                    return Err(_cli::service::ParseError::UnexpectedLongOption {
                                        name: option,
                                    });
                                }
                                _cli::arguments::Arg::ShortOption(option) => {
                                    return Err(_cli::service::ParseError::UnexpectedShortOption {
                                        name: option,
                                    });
                                }
                                _cli::arguments::Arg::DoubleDash => {}
                            }
                        }
                        AdcCommand::Read {
                            verbose: arg_verbose.unwrap_or(false),
                            samples: arg_samples,
                            sampler: arg_sampler
                                .ok_or(_cli::service::ParseError::MissingRequiredArgument {
                                    name: "--sampler <SAMPLER>",
                                })?,
                        }
                    }
                    cmd => return Err(_cli::service::ParseError::UnknownCommand),
                };
                Ok(command)
            }
        }
        impl<'a> AdcCommand<'a> {
            fn processor<
                W: _io::Write<Error = E>,
                E: _io::Error,
                F: FnMut(
                        &mut _cli::cli::CliHandle<'_, W, E>,
                        AdcCommand<'_>,
                    ) -> Result<(), E>,
            >(f: F) -> impl _cli::service::CommandProcessor<W, E> {
                struct Processor<
                    W: _io::Write<Error = E>,
                    E: _io::Error,
                    F: FnMut(
                            &mut _cli::cli::CliHandle<'_, W, E>,
                            AdcCommand<'_>,
                        ) -> Result<(), E>,
                > {
                    f: F,
                    _ph: core::marker::PhantomData<(W, E)>,
                }
                impl<
                    W: _io::Write<Error = E>,
                    E: _io::Error,
                    F: FnMut(
                            &mut _cli::cli::CliHandle<'_, W, E>,
                            AdcCommand<'_>,
                        ) -> Result<(), E>,
                > _cli::service::CommandProcessor<W, E> for Processor<W, E, F> {
                    fn process<'a>(
                        &mut self,
                        cli: &mut _cli::cli::CliHandle<'_, W, E>,
                        raw: _cli::command::RawCommand<'a>,
                    ) -> Result<(), _cli::service::ProcessError<'a, E>> {
                        let cmd = <AdcCommand<
                            '_,
                        > as _cli::service::FromRaw>::parse(raw)?;
                        (self.f)(cli, cmd)?;
                        Ok(())
                    }
                }
                Processor {
                    f,
                    _ph: core::marker::PhantomData,
                }
            }
        }
    };
    pub enum BaseCommand<'a> {
        /// Control LEDs
        Led {
            /// LED id
            #[arg(long)]
            id: u8,
            #[command(subcommand)]
            command: LedCommand,
        },
        /// Control ADC
        Adc {
            /// ADC id
            #[arg(long)]
            id: u8,
            #[command(subcommand)]
            command: AdcCommand<'a>,
        },
        /// Show some status
        Status,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for BaseCommand<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                BaseCommand::Led { id: __self_0, command: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Led",
                        "id",
                        __self_0,
                        "command",
                        &__self_1,
                    )
                }
                BaseCommand::Adc { id: __self_0, command: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Adc",
                        "id",
                        __self_0,
                        "command",
                        &__self_1,
                    )
                }
                BaseCommand::Status => ::core::fmt::Formatter::write_str(f, "Status"),
            }
        }
    }
    const _: () = {
        extern crate embedded_cli as _cli;
        use _cli::__private::io as _io;
        impl<'a> _cli::service::Autocomplete for BaseCommand<'a> {
            fn autocomplete(
                request: _cli::autocomplete::Request<'_>,
                autocompletion: &mut _cli::autocomplete::Autocompletion<'_>,
            ) {
                const NAMES: &[&str; 3usize] = &["led", "adc", "status"];
                if let _cli::autocomplete::Request::CommandName(name) = request {
                    NAMES
                        .iter()
                        .skip_while(|n| !n.starts_with(name))
                        .take_while(|n| n.starts_with(name))
                        .for_each(|n| {
                            let autocompleted = unsafe { n.get_unchecked(name.len()..) };
                            autocompletion.merge_autocompletion(autocompleted)
                        });
                }
            }
        }
        impl<'a> _cli::service::Help for BaseCommand<'a> {
            fn command_count() -> usize {
                3usize
            }
            fn list_commands<W: _io::Write<Error = E>, E: _io::Error>(
                writer: &mut _cli::writer::Writer<'_, W, E>,
            ) -> Result<(), E> {
                writer.write_title("Commands:")?;
                writer.writeln_str("")?;
                writer.write_list_element("led", "Control LEDs", 6usize)?;
                writer.write_list_element("adc", "Control ADC", 6usize)?;
                writer.write_list_element("status", "Show some status", 6usize)?;
                Ok(())
            }
            fn command_help<
                W: _io::Write<Error = E>,
                E: _io::Error,
                F: FnMut(&mut _cli::writer::Writer<'_, W, E>) -> Result<(), E>,
            >(
                parent: &mut F,
                command: _cli::command::RawCommand<'_>,
                writer: &mut _cli::writer::Writer<'_, W, E>,
            ) -> Result<(), _cli::service::HelpError<E>> {
                match command.name() {
                    "led" => {
                        enum States {
                            Normal,
                            ExpectId,
                        }
                        #[automatically_derived]
                        impl ::core::cmp::Eq for States {
                            #[inline]
                            #[doc(hidden)]
                            #[coverage(off)]
                            fn assert_receiver_is_total_eq(&self) -> () {}
                        }
                        #[automatically_derived]
                        impl ::core::marker::StructuralPartialEq for States {}
                        #[automatically_derived]
                        impl ::core::cmp::PartialEq for States {
                            #[inline]
                            fn eq(&self, other: &States) -> bool {
                                let __self_discr = ::core::intrinsics::discriminant_value(
                                    self,
                                );
                                let __arg1_discr = ::core::intrinsics::discriminant_value(
                                    other,
                                );
                                __self_discr == __arg1_discr
                            }
                        }
                        let mut state = States::Normal;
                        let mut args = command.args().args();
                        while let Some(Ok(arg)) = args.next() {
                            match arg {
                                _cli::arguments::Arg::LongOption("id") => {
                                    state = States::ExpectId;
                                }
                                _cli::arguments::Arg::Value(
                                    val,
                                ) if state == States::ExpectId => {
                                    state = States::Normal;
                                }
                                _cli::arguments::Arg::Value(
                                    name,
                                ) if state == States::Normal => {
                                    let args = args.into_args();
                                    let raw = _cli::command::RawCommand::new(name, args);
                                    let mut parent = |
                                        writer: &mut _cli::writer::Writer<'_, W, E>|
                                    {
                                        parent(writer)?;
                                        writer.write_str("led")?;
                                        writer.write_str(" ")?;
                                        Ok(())
                                    };
                                    return <LedCommand as _cli::service::Help>::command_help(
                                        &mut parent,
                                        raw,
                                        writer,
                                    );
                                }
                                _cli::arguments::Arg::Value(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _cli::arguments::Arg::LongOption(_)
                                | _cli::arguments::Arg::ShortOption(_) => break,
                                _cli::arguments::Arg::DoubleDash => {}
                            }
                        }
                        writer.writeln_str("Control LEDs")?;
                        writer.writeln_str("")?;
                        writer.write_title("Usage:")?;
                        writer.write_str(" ")?;
                        parent(writer)?;
                        writer.write_str("led")?;
                        writer.write_str(" [OPTIONS]")?;
                        writer.write_str(" <COMMAND>")?;
                        writer.writeln_str("")?;
                        writer.writeln_str("")?;
                        writer.write_title("Options:")?;
                        writer.writeln_str("")?;
                        writer.write_list_element("--id <ID>", "LED id", 10usize)?;
                        writer.write_list_element("-h, --help", "Print help", 10usize)?;
                        writer.writeln_str("")?;
                        <LedCommand as _cli::service::Help>::list_commands(writer)?;
                    }
                    "adc" => {
                        enum States {
                            Normal,
                            ExpectId,
                        }
                        #[automatically_derived]
                        impl ::core::cmp::Eq for States {
                            #[inline]
                            #[doc(hidden)]
                            #[coverage(off)]
                            fn assert_receiver_is_total_eq(&self) -> () {}
                        }
                        #[automatically_derived]
                        impl ::core::marker::StructuralPartialEq for States {}
                        #[automatically_derived]
                        impl ::core::cmp::PartialEq for States {
                            #[inline]
                            fn eq(&self, other: &States) -> bool {
                                let __self_discr = ::core::intrinsics::discriminant_value(
                                    self,
                                );
                                let __arg1_discr = ::core::intrinsics::discriminant_value(
                                    other,
                                );
                                __self_discr == __arg1_discr
                            }
                        }
                        let mut state = States::Normal;
                        let mut args = command.args().args();
                        while let Some(Ok(arg)) = args.next() {
                            match arg {
                                _cli::arguments::Arg::LongOption("id") => {
                                    state = States::ExpectId;
                                }
                                _cli::arguments::Arg::Value(
                                    val,
                                ) if state == States::ExpectId => {
                                    state = States::Normal;
                                }
                                _cli::arguments::Arg::Value(
                                    name,
                                ) if state == States::Normal => {
                                    let args = args.into_args();
                                    let raw = _cli::command::RawCommand::new(name, args);
                                    let mut parent = |
                                        writer: &mut _cli::writer::Writer<'_, W, E>|
                                    {
                                        parent(writer)?;
                                        writer.write_str("adc")?;
                                        writer.write_str(" ")?;
                                        Ok(())
                                    };
                                    return <AdcCommand<
                                        'a,
                                    > as _cli::service::Help>::command_help(
                                        &mut parent,
                                        raw,
                                        writer,
                                    );
                                }
                                _cli::arguments::Arg::Value(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _cli::arguments::Arg::LongOption(_)
                                | _cli::arguments::Arg::ShortOption(_) => break,
                                _cli::arguments::Arg::DoubleDash => {}
                            }
                        }
                        writer.writeln_str("Control ADC")?;
                        writer.writeln_str("")?;
                        writer.write_title("Usage:")?;
                        writer.write_str(" ")?;
                        parent(writer)?;
                        writer.write_str("adc")?;
                        writer.write_str(" [OPTIONS]")?;
                        writer.write_str(" <COMMAND>")?;
                        writer.writeln_str("")?;
                        writer.writeln_str("")?;
                        writer.write_title("Options:")?;
                        writer.writeln_str("")?;
                        writer.write_list_element("--id <ID>", "ADC id", 10usize)?;
                        writer.write_list_element("-h, --help", "Print help", 10usize)?;
                        writer.writeln_str("")?;
                        <AdcCommand<'a> as _cli::service::Help>::list_commands(writer)?;
                    }
                    "status" => {
                        writer.writeln_str("Show some status")?;
                        writer.writeln_str("")?;
                        writer.write_title("Usage:")?;
                        writer.write_str(" ")?;
                        parent(writer)?;
                        writer.write_str("status")?;
                        writer.writeln_str("")?;
                        writer.writeln_str("")?;
                        writer.write_title("Options:")?;
                        writer.writeln_str("")?;
                        writer.write_list_element("-h, --help", "Print help", 10usize)?;
                    }
                    _ => return Err(_cli::service::HelpError::UnknownCommand),
                }
                Ok(())
            }
        }
        impl<'a> _cli::service::FromRaw<'a> for BaseCommand<'a> {
            fn parse(
                command: _cli::command::RawCommand<'a>,
            ) -> Result<Self, _cli::service::ParseError<'a>> {
                let command = match command.name() {
                    "led" => {
                        let mut arg_id = None;
                        let mut sub_command = None;
                        enum States {
                            Normal,
                            ExpectId,
                        }
                        #[automatically_derived]
                        impl ::core::cmp::Eq for States {
                            #[inline]
                            #[doc(hidden)]
                            #[coverage(off)]
                            fn assert_receiver_is_total_eq(&self) -> () {}
                        }
                        #[automatically_derived]
                        impl ::core::marker::StructuralPartialEq for States {}
                        #[automatically_derived]
                        impl ::core::cmp::PartialEq for States {
                            #[inline]
                            fn eq(&self, other: &States) -> bool {
                                let __self_discr = ::core::intrinsics::discriminant_value(
                                    self,
                                );
                                let __arg1_discr = ::core::intrinsics::discriminant_value(
                                    other,
                                );
                                __self_discr == __arg1_discr
                            }
                        }
                        let mut state = States::Normal;
                        let mut positional = 0;
                        let mut args = command.args().args();
                        while let Some(arg) = args.next() {
                            let arg = arg
                                .map_err(|err| match err {
                                    _cli::arguments::ArgError::NonAsciiShortOption => {
                                        _cli::service::ParseError::NonAsciiShortOption
                                    }
                                })?;
                            match arg {
                                _cli::arguments::Arg::LongOption("id") => {
                                    state = States::ExpectId;
                                }
                                _cli::arguments::Arg::Value(
                                    val,
                                ) if state == States::ExpectId => {
                                    arg_id = Some(
                                        <u8 as _cli::arguments::FromArgument>::from_arg(val)?,
                                    );
                                    state = States::Normal;
                                }
                                _cli::arguments::Arg::Value(
                                    name,
                                ) if state == States::Normal => {
                                    let args = args.into_args();
                                    let raw = _cli::command::RawCommand::new(name, args);
                                    sub_command = Some(
                                        <LedCommand as _cli::service::FromRaw>::parse(raw)?,
                                    );
                                    break;
                                }
                                _cli::arguments::Arg::Value(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _cli::arguments::Arg::LongOption(option) => {
                                    return Err(_cli::service::ParseError::UnexpectedLongOption {
                                        name: option,
                                    });
                                }
                                _cli::arguments::Arg::ShortOption(option) => {
                                    return Err(_cli::service::ParseError::UnexpectedShortOption {
                                        name: option,
                                    });
                                }
                                _cli::arguments::Arg::DoubleDash => {}
                            }
                        }
                        BaseCommand::Led {
                            id: arg_id
                                .ok_or(_cli::service::ParseError::MissingRequiredArgument {
                                    name: "--id <ID>",
                                })?,
                            command: sub_command
                                .ok_or(_cli::service::ParseError::MissingRequiredArgument {
                                    name: "<COMMAND>",
                                })?,
                        }
                    }
                    "adc" => {
                        let mut arg_id = None;
                        let mut sub_command = None;
                        enum States {
                            Normal,
                            ExpectId,
                        }
                        #[automatically_derived]
                        impl ::core::cmp::Eq for States {
                            #[inline]
                            #[doc(hidden)]
                            #[coverage(off)]
                            fn assert_receiver_is_total_eq(&self) -> () {}
                        }
                        #[automatically_derived]
                        impl ::core::marker::StructuralPartialEq for States {}
                        #[automatically_derived]
                        impl ::core::cmp::PartialEq for States {
                            #[inline]
                            fn eq(&self, other: &States) -> bool {
                                let __self_discr = ::core::intrinsics::discriminant_value(
                                    self,
                                );
                                let __arg1_discr = ::core::intrinsics::discriminant_value(
                                    other,
                                );
                                __self_discr == __arg1_discr
                            }
                        }
                        let mut state = States::Normal;
                        let mut positional = 0;
                        let mut args = command.args().args();
                        while let Some(arg) = args.next() {
                            let arg = arg
                                .map_err(|err| match err {
                                    _cli::arguments::ArgError::NonAsciiShortOption => {
                                        _cli::service::ParseError::NonAsciiShortOption
                                    }
                                })?;
                            match arg {
                                _cli::arguments::Arg::LongOption("id") => {
                                    state = States::ExpectId;
                                }
                                _cli::arguments::Arg::Value(
                                    val,
                                ) if state == States::ExpectId => {
                                    arg_id = Some(
                                        <u8 as _cli::arguments::FromArgument>::from_arg(val)?,
                                    );
                                    state = States::Normal;
                                }
                                _cli::arguments::Arg::Value(
                                    name,
                                ) if state == States::Normal => {
                                    let args = args.into_args();
                                    let raw = _cli::command::RawCommand::new(name, args);
                                    sub_command = Some(
                                        <AdcCommand<'a> as _cli::service::FromRaw>::parse(raw)?,
                                    );
                                    break;
                                }
                                _cli::arguments::Arg::Value(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                                _cli::arguments::Arg::LongOption(option) => {
                                    return Err(_cli::service::ParseError::UnexpectedLongOption {
                                        name: option,
                                    });
                                }
                                _cli::arguments::Arg::ShortOption(option) => {
                                    return Err(_cli::service::ParseError::UnexpectedShortOption {
                                        name: option,
                                    });
                                }
                                _cli::arguments::Arg::DoubleDash => {}
                            }
                        }
                        BaseCommand::Adc {
                            id: arg_id
                                .ok_or(_cli::service::ParseError::MissingRequiredArgument {
                                    name: "--id <ID>",
                                })?,
                            command: sub_command
                                .ok_or(_cli::service::ParseError::MissingRequiredArgument {
                                    name: "<COMMAND>",
                                })?,
                        }
                    }
                    "status" => BaseCommand::Status,
                    cmd => return Err(_cli::service::ParseError::UnknownCommand),
                };
                Ok(command)
            }
        }
        impl<'a> BaseCommand<'a> {
            fn processor<
                W: _io::Write<Error = E>,
                E: _io::Error,
                F: FnMut(
                        &mut _cli::cli::CliHandle<'_, W, E>,
                        BaseCommand<'_>,
                    ) -> Result<(), E>,
            >(f: F) -> impl _cli::service::CommandProcessor<W, E> {
                struct Processor<
                    W: _io::Write<Error = E>,
                    E: _io::Error,
                    F: FnMut(
                            &mut _cli::cli::CliHandle<'_, W, E>,
                            BaseCommand<'_>,
                        ) -> Result<(), E>,
                > {
                    f: F,
                    _ph: core::marker::PhantomData<(W, E)>,
                }
                impl<
                    W: _io::Write<Error = E>,
                    E: _io::Error,
                    F: FnMut(
                            &mut _cli::cli::CliHandle<'_, W, E>,
                            BaseCommand<'_>,
                        ) -> Result<(), E>,
                > _cli::service::CommandProcessor<W, E> for Processor<W, E, F> {
                    fn process<'a>(
                        &mut self,
                        cli: &mut _cli::cli::CliHandle<'_, W, E>,
                        raw: _cli::command::RawCommand<'a>,
                    ) -> Result<(), _cli::service::ProcessError<'a, E>> {
                        let cmd = <BaseCommand<
                            '_,
                        > as _cli::service::FromRaw>::parse(raw)?;
                        (self.f)(cli, cmd)?;
                        Ok(())
                    }
                }
                Processor {
                    f,
                    _ph: core::marker::PhantomData,
                }
            }
        }
    };
}
