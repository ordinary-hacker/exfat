#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct SectorID(u64);

impl core::fmt::Display for SectorID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for SectorID {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<SectorID> for u64 {
    fn from(value: SectorID) -> Self {
        value.0
    }
}

impl<I: Into<u64>> core::ops::Add<I> for SectorID {
    type Output = Self;

    fn add(self, rhs: I) -> Self {
        Self(self.0 + rhs.into())
    }
}

impl<I: Into<u64>> core::ops::AddAssign<I> for SectorID {
    fn add_assign(&mut self, rhs: I) {
        self.0 += rhs.into()
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct ClusterID(u32);

impl core::fmt::Display for ClusterID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for ClusterID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<ClusterID> for u32 {
    fn from(value: ClusterID) -> Self {
        value.0
    }
}

impl ClusterID {
    pub fn valid(&self) -> bool {
        return self.0 > 0;
    }
}

impl<I: Into<u32>> core::ops::Add<I> for ClusterID {
    type Output = Self;

    fn add(self, rhs: I) -> Self {
        Self(self.0 + rhs.into())
    }
}

impl<I: Into<u32>> core::ops::AddAssign<I> for ClusterID {
    fn add_assign(&mut self, rhs: I) {
        self.0 += rhs.into()
    }
}
