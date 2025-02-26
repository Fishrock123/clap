use crate::Str;

/// [`Arg`][crate::Arg] or [`ArgGroup`][crate::ArgGroup] identifier
///
/// This is used for accessing the value in [`ArgMatches`][crate::ArgMatches] or defining
/// relationships between `Arg`s and `ArgGroup`s with functions like
/// [`Arg::conflicts_with`][crate::Arg::conflicts_with].
#[derive(Default, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Id(Str);

impl Id {
    pub(crate) const HELP: Self = Self::from_static_ref("help");
    pub(crate) const VERSION: Self = Self::from_static_ref("version");
    pub(crate) const EXTERNAL: Self = Self::from_static_ref("");

    const fn from_static_ref(name: &'static str) -> Self {
        Self(Str::from_static_ref(name))
    }

    /// Get the raw string of the `Id`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&'_ Id> for Id {
    fn from(id: &'_ Id) -> Self {
        id.clone()
    }
}

impl From<Str> for Id {
    fn from(name: Str) -> Self {
        Self(name)
    }
}

impl From<&'_ Str> for Id {
    fn from(name: &'_ Str) -> Self {
        Self(name.into())
    }
}

impl From<std::string::String> for Id {
    fn from(name: std::string::String) -> Self {
        Self(name.into())
    }
}

impl From<&'_ std::string::String> for Id {
    fn from(name: &'_ std::string::String) -> Self {
        Self(name.into())
    }
}

impl From<&'static str> for Id {
    fn from(name: &'static str) -> Self {
        Self(name.into())
    }
}

impl From<&'_ &'static str> for Id {
    fn from(name: &'_ &'static str) -> Self {
        Self(name.into())
    }
}

impl std::fmt::Display for Id {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_str(), f)
    }
}

impl std::fmt::Debug for Id {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_str(), f)
    }
}

impl AsRef<str> for Id {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::borrow::Borrow<str> for Id {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for Id {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

impl PartialEq<&'_ str> for Id {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }
}

impl PartialEq<Str> for Id {
    #[inline]
    fn eq(&self, other: &Str) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl PartialEq<std::string::String> for Id {
    #[inline]
    fn eq(&self, other: &std::string::String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}
