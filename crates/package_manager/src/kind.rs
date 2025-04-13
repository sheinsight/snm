use strum::EnumString;

#[derive(Debug, PartialEq, Eq, Clone, strum::Display, EnumString, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum PackageManagerKind {
  Npm,
  Yarn,
  Pnpm,
}
