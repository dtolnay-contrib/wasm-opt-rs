pub use crate::features::Feature;
pub use crate::passes::Pass;
use std::collections::HashSet;

/// Optimization options and optimization builder.
///
/// This type declares all supported Binaryen options.
/// It can be modified directly or by its [builder-pattern] methods.
///
/// Call [`OptimizationOptions::run`] to perform the optimizations.
///
/// [builder-pattern]: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
#[derive(Clone, Debug, Default)]
pub struct OptimizationOptions {
    /// Options for reading the unoptimized wasm module.
    pub reader: ReaderOptions,
    /// Options for writing the optimized wasm module.
    pub writer: WriterOptions,
    /// Options related to inlining.
    pub inlining: InliningOptions,
    /// Options that affect how optimization passes behave.
    pub passopts: PassOptions,
    /// The set of optimization passes to apply.
    pub passes: Passes,
    /// The set of wasm-features.
    pub features: Features,
}

/// Options for reading the unoptimized wasm module.
#[derive(Copy, Clone, Debug)]
pub struct ReaderOptions {
    /// The module format: wasm, wat, or either.
    ///
    /// The default value is [`FileType::Any`].
    ///
    /// When this is `FileType::Any` the first bytes
    /// of the module will be inspected to determine whether
    /// the module is in binary or text format,
    /// then read as appropriate.
    pub file_type: FileType,
}

/// Options for writing the optimized wasm module.
#[derive(Copy, Clone, Debug)]
pub struct WriterOptions {
    /// The module format: wasm, wat, or either.
    ///
    /// The default value is [`FileType::Wasm`].
    ///
    /// Note that when this is [`FileType::Any`] the following logic applies:
    ///
    /// If [`ReaderOptions::file_type`] is [`FileType::Wat`],
    /// write a wat file, otherwise write a wasm file.
    pub file_type: FileType,
}

/// Module format used by [`ReaderOptions`] and [`WriterOptions`].
#[derive(Copy, Clone, Debug)]
pub enum FileType {
    /// A binary wasm module.
    Wasm,
    /// A text wasm module in wat format.
    Wat,
    /// Either a binary or text module.
    ///
    /// See the documentation for [`ReaderOptions`] and [`WriterOptions`]
    /// for an explanation of how this is interpreted.
    Any,
}

/// Options related to inlining.
#[derive(Copy, Clone, Debug)]
pub struct InliningOptions {
    pub always_inline_max_size: u32,
    pub one_caller_inline_max_size: u32,
    pub flexible_inline_max_size: u32,
    pub allow_functions_with_loops: bool,
    pub partial_inlining_ifs: u32,
}

/// Options that affect how optimization passes behave.
#[derive(Copy, Clone, Debug)]
pub struct PassOptions {
    /// Validate both the unoptimized module and the optimized module.
    ///
    /// Default: `true`.
    pub validate: bool,
    pub validate_globally: bool,
    /// The amount of optimization to apply.
    ///
    /// The default depends on how [`OptimizationOptions`] is constructed.
    pub optimize_level: OptimizeLevel,
    /// The amount of effort to put into reducing module size.
    ///
    /// The default depends on how [`OptimizationOptions`] is constructed.
    pub shrink_level: ShrinkLevel,
    pub traps_never_happen: bool,
    pub low_memory_unused: bool,
    pub fast_math: bool,
    pub zero_filled_memory: bool,
    pub debug_info: bool,
}

/// The amount of optimization to apply.
///
/// This is interpreted differently by different passes.
///
/// See the documentation of various [`OptimizationOptions`]
/// constructors for a general description of how these behave.
#[derive(Copy, Clone, Debug)]
pub enum OptimizeLevel {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
}

/// The amount of effort to put into reducing module size.
///
/// This is interpreted differently by different passes.
///
/// See the documentation of various [`OptimizationOptions`]
/// constructors for a general description of how these behave.
#[derive(Copy, Clone, Debug)]
pub enum ShrinkLevel {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
}

/// The set of optimization passes to apply.
#[derive(Clone, Debug)]
pub struct Passes {
    /// Apply the default set of optimization passes.
    pub add_default_passes: bool,
    /// Additional passes to apply.
    pub more_passes: Vec<Pass>,
}

#[derive(Clone, Debug)]
pub enum Features {
    Default,
    MvpOnly,
    All,
    Custom {
        enabled: HashSet<Feature>,
        disabled: HashSet<Feature>,
    },
}

/// Constructors.
impl OptimizationOptions {
    /// Optimize for size.
    ///
    /// This corresponds to the `-Os` argument to `wasm-opt`,
    /// and also the `-O` argument to `wasm-opt`.
    ///
    /// It is the same as [`OptimizationOptions::default`].
    ///
    /// It applies
    /// - [`Passes::add_default_passes`],
    /// - [`OptimizeLevel::Level2`],
    /// - [`ShrinkLevel::Level1`].
    pub fn new_optimize_for_size() -> Self {
        OptimizationOptions::default()
    }

    /// Optimize for size, but even more.
    ///
    /// It applies
    /// - [`Passes::add_default_passes`],
    /// - [`OptimizeLevel::Level2`],
    /// - [`ShrinkLevel::Level2`].
    ///
    /// This corresponds to the `-Oz` argument to `wasm-opt`.
    pub fn new_optimize_for_size_aggressively() -> Self {
        let mut passopts = PassOptions::default();
        passopts.optimize_level = OptimizeLevel::Level2;
        passopts.shrink_level = ShrinkLevel::Level2;

        let mut opts = OptimizationOptions::default();
        opts.passopts = passopts;

        opts
    }

