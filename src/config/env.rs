use std::path::PathBuf;

use typed_env::{Envar, EnvarDef};

pub static EDITOR: Envar<String> = Envar::on_demand("EDITOR", || EnvarDef::Unset);
pub static GEST_CONFIG: Envar<PathBuf> = Envar::on_demand("GEST_CONFIG", || EnvarDef::Unset);
pub static GEST_DATA_DIR: Envar<PathBuf> = Envar::on_demand("GEST_DATA_DIR", || EnvarDef::Unset);
pub static GEST_LOG_LEVEL: Envar<String> = Envar::on_demand("GEST_LOG_LEVEL", || EnvarDef::Unset);
pub static VISUAL: Envar<String> = Envar::on_demand("VISUAL", || EnvarDef::Unset);
