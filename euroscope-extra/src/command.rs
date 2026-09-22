//! Dot-command dispatch backed by [`argh`].
//!
//! EuroScope hands a plugin the raw command line through
//! [`Plugin::on_compile_command`] and expects a `bool` back: `true` means "I
//! consumed this, stop asking", `false` means "not mine, keep going". Every
//! plugin then repeats the same three chores — match its own prefix, split the
//! line into words, and print help and errors back through
//! [`Context::display_message`].
//!
//! [`CommandPlugin`] does all three. You declare a prefix and an [`argh`]
//! command tree, implement [`handle`](CommandPlugin::handle), and delegate
//! [`Plugin::on_compile_command`] to
//! [`dispatch_command`](CommandPlugin::dispatch_command).
//!
//! # What you get
//!
//! For `PREFIX = "myplugin"`:
//!
//! | Typed | Effect |
//! | --- | --- |
//! | `.myplugin <args>` | parsed and passed to [`handle`](CommandPlugin::handle) |
//! | `.myplugin <bad args>` | [`argh`]'s error and usage printed; consumed |
//! | `.myplugin help`, `.myplugin --help` | [`argh`]'s help printed; consumed |
//! | `.help myplugin` | the same help printed; consumed |
//! | `.help` | one summary line printed, then **declined** |
//! | anything else | declined, untouched |
//!
//! A bare `.help` is deliberately *not* consumed: several plugins may implement
//! `CommandPlugin`, and since EuroScope stops at the first handler that returns
//! `true`, consuming it would let whichever plugin is asked first swallow
//! `.help` for all the others. Declining lets each contribute its line.
//!
//! `.myplugin help` needs no code here — [`argh`]'s default help triggers are
//! `--help` and `help`.
//!
//! [`Plugin::on_compile_command`]: euroscope::Plugin::on_compile_command

use argh::TopLevelCommand;
use euroscope::{Context, Plugin};

/// What a command line resolves to, worked out without touching a [`Context`].
///
/// This is the whole decision [`CommandPlugin::dispatch_command`] makes;
/// [`route`] produces it and is where the behaviour is actually tested.
#[derive(Debug, PartialEq, Eq)]
enum Route<C> {
    /// Not addressed at us. The plugin declines (`false`) and prints nothing.
    Ignore,
    /// Parsed successfully. Hand to [`CommandPlugin::handle`], then consume
    /// (`true`).
    Run(C),
    /// Print these lines, then consume (`true`) — our own help, or an error on a
    /// line that used our prefix.
    Handled(Vec<String>),
    /// Print these lines, then decline (`false`) — our contribution to a bare
    /// `.help`.
    Offered(Vec<String>),
}

/// Resolve a raw EuroScope command line against a prefix, without needing a
/// [`Context`].
///
/// `prefix` is matched case-insensitively and carries **no** leading dot;
/// `description` is the one-line summary offered for a bare `.help`. The first
/// word of `command_line` is passed to [`argh`] as the command name, so
/// generated usage reads `Usage: .myplugin …` rather than naming a binary.
///
/// [`CommandPlugin::dispatch_command`] is a thin veneer over this; call it
/// directly only if you need the decision without the printing.
///
/// ```ignore
/// use argh::FromArgs;
/// use euroscope_extra::command::{Route, route};
///
/// /// Do the thing.
/// #[derive(FromArgs, Debug, PartialEq, Eq)]
/// struct Cmd {
///     /// altitude to assign
///     #[argh(option)]
///     alt: u32,
/// }
///
/// let parsed = route::<Cmd>("myplugin", "does the thing", ".myplugin --alt 5000");
/// assert_eq!(parsed, Route::Run(Cmd { alt: 5000_u32 }));
///
/// assert_eq!(route::<Cmd>("myplugin", "does the thing", ".other thing"), Route::Ignore);
/// ```
fn route<C: TopLevelCommand>(prefix: &str, description: &str, command_line: &str) -> Route<C> {
    let mut words = command_line.split_whitespace();
    // A null or non-UTF-8 command line reaches us as `""`, so an empty
    // iterator is a genuinely reachable input, not just a defensive branch.
    let Some(head) = words.next() else {
        return Route::Ignore;
    };
    let rest: Vec<&str> = words.collect();

    if head.eq_ignore_ascii_case(".help") {
        return match rest.first() {
            None => Route::Offered(vec![format!(".{prefix} — {description}")]),
            Some(asked) if asked.eq_ignore_ascii_case(prefix) => {
                Route::Handled(help_lines::<C>(prefix))
            }
            Some(_) => Route::Ignore,
        };
    }

    let Some(name) = head.strip_prefix('.') else {
        return Route::Ignore;
    };
    if !name.eq_ignore_ascii_case(prefix) {
        return Route::Ignore;
    }

    match C::from_args(&[head], &rest) {
        Ok(command) => Route::Run(command),
        // Both `EarlyExit` statuses land here: `Ok(())` is help that argh
        // rendered for us, `Err(())` is a parse failure with usage appended.
        // Either way the line was ours, so we print and consume.
        Err(early) => Route::Handled(split_lines(&early.output)),
    }
}

