// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-application-uninstallation: application uninstallation component model.
//!
//! Real implementation: uninstallations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uninstallation {
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
pub enum ApplicationUninstallationError {
    EmptyUninstallationName,
    DuplicateUninstallation(String),
    EmptyEventSubject,
}

impl fmt::Display for ApplicationUninstallationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUninstallationName => write!(f, "uninstallation name must not be empty"),
            Self::DuplicateUninstallation(id) => write!(f, "duplicate uninstallation {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ApplicationUninstallationError {}

pub struct ApplicationUninstallation {
    pub name: String,
    uninstallations: BTreeMap<String, Uninstallation>,
    events: BTreeMap<String, Event>,
}

impl ApplicationUninstallation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uninstallations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_uninstallation(&mut self, item: Uninstallation) -> Result<(), ApplicationUninstallationError> {
        if item.name.trim().is_empty() {
            return Err(ApplicationUninstallationError::EmptyUninstallationName);
        }
        let id = item.id.clone();
        if self.uninstallations.contains_key(&id) {
            return Err(ApplicationUninstallationError::DuplicateUninstallation(id.clone()));
        }
        self.uninstallations.insert(id, item);
        Ok(())
    }

    pub fn uninstallation_count(&self) -> usize {
        self.uninstallations.len()
    }

    pub fn get_uninstallation(&self, id: &str) -> Option<&Uninstallation> {
        self.uninstallations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ApplicationUninstallationError> {
        if event.subject.trim().is_empty() {
            return Err(ApplicationUninstallationError::EmptyEventSubject);
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

    fn sample_uninstallation(id: &str, name: &str) -> Uninstallation {
        Uninstallation {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ApplicationUninstallation::new("ApplicationUninstallation");
        assert_eq!(m.name, "ApplicationUninstallation");
        assert_eq!(m.uninstallation_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_uninstallation() {
        let mut m = ApplicationUninstallation::new("ApplicationUninstallation");
        m.add_uninstallation(sample_uninstallation("a1", "Alpha")).unwrap();
        assert_eq!(m.uninstallation_count(), 1);
        assert!(m.get_uninstallation("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ApplicationUninstallation::new("ApplicationUninstallation");
        assert!(m.add_uninstallation(sample_uninstallation("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ApplicationUninstallation::new("ApplicationUninstallation");
        m.add_uninstallation(sample_uninstallation("x", "One")).unwrap();
        assert!(m.add_uninstallation(sample_uninstallation("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ApplicationUninstallation::new("ApplicationUninstallation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
