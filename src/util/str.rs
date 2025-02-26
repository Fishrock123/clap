/// A UTF-8-encoded fixed string
#[derive(Default, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Str {
    name: Inner,
}

impl Str {
    pub(crate) fn from_string(name: std::string::String) -> Self {
        Self {
            name: Inner::Owned(name.into_boxed_str()),
        }
    }

    pub(crate) fn from_ref(name: &str) -> Self {
        Self {
            name: Inner::Owned(Box::from(name)),
        }
    }

    pub(crate) const fn from_static_ref(name: &'static str) -> Self {
        Self {
            name: Inner::Static(name),
        }
    }

    /// Get the raw string of the `Str`
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}

impl From<&'_ Str> for Str {
    fn from(id: &'_ Str) -> Self {
        id.clone()
    }
}

impl From<std::string::String> for Str {
    fn from(name: std::string::String) -> Self {
        Self::from_string(name)
    }
}

impl From<&'_ std::string::String> for Str {
    fn from(name: &'_ std::string::String) -> Self {
        Self::from_ref(name.as_str())
    }
}

impl From<&'static str> for Str {
    fn from(name: &'static str) -> Self {
        Self::from_static_ref(name)
    }
}

impl From<&'_ &'static str> for Str {
    fn from(name: &'_ &'static str) -> Self {
        Self::from_static_ref(*name)
    }
}

impl std::fmt::Display for Str {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.as_str(), f)
    }
}

impl std::fmt::Debug for Str {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.as_str(), f)
    }
}

impl std::ops::Deref for Str {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Str {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Str {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<std::ffi::OsStr> for Str {
    #[inline]
    fn as_ref(&self) -> &std::ffi::OsStr {
        (&**self).as_ref()
    }
}

impl AsRef<std::path::Path> for Str {
    #[inline]
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self)
    }
}

impl std::borrow::Borrow<str> for Str {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for Str {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.as_str(), other)
    }
}

impl PartialEq<&'_ str> for Str {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        PartialEq::eq(self.as_str(), *other)
    }
}

impl PartialEq<std::string::String> for Str {
    #[inline]
    fn eq(&self, other: &std::string::String) -> bool {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

#[derive(Clone)]
enum Inner {
    Static(&'static str),
    Owned(Box<str>),
}

impl Inner {
    fn as_str(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s.as_ref(),
        }
    }
}

impl Default for Inner {
    fn default() -> Self {
        Self::Static("")
    }
}

impl PartialEq for Inner {
    fn eq(&self, other: &Inner) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for Inner {
    fn cmp(&self, other: &Inner) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Eq for Inner {}

impl std::hash::Hash for Inner {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
