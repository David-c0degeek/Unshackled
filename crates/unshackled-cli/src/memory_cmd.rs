//! `unshackled memory` subcommands: inspect, search, delete, and disable the
//! local project memory store. Memory is project-local by default; a global
//! (user-level) memory would require explicit first-run consent, which is not
//! enabled here.

use std::io::Write;
use std::path::Path;

use unshackled_memory::MemoryStore;

/// Print a one-line status: entry count and whether injection is enabled.
///
/// # Errors
/// Returns an error if the store cannot be read or output written.
pub fn status(root: &Path, out: &mut dyn Write) -> anyhow::Result<()> {
    let store = MemoryStore::open(root);
    let count = store.all()?.len();
    let state = if store.is_enabled() {
        "enabled"
    } else {
        "disabled"
    };
    writeln!(out, "memory: {count} entries ({state})")?;
    Ok(())
}

/// List all entries (id, kind, text).
///
/// # Errors
/// Returns an error if the store cannot be read or output written.
pub fn inspect(root: &Path, out: &mut dyn Write) -> anyhow::Result<()> {
    for entry in MemoryStore::open(root).all()? {
        writeln!(out, "{}  [{:?}]  {}", entry.id, entry.kind, entry.text)?;
    }
    Ok(())
}

/// List entries relevant to a query.
///
/// # Errors
/// Returns an error if the store cannot be read or output written.
pub fn search(root: &Path, query: &str, out: &mut dyn Write) -> anyhow::Result<()> {
    for entry in MemoryStore::open(root).search(query)? {
        writeln!(out, "{}  {}", entry.id, entry.text)?;
    }
    Ok(())
}

/// Delete an entry by id.
///
/// # Errors
/// Returns an error if the store cannot be written or output written.
pub fn delete(root: &Path, id: &str, out: &mut dyn Write) -> anyhow::Result<()> {
    if MemoryStore::open(root).delete(id)? {
        writeln!(out, "deleted {id}")?;
    } else {
        writeln!(out, "no entry with id {id}")?;
    }
    Ok(())
}

/// Disable memory injection for this project.
///
/// # Errors
/// Returns an error if the flag cannot be written.
pub fn disable(root: &Path, out: &mut dyn Write) -> anyhow::Result<()> {
    MemoryStore::open(root).disable()?;
    writeln!(out, "memory injection disabled for this project")?;
    Ok(())
}