    /// Do not optimize.
    ///
    /// It applies
    /// - [`OptimizeLevel::Level0`],
    /// - [`ShrinkLevel::Level0`].
    ///
    /// It adds no default passes.
    ///
    /// This corresponds to the `-O0` argument to `wasm-opt`.
    pub fn new_opt_level_0() -> Self {
        let mut passopts = PassOptions::default();
        passopts.optimize_level = OptimizeLevel::Level0;
        passopts.shrink_level = ShrinkLevel::Level0;

        let mut opts = OptimizationOptions::default();
        opts.passopts = passopts;

        opts.passes.add_default_passes = false;

        opts
    }

    /// Apply basic optimizations.
    ///
    /// Useful for fast iteration.
    ///
    /// It applies
    /// - [`Passes::add_default_passes`],
    /// - [`OptimizeLevel::Level1`],
    /// - [`ShrinkLevel::Level0`].
    ///
    /// This corresponds to the `-O1` argument to `wasm-opt`.
    pub fn new_opt_level_1() -> Self {
        let mut passopts = PassOptions::default();
        passopts.optimize_level = OptimizeLevel::Level1;
        passopts.shrink_level = ShrinkLevel::Level0;

        let mut opts = OptimizationOptions::default();
        opts.passopts = passopts;

        opts
    }

    /// Apply most optimizations.
    ///
    /// This level of optimization is appropriate for most applications.
    /// Higher optimization levels will not necessarily yield better performance,
    /// but will take longer to optimize.
    ///
    /// It applies
    /// - [`Passes::add_default_passes`],
    /// - [`OptimizeLevel::Level2`],
    /// - [`ShrinkLevel::Level0`].
    ///
    /// This corresponds to the `-O2` argument to `wasm-opt`.
    pub fn new_opt_level_2() -> Self {
        let mut passopts = PassOptions::default();
        passopts.optimize_level = OptimizeLevel::Level2;
        passopts.shrink_level = ShrinkLevel::Level0;

        let mut opts = OptimizationOptions::default();
        opts.passopts = passopts;

        opts
    }

    /// Apply slower optimizations.
    ///
    /// Spends potentially a lot of time on optimizations.
    ///
    /// It applies
    /// - [`Passes::add_default_passes`],
    /// - [`OptimizeLevel::Level3`],
    /// - [`ShrinkLevel::Level0`].
    ///
    /// This corresponds to the `-O3` argument to `wasm-opt`.
    pub fn new_opt_level_3() -> Self {
        let mut passopts = PassOptions::default();
        passopts.optimize_level = OptimizeLevel::Level3;
        passopts.shrink_level = ShrinkLevel::Level0;

        let mut opts = OptimizationOptions::default();
        opts.passopts = passopts;

        opts
    }

    /// Apply the most aggressive optimizations.
    ///
    /// Flattens the IR, which can take a lot of time and memory,
    /// but may be useful on nested / complex / less-optimized input.
    ///
    /// It applies
    /// - [`Passes::add_default_passes`],
    /// - [`OptimizeLevel::Level4`],
    /// - [`ShrinkLevel::Level0`].
    ///
    /// This corresponds to the `-O4` argument to `wasm-opt`.
    pub fn new_opt_level_4() -> Self {
        let mut passopts = PassOptions::default();
        passopts.optimize_level = OptimizeLevel::Level4;
        passopts.shrink_level = ShrinkLevel::Level0;

        let mut opts = OptimizationOptions::default();
        opts.passopts = passopts;

        opts
    }
}

impl Default for ReaderOptions {
    fn default() -> ReaderOptions {
        ReaderOptions {
            file_type: FileType::Any,
        }
    }
}

impl Default for WriterOptions {
    fn default() -> WriterOptions {
        WriterOptions {
            file_type: FileType::Wasm,
        }
    }
}

impl Default for InliningOptions {
    fn default() -> InliningOptions {
        InliningOptions {
            always_inline_max_size: 2,
            one_caller_inline_max_size: u32::max_value(),
            flexible_inline_max_size: 20,
            allow_functions_with_loops: false,
            partial_inlining_ifs: 0,
        }
    }
}

impl Default for PassOptions {
    fn default() -> PassOptions {
        PassOptions {
            validate: true,
            validate_globally: false,
            optimize_level: OptimizeLevel::default(),
            shrink_level: ShrinkLevel::default(),
            traps_never_happen: false,
            low_memory_unused: false,
            fast_math: false,
            zero_filled_memory: false,
            debug_info: false,
        }
    }
}

impl Default for OptimizeLevel {
    fn default() -> OptimizeLevel {
        OptimizeLevel::Level2
    }
}

impl Default for ShrinkLevel {
    fn default() -> ShrinkLevel {
        ShrinkLevel::Level1
    }
}

impl Default for Passes {
    fn default() -> Passes {
        Passes {
            add_default_passes: true,
            more_passes: vec![],
        }
    }
}

impl Default for Features {
    fn default() -> Features {
        Features::Default
    }
}
