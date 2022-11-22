// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

/// Simple serialization into YAML for complex types
///
/// # Example
///
/// ```ignore
/// # #[macro_use] extern crate serde;
/// #[macro_use]
/// extern crate amplify_derive;
///
/// use amplify::ToYamlString;
///
/// #[derive(Clone, PartialEq, Eq, Debug, Display, Serialize, Deserialize)]
/// #[display(ComplexType::to_yaml_string)]
/// pub struct ComplexType {/* Some really complex data */}
///
/// impl ToYamlString for ComplexType {}
/// ```
#[cfg(feature = "serde")]
pub trait ToYamlString
where
    Self: serde::Serialize,
{
    /// Performs conversion of the `self` into a YAML-encoded string
    fn to_yaml_string(&self) -> String {
        serde_yaml::to_string(self).expect("internal YAML serialization error")
    }
}

/// Simple serialization into JSON for complex types
///
/// # Example
///
/// ```ignore
/// # #[macro_use] extern crate serde;
/// #[macro_use]
/// extern crate amplify_derive;
/// use amplify::ToJsonString;
///
/// #[derive(Clone, PartialEq, Eq, Debug, Display, Serialize, Deserialize)]
/// #[display(ComplexType::to_json_string)]
/// pub struct ComplexType {/* Some really complex data */}
///
/// impl ToJsonString for ComplexType {}
/// ```
#[cfg(feature = "serde")]
pub trait ToJsonString
where
    Self: serde::Serialize,
{
    /// Performs conversion of the `self` into a JSON-encoded string
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).expect("internal JSON serialization error")
    }
}

/// Simple serialization into TOML for complex types
///
/// # Example
///
/// ```ignore
/// # #[macro_use] extern crate serde;
/// #[macro_use]
/// extern crate amplify_derive;
/// use amplify::ToTomlString;
///
/// #[derive(Clone, PartialEq, Eq, Debug, Display, Serialize, Deserialize)]
/// #[display(ComplexType::to_toml_string)]
/// pub struct ComplexType {/* Some really complex data */}
///
/// impl ToTomlString for ComplexType {}
/// ```
#[cfg(feature = "serde")]
pub trait ToTomlString
where
    Self: serde::Serialize,
{
    /// Performs conversion of the `self` into a TOML-encoded string
    fn to_toml_string(&self) -> String {
        toml::to_string(self).expect("internal TOML serialization error")
    }
}
