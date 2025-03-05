use anyhow::Result;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SemVer {
    major: u64,
    minor: u64,
    patch: u64,
    rc: Option<u64>,
}

impl SemVer {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            rc: None,
        }
    }

    pub fn new_with_rc(major: u64, minor: u64, patch: u64, rc: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            rc: Some(rc),
        }
    }

    pub fn new_with_string(version: &str) -> Result<Self> {
        // X.Y.Z-rc.A
        let mut version_split_rc = version.split("-rc.");
        let mut version = version_split_rc
            .next()
            .ok_or_else(|| anyhow::anyhow!("No version found"))?
            .split('.');
        let rc = version_split_rc
            .next()
            .map(|rc| rc.parse())
            .transpose()?;
        let major = version
            .next()
            .ok_or_else(|| anyhow::anyhow!("No major version found"))?
            .parse()?;
        let minor = version
            .next()
            .ok_or_else(|| anyhow::anyhow!("No minor version found"))?
            .parse()?;
        let patch = version
            .next()
            .ok_or_else(|| anyhow::anyhow!("No patch version found"))?
            .parse()?;
        match rc {
            Some(rc) => Ok(Self::new_with_rc(major, minor, patch, rc)),
            None => Ok(Self::new(major, minor, patch)),
        }
    }

    pub fn next_major(&self) -> Result<Self> {
        match self.rc {
            Some(_) => Err(anyhow::anyhow!("You have to release before bumping major version")),
            None => Ok(Self::new_with_rc(self.major + 1, 0, 0, 1)),
        }
    }

    pub fn next_minor(&self) -> Result<Self> {
        match self.rc {
            Some(_) => Err(anyhow::anyhow!("You have to release before bumping minor version")),
            None => Ok(Self::new_with_rc(self.major, self.minor + 1, 0, 1)),
        }
    }

    pub fn next_patch(&self) -> Result<Self> {
        match self.rc {
            Some(_) => Err(anyhow::anyhow!("You have to release before bumping patch version")),
            None => Ok(Self::new_with_rc(self.major, self.minor, self.patch + 1, 1)),
        }
    }

    pub fn next_rc(&self) -> Result<Self> {
        match self.rc {
            Some(rc) => Ok(Self::new_with_rc(self.major, self.minor, self.patch, rc + 1)),
            None => Err(anyhow::anyhow!("Not release candidate version")),
        }
    }

    pub fn release(&self) -> Result<Self> {
        match self.rc {
            Some(_) => Ok(Self::new(self.major, self.minor, self.patch)),
            None => Err(anyhow::anyhow!("Not release candidate version")),
        }
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self.rc {
            Some(rc) => write!(f, "{}.{}.{}-rc.{}", self.major, self.minor, self.patch, rc),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}
