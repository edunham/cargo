use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::Path;

use util::{CargoResult, ProcessBuilder, process};
use util::Config;

/// Prototype for a command that must be executed.
#[derive(Clone)]
pub struct CommandPrototype {
    ty: CommandType,
    builder: ProcessBuilder,
}

impl CommandPrototype {
    pub fn new(ty: CommandType, config: &Config)
               -> CargoResult<CommandPrototype> {
        Ok(CommandPrototype {
            builder: {
                let mut p = match ty {
                    CommandType::Rustc => try!(config.rustc()).process(),
                    CommandType::Rustdoc => process(&*try!(config.rustdoc())),
                    CommandType::Target(ref s) |
                    CommandType::Host(ref s) => process(s),
                };
                p.cwd(config.cwd());
                p
            },
            ty: ty,
        })
    }

    pub fn get_type(&self) -> &CommandType { &self.ty }

    pub fn arg<T: AsRef<OsStr>>(&mut self, arg: T) -> &mut CommandPrototype {
        self.builder.arg(arg);
        self
    }

    pub fn args<T: AsRef<OsStr>>(&mut self, arguments: &[T]) -> &mut CommandPrototype {
        self.builder.args(arguments);
        self
    }

    pub fn cwd<T: AsRef<OsStr>>(&mut self, path: T) -> &mut CommandPrototype {
        self.builder.cwd(path);
        self
    }

    pub fn env<T: AsRef<OsStr>>(&mut self, key: &str, val: T)
                                -> &mut CommandPrototype {
        self.builder.env(key, val);
        self
    }

    pub fn get_args(&self) -> &[OsString] { self.builder.get_args() }
    pub fn get_cwd(&self) -> Option<&Path> { self.builder.get_cwd() }

    pub fn get_env(&self, var: &str) -> Option<OsString> {
        self.builder.get_env(var)
    }

    pub fn get_envs(&self) -> &HashMap<String, Option<OsString>> {
        self.builder.get_envs()
    }

    pub fn into_process_builder(self) -> ProcessBuilder {
        self.builder
    }
}

impl fmt::Display for CommandPrototype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.builder.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub enum CommandType {
    Rustc,
    Rustdoc,

    /// The command is to be executed for the target architecture.
    Target(OsString),

    /// The command is to be executed for the host architecture.
    Host(OsString),
}