/// Render `C`'s help text. [`argh`] only produces it as a side effect of
/// parsing, so ask for `--help` and keep the output.
fn help_lines<C: TopLevelCommand>(prefix: &str) -> Vec<String> {
    let name = format!(".{prefix}");
    match C::from_args(&[&name], &["--help"]) {
        // Unreachable in practice: `--help` always exits early.
        Ok(_) => Vec::new(),
        Err(early) => split_lines(&early.output),
    }
}

/// Split multi-line [`argh`] output into chat lines.
///
/// [`Context::display_message`] prints exactly one line per call, while
/// [`argh`] hands back an 80-column block. Interior blank lines are kept so
/// argh's section breaks survive.
fn split_lines(output: &str) -> Vec<String> {
    output.lines().map(str::to_owned).collect()
}

/// Print each line into the chat area under `handler`.
///
/// An empty sender makes the lines look as though the controller typed them,
/// matching how the `tracing` bridge in `euroscope` reports.
fn reply(ctx: &Context, handler: &str, lines: &[String]) {
    for line in lines {
        ctx.display_message(handler, "", line);
    }
}

/// A [`Plugin`] whose dot-commands are parsed by [`argh`].
///
/// Implement this alongside [`Plugin`], then delegate
/// [`Plugin::on_compile_command`] to
/// [`dispatch_command`](Self::dispatch_command) — that one line is the whole
/// wiring. See the [module docs](self) for the exact command lines this
/// recognises.
///
/// ```ignore
/// use argh::FromArgs;
/// use euroscope::{Context, Plugin};
/// use euroscope_extra::command::CommandPlugin;
///
/// struct MyPlugin;
///
/// /// Control MyPlugin.
/// #[derive(FromArgs)]
/// struct Cmd {
///     /// altitude to assign
///     #[argh(option)]
///     alt: u32,
/// }
///
/// #[expect(
///     clippy::missing_trait_methods,
///     reason = "dispatch_command's default body is the point"
/// )]
/// impl CommandPlugin for MyPlugin {
///     type Command = Cmd;
///
///     const PREFIX: &'static str = "myplugin";
///     const DESCRIPTION: &'static str = "assign altitudes";
///
///     fn handle(&mut self, ctx: &mut Context, command: Cmd) {
///         let _ = (ctx, command.alt);
///     }
/// }
///
/// impl Plugin for MyPlugin {
///     const NAME: &'static str = "MyPlugin";
///     const VERSION: &'static str = "1.0.0";
///     const AUTHOR: &'static str = "me";
///
///     fn new(_ctx: &mut Context) -> Self {
///         Self
///     }
///
///     fn on_compile_command(&mut self, ctx: &mut Context, command_line: &str) -> bool {
///         self.dispatch_command(ctx, command_line)
///     }
/// }
/// ```
pub trait CommandPlugin: Plugin {
    /// The [`argh`] command tree. A `#[derive(FromArgs)]` struct — typically one
    /// holding an `#[argh(subcommand)]` enum.
    type Command: TopLevelCommand;

    /// The dot-command prefix, **without** the leading dot. Matched
    /// case-insensitively, so `"myplugin"` answers to `.myplugin` and
    /// `.MyPlugin` alike.
    const PREFIX: &'static str;

    /// One-line summary offered in response to a bare `.help`. Defaults to
    /// [`Plugin::NAME`].
    const DESCRIPTION: &'static str = Self::NAME;

    /// Run a command that parsed cleanly.
    ///
    /// [`argh`] has already rejected unknown subcommands and bad arguments, so
    /// there is no "decline" case here: reaching this method means the line was
    /// yours.
    fn handle(&mut self, ctx: &mut Context, command: Self::Command);

