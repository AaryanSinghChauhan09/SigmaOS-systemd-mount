// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-mount: systemd mount component model.
//!
//! Real implementation: mounts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mount {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdMountError {
    EmptyMountName,
    DuplicateMount(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdMountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMountName => write!(f, "mount name must not be empty"),
            Self::DuplicateMount(id) => write!(f, "duplicate mount {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdMountError {}

pub struct SystemdMount {
    pub name: String,
    mounts: BTreeMap<String, Mount>,
    events: BTreeMap<String, Event>,
}

impl SystemdMount {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mounts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_mount(&mut self, item: Mount) -> Result<(), SystemdMountError> {
        if item.name.trim().is_empty() {
            return Err(SystemdMountError::EmptyMountName);
        }
        let id = item.id.clone();
        if self.mounts.contains_key(&id) {
            return Err(SystemdMountError::DuplicateMount(id.clone()));
        }
        self.mounts.insert(id, item);
        Ok(())
    }

    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }

    pub fn get_mount(&self, id: &str) -> Option<&Mount> {
        self.mounts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdMountError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdMountError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_mount(id: &str, name: &str) -> Mount {
        Mount {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdMount::new("SystemdMount");
        assert_eq!(m.name, "SystemdMount");
        assert_eq!(m.mount_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_mount() {
        let mut m = SystemdMount::new("SystemdMount");
        m.add_mount(sample_mount("a1", "Alpha")).unwrap();
        assert_eq!(m.mount_count(), 1);
        assert!(m.get_mount("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdMount::new("SystemdMount");
        assert!(m.add_mount(sample_mount("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdMount::new("SystemdMount");
        m.add_mount(sample_mount("x", "One")).unwrap();
        assert!(m.add_mount(sample_mount("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdMount::new("SystemdMount");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
