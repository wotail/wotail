use std::fmt;


/// A struct that represents an IP Address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IPAddrV4 {
  bytes: [u8; 4],
}

impl IPAddrV4 {

  /// Converts a vec of bytes into an IP address
  pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self, &'static str> {
    let b: &[u8] = bytes.as_ref();
    if b.len() != 4 {
      return Err("IP address must be 4 bytes");
    }
    Ok(Self { bytes: b.try_into().map_err(|_| "IP address must be 4 bytes")? })
  }

  /// Converts a string into an IP address
  pub fn from_str(s: &str) -> Result<Self, &'static str> {
    let parts: Vec<&str> = s.split(|c| c == '.' || c == ',').collect();
    if parts.len() != 4 {
      return Err("Invalid IP address format");
    }
    let mut bytes: Vec<u8> = Vec::with_capacity(4);
    for part in parts {
      match u8::from_str_radix(part, 10) {
        Ok(b) => bytes.push(b),
        Err(_) => return Err("Invalid Dec in IP address"),
      }
    }
    Ok(Self { bytes: bytes.try_into().map_err(|_| "IP Address must be 4 bytes")? })
  }

  /// checks if an IP address is local
  pub fn is_local(&self) -> bool {
    let ca: bool = self.bytes[0] == 10;
    let cb: bool = self.bytes[0] == 172 && self.bytes[1] == 16;
    let cc: bool = self.bytes[0] == 192 && self.bytes[1] == 168 && self.bytes[2] == 1;
    ca || cb || cc
  }

  /// Returns the IP address as a vec of bytes
  pub fn as_bytes(&self) -> &[u8; 4] {
    &self.bytes
  }
}

impl fmt::Display for IPAddrV4 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (i, byte) in self.bytes.iter().enumerate() {
      if i != 0 {
        write!(f, ".")?;
      }
      write!(f, "{}", byte)?;
    }
    Ok(())
  }
}

impl TryFrom<&str> for IPAddrV4 {
  type Error = &'static str;

  fn try_from(s: &str) -> Result<Self, Self::Error> {
    IPAddrV4::from_str(s)
  }
}

impl TryFrom<String> for IPAddrV4 {
  type Error = &'static str;

  fn try_from(s: String) -> Result<Self, Self::Error> {
    IPAddrV4::from_str(&s)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_valid() {
    let ip = IPAddrV4::from_bytes([192, 168, 1, 1]);
    assert!(ip.is_ok());
    assert_eq!(ip.unwrap().as_bytes(), &[192, 168, 1, 1]);

    let ip = IPAddrV4::from_str("10.0.0.1");
    assert!(ip.is_ok());
    assert_eq!(ip.unwrap().as_bytes(), &[10, 0, 0, 1]);

    let ip: Result<IPAddrV4, _> = IPAddrV4::try_from("192.168.1.1");
    assert!(ip.is_ok());
    assert_eq!(ip.unwrap().as_bytes(), &[192, 168, 1, 1]);

    let ip: Result<IPAddrV4, _> = IPAddrV4::try_from("10.0.0.1".to_string());
    assert!(ip.is_ok());
    assert_eq!(ip.unwrap().as_bytes(), &[10, 0, 0, 1]);
  }

  #[test]
  fn test_invalid_format() {
    let ip = IPAddrV4::from_bytes([192, 168, 1, 1, 2, 3, 4, 5]);
    assert!(ip.is_err());

    let ip = IPAddrV4::from_str("10.0.1");
    assert!(ip.is_err());
  
    let ip = IPAddrV4::from_str("10.0.0.x");
    assert!(ip.is_err());
  }

  #[test]
  fn test_display() {
    let ip = IPAddrV4::from_bytes([127, 0, 0, 1]).unwrap();
    assert_eq!(format!("{}", ip), "127.0.0.1");
  }


  #[test]
  fn test_is_local() {
    let ip = IPAddrV4::from_bytes([10, 1, 2, 3]).unwrap();
    assert!(ip.is_local());

    let ip = IPAddrV4::from_bytes([172, 16, 0, 1]).unwrap();
    assert!(ip.is_local());

    let ip = IPAddrV4::from_bytes([192, 168, 1, 100]).unwrap();
    assert!(ip.is_local());

    let ip = IPAddrV4::from_bytes([8, 8, 8, 8]).unwrap();
    assert!(!ip.is_local());
  }
}
