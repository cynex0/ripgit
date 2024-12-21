use crate::utils::path_utils::repo_file;
use anyhow::{anyhow, Context, Result};
use ini::Ini;
use std::path::Path;

// .git configuration file
#[derive(Debug)]
pub struct Config {
    pub core: CoreSection,
}

#[derive(Debug)]
pub struct CoreSection {
    pub repositoryformatversion: usize,
    pub filemode: bool,
    pub bare: bool,
}

impl Config {
    /// Create a default Config instance
    pub fn new() -> Config {
        Config {
            core: CoreSection {
                repositoryformatversion: 0,
                filemode: false,
                bare: false,
            },
        }
    }

    pub fn write<P>(&self, gitpath: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut conf = Ini::new();
        conf.with_section(Some("core"))
            .set(
                "repositoryformatversion",
                self.core.repositoryformatversion.to_string(),
            )
            .set("filemode", self.core.filemode.to_string())
            .set("bare", self.core.bare.to_string());

        let path = repo_file(gitpath, "config");

        conf.write_to_file(path)?;
        Ok(())
    }

    pub fn read<P>(gitpath: P) -> Result<Config>
    where
        P: AsRef<Path>,
    {
        let file = repo_file(gitpath, "config");
        if !file.exists() {
            return Err(anyhow!(format!("No configuration file at {:?}!", file)));
        }

        let conf = Ini::load_from_file(&file)
            .with_context(|| format!("Failed to parse config file at {:?}", file))?;

        if let Some(core_section) = conf.section(Some("core")) {
            let rfv = core_section.get("repositoryformatversion");
            let filemode = core_section.get("filemode");
            let bare = core_section.get("bare");
            match (rfv, filemode, bare) {
                (Some(r), Some(f), Some(b)) => {
                    return Ok(Config {
                        core: CoreSection {
                            repositoryformatversion: r.parse()?,
                            filemode: f.parse()?,
                            bare: b.parse()?,
                        },
                    })
                }
                _ => return Err(anyhow!("Malformatted configuration file!")),
            }
        } else {
            return Err(anyhow!("Malformatted configuration file!"));
        }
    }
}