    /// Drop-in body for [`Plugin::on_compile_command`]. Returns what EuroScope
    /// expects: `true` if the line was consumed.
    ///
    /// [`Plugin::on_compile_command`]: euroscope::Plugin::on_compile_command
    fn dispatch_command(&mut self, ctx: &mut Context, command_line: &str) -> bool {
        match route::<Self::Command>(Self::PREFIX, Self::DESCRIPTION, command_line) {
            Route::Ignore => false,
            Route::Run(command) => {
                self.handle(ctx, command);
                true
            }
            Route::Handled(lines) => {
                reply(ctx, Self::NAME, &lines);
                true
            }
            Route::Offered(lines) => {
                reply(ctx, Self::NAME, &lines);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use argh::FromArgs;
    use euroscope::{Context, Plugin};

    use super::{CommandPlugin, Route, route};

    /// Control the test plugin.
    #[derive(FromArgs, Debug, PartialEq, Eq)]
    struct Cmd {
        #[argh(subcommand)]
        sub: Sub,
    }

    #[derive(FromArgs, Debug, PartialEq, Eq)]
    #[argh(subcommand)]
    enum Sub {
        Set(Set),
    }

    /// Assign an altitude.
    #[derive(FromArgs, Debug, PartialEq, Eq)]
    #[argh(subcommand, name = "set")]
    struct Set {
        /// altitude in feet
        #[argh(positional)]
        alt: u32,
    }

    const PREFIX: &str = "myplugin";
    const DESCRIPTION: &str = "does the thing";

    /// Mirrors the [`CommandPlugin`] doc example so the author-facing shape stays
    /// compile-checked. The doc block itself is `ignore`d: a doctest links a real
    /// executable, and pulling `es_screen_*` out of the shim archive drags in
    /// `RustRadarScreen`, whose `rust_screen_*` callbacks only exist once
    /// `register_plugin!` has been invoked.
    struct TestPlugin;

    #[expect(
        clippy::missing_trait_methods,
        reason = "dispatch_command's default body is the whole point of the trait"
    )]
    impl CommandPlugin for TestPlugin {
        type Command = Cmd;

        const PREFIX: &'static str = PREFIX;

        fn handle(&mut self, ctx: &mut Context, command: Cmd) {
            let _ = (ctx, command);
        }
    }

    #[expect(
        clippy::missing_trait_methods,
        reason = "only on_compile_command matters for this fixture"
    )]
    impl Plugin for TestPlugin {
        const AUTHOR: &'static str = "tests";
        const NAME: &'static str = "TestPlugin";
        const VERSION: &'static str = "0.0.0";

        fn new(_ctx: &mut Context) -> Self {
            Self
        }

        fn on_compile_command(&mut self, ctx: &mut Context, command_line: &str) -> bool {
            self.dispatch_command(ctx, command_line)
        }
    }

    fn go(line: &str) -> Route<Cmd> {
        route::<Cmd>(PREFIX, DESCRIPTION, line)
    }

    fn set(alt: u32) -> Route<Cmd> {
        Route::Run(Cmd {
            sub: Sub::Set(Set { alt }),
        })
    }

    /// The lines of a `Handled`/`Offered` route, or a panic naming what we got.
    fn lines(route: &Route<Cmd>) -> &[String] {
        match route {
            Route::Handled(lines) | Route::Offered(lines) => lines,
            Route::Ignore | Route::Run(_) => panic!("expected printed output, got {route:?}"),
        }
    }

    #[test]
    fn description_defaults_to_the_plugin_name() {
        assert_eq!(<TestPlugin as CommandPlugin>::DESCRIPTION, TestPlugin::NAME);
    }

    #[test]
    fn blank_lines_are_not_ours() {
        assert_eq!(go(""), Route::Ignore);
        assert_eq!(go("   "), Route::Ignore);
        assert_eq!(go("\t \n"), Route::Ignore);
    }

    #[test]
    fn other_prefixes_are_not_ours() {
        assert_eq!(go(".other set 5000"), Route::Ignore);
        assert_eq!(go(".myplugins set 5000"), Route::Ignore);
        // No leading dot at all.
        assert_eq!(go("myplugin set 5000"), Route::Ignore);
    }

    #[test]
    fn bare_help_offers_a_summary_without_consuming() {
        let route = go(".help");
        assert!(
            matches!(route, Route::Offered(_)),
            "bare .help must decline so other plugins are still asked, got {route:?}"
        );
        let lines = lines(&route);
        assert_eq!(lines.len(), 1_usize);
        assert!(lines[0_usize].contains(".myplugin"), "{lines:?}");
        assert!(lines[0_usize].contains(DESCRIPTION), "{lines:?}");
    }

    #[test]
    fn help_for_our_prefix_prints_usage_and_consumes() {
        let route = go(".help myplugin");
        assert!(matches!(route, Route::Handled(_)), "{route:?}");
        let joined = lines(&route).join("\n");
        assert!(joined.contains("Usage: .myplugin"), "{joined}");
    }

    #[test]
    fn prefix_matching_ignores_case() {
        assert!(matches!(go(".HELP MyPlugin"), Route::Handled(_)));
        assert_eq!(go(".MYPLUGIN set 5000"), set(5000_u32));
    }

    #[test]
    fn help_for_another_prefix_is_not_ours() {
        assert_eq!(go(".help other"), Route::Ignore);
    }

    #[test]
    fn argh_help_triggers_work_on_our_prefix() {
        for line in [".myplugin help", ".myplugin --help"] {
            let route = go(line);
            assert!(matches!(route, Route::Handled(_)), "{line}: {route:?}");
            assert!(
                lines(&route).join("\n").contains("Usage: .myplugin"),
                "{line}"
            );
        }
    }

    #[test]
    fn parse_errors_are_reported_and_consumed() {
        // Missing the required subcommand, and an unknown one.
        for line in [".myplugin", ".myplugin bogus"] {
            let route = go(line);
            assert!(
                matches!(route, Route::Handled(_)),
                "{line} used our prefix, so it must be consumed, got {route:?}"
            );
            assert!(
                !lines(&route).is_empty(),
                "{line}: nothing to show the user"
            );
        }
    }

    #[test]
    fn valid_commands_parse() {
        assert_eq!(go(".myplugin set 5000"), set(5000_u32));
    }

    #[test]
    fn surrounding_whitespace_is_irrelevant() {
        assert_eq!(go("  .myplugin   set    5000  "), set(5000_u32));
    }
}
