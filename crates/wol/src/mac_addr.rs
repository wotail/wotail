use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacAddr {
  bytes: Vec<u8>,
}

impl MacAddr {

  pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, &'static str> {
    let b = bytes.as_ref();
    if b.len() != 6 {
      return Err("MAC address must be 6 bytes");
    }
    Ok(Self { bytes: b.to_vec() })
  }

  pub fn from_str(s: &str) -> Result<Self, &'static str> {
    let parts: Vec<&str> = s.split(|c| c == ':' || c == '-').collect();
    if parts.len() != 6 {
      return Err("Invalid MAC address format");
    }
    let mut bytes = Vec::with_capacity(6);
    for part in parts {
      match u8::from_str_radix(part, 16) {
        Ok(b) => bytes.push(b),
        Err(_) => return Err("Invalid hex in MAC address"),
      }
    }
    Ok(Self { bytes })
  }

  pub fn as_bytes(&self) -> &[u8] {
    &self.bytes
  }
}

impl fmt::Display for MacAddr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, byte) in self.bytes.iter().enumerate() {
      if i != 0 {
        write!(f, ":")?;
      }
      write!(f, "{:02x}", byte)?;
    }
    Ok(())
  }
}

impl TryFrom<&str> for MacAddr {
  type Error = &'static str;

  fn try_from(s: &str) -> Result<Self, Self::Error> {
    MacAddr::from_str(s)
  }
}

impl TryFrom<String> for MacAddr {
  type Error = &'static str;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    MacAddr::from_str(&s)
  }
}
