use std::collections::HashMap;
use std::path::Path;
use std::result;

pub type Result<T> = result::Result<T, Error>;
pub type RawDesktopEntry = HashMap<String, HashMap<String, String>>;

#[derive(Debug)]
#[allow(unused)]

pub enum Error {
    IoError(std::io::Error),
    FormatError(String),
}

#[derive(Debug, Clone)]
#[allow(unused)]

pub struct ApplicationDesktopEntry {
    pub version: Option<String>,
    pub name: String,
    pub generic_name: Option<String>,
    pub no_display: Option<bool>,
    pub comment: Option<String>,
    pub icon: Option<String>,
    pub hidden: Option<bool>,
    pub only_show_in: Option<String>,
    pub not_show_in: Option<String>,
    pub try_exec: Option<String>,
    pub exec: Option<String>,
    pub path: Option<String>,
    pub terminal: Option<bool>,
    pub actions: Option<String>,
    pub mime_type: Option<String>,
    pub categories: Option<String>,
    pub keywords: Option<String>,
    pub startup_notify: Option<bool>,
    pub startup_wm_class: Option<String>,
    pub prefers_non_default_gpu: Option<bool>,
    pub single_main_window: Option<bool>,
}

#[derive(Debug, Clone)]
#[allow(unused)]

pub struct LinkDesktopEntry {
    pub version: Option<String>,
    pub name: String,
    pub generic_name: Option<String>,
    pub no_display: Option<bool>,
    pub comment: Option<String>,
    pub icon: Option<String>,
    pub hidden: Option<bool>,
    pub only_show_in: Option<String>,
    pub not_show_in: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct DirectoryDesktopEntry {
    pub version: Option<String>,
    pub name: String,
    pub generic_name: Option<String>,
    pub no_display: Option<bool>,
    pub comment: Option<String>,
    pub icon: Option<String>,
    pub hidden: Option<bool>,
    pub only_show_in: Option<String>,
    pub not_show_in: Option<String>,
}

#[derive(Debug)]
#[allow(unused)]

pub enum DesktopEntryType {
    Application(ApplicationDesktopEntry),
    Link(LinkDesktopEntry),
    Directory(DirectoryDesktopEntry),
}

pub fn parse_desktop_entry_raw<P: AsRef<Path>>(path: P) -> Result<RawDesktopEntry> {
    let mut groups: RawDesktopEntry = HashMap::new();
    let mut current_group: String = String::new();

    let content = std::fs::read_to_string(path).map_err(|e| Error::IoError(e))?;

    for line in content.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        };

        if line.starts_with('[') && line.ends_with(']') {
            current_group = line[1..line.len() - 1].to_string();
            continue;
        }

        if current_group.is_empty() {
            return Err(Error::FormatError(
                "Entry found outside of group".to_string(),
            ));
        }

        let entry: Vec<&str> = line.splitn(2, '=').collect();

        if entry.len() != 2 {
            return Err(Error::FormatError("Entry not key/value".to_string()));
        }

        groups
            .entry(current_group.clone())
            .or_insert_with(|| HashMap::new())
            .insert(entry[0].trim().to_string(), entry[1].trim().to_string());
    }

    Ok(groups)
}

pub fn parse_desktop_entry<P: AsRef<Path>>(path: P) -> Result<DesktopEntryType> {
    match parse_desktop_entry_raw(path) {
        Ok(raw_entry) => raw_entry.try_into(),
        Err(error) => Err(error),
    }
}

impl TryFrom<RawDesktopEntry> for DesktopEntryType {
    type Error = Error;

    fn try_from(value: RawDesktopEntry) -> result::Result<Self, Self::Error> {
        let group = value.get("Desktop Entry").ok_or(Error::FormatError(
            "Desktop entry group missing!".to_string(),
        ))?;
        match group
            .get("Type")
            .ok_or(Error::FormatError("Entry type missing!".to_string()))?
            .as_str()
        {
            "Application" => {
                return ApplicationDesktopEntry::try_from(group)
                    .map(|e| DesktopEntryType::Application(e));
            }
            "Link" => {
                return LinkDesktopEntry::try_from(group).map(|e| DesktopEntryType::Link(e));
            }
            "Directory" => {
                return DirectoryDesktopEntry::try_from(group)
                    .map(|e| DesktopEntryType::Directory(e));
            }
            unknown => return Err(Error::FormatError(format!("Unknown entry type {unknown}"))),
        }
    }
}

impl TryFrom<&HashMap<String, String>> for ApplicationDesktopEntry {
    type Error = Error;

    fn try_from(entry: &HashMap<String, String>) -> result::Result<Self, Self::Error> {
        Ok(ApplicationDesktopEntry {
            version: entry.get("Version").cloned(),
            name: entry
                .get("Name")
                .ok_or(Error::FormatError(
                    "Missing required key 'Name'".to_string(),
                ))?
                .to_string(),
            generic_name: entry.get("GenericName").cloned(),
            no_display: entry
                .get("NoDisplay")
                .map(|value| value.parse().is_ok_and(|e| e)),
            comment: entry.get("Comment").cloned(),
            icon: entry.get("Icon").cloned(),
            hidden: entry
                .get("Hidden")
                .map(|value| value.parse().is_ok_and(|e| e)),
            only_show_in: entry.get("OnlyShowIn").cloned(),
            not_show_in: entry.get("NotShowIn").cloned(),
            try_exec: entry.get("TryExec").cloned(),
            exec: entry.get("Exec").cloned(),
            path: entry.get("Path").cloned(),
            terminal: entry
                .get("Terminal")
                .map(|value| value.parse().is_ok_and(|e| e)),
            actions: entry.get("Actions").cloned(),
            mime_type: entry.get("MimeType").cloned(),
            categories: entry.get("Categories").cloned(),
            keywords: entry.get("Keywords").cloned(),
            startup_notify: entry
                .get("StartupNotify")
                .map(|value| value.parse().is_ok_and(|e| e)),
            startup_wm_class: entry.get("StartupWMClass").cloned(),
            prefers_non_default_gpu: entry
                .get("PrefersNonDefaultGPU")
                .map(|value| value.parse().is_ok_and(|e| e)),
            single_main_window: entry
                .get("SingleMainWindow")
                .map(|value| value.parse().is_ok_and(|e| e)),
        })
    }
}

impl TryFrom<&HashMap<String, String>> for LinkDesktopEntry {
    type Error = Error;

    fn try_from(entry: &HashMap<String, String>) -> result::Result<Self, Self::Error> {
        Ok(LinkDesktopEntry {
            version: entry.get("Version").cloned(),
            name: entry
                .get("Name")
                .ok_or(Error::FormatError(
                    "Missing required key 'Name'".to_string(),
                ))?
                .to_string(),
            generic_name: entry.get("GenericName").cloned(),
            no_display: entry
                .get("NoDisplay")
                .map(|value| value.parse().is_ok_and(|e| e)),
            comment: entry.get("Comment").cloned(),
            icon: entry.get("Icon").cloned(),
            hidden: entry
                .get("Hidden")
                .map(|value| value.parse().is_ok_and(|e| e)),
            only_show_in: entry.get("OnlyShowIn").cloned(),
            not_show_in: entry.get("NotShowIn").cloned(),
            url: entry
                .get("URL")
                .ok_or(Error::FormatError("Missing required key 'URL'".to_string()))?
                .to_string(),
        })
    }
}

impl TryFrom<&HashMap<String, String>> for DirectoryDesktopEntry {
    type Error = Error;

    fn try_from(entry: &HashMap<String, String>) -> result::Result<Self, Self::Error> {
        Ok(DirectoryDesktopEntry {
            version: entry.get("Version").cloned(),
            name: entry
                .get("Name")
                .ok_or(Error::FormatError(
                    "Missing required key 'Name'".to_string(),
                ))?
                .to_string(),
            generic_name: entry.get("GenericName").cloned(),
            no_display: entry
                .get("NoDisplay")
                .map(|value| value.parse().is_ok_and(|e| e)),
            comment: entry.get("Comment").cloned(),
            icon: entry.get("Icon").cloned(),
            hidden: entry
                .get("Hidden")
                .map(|value| value.parse().is_ok_and(|e| e)),
            only_show_in: entry.get("OnlyShowIn").cloned(),
            not_show_in: entry.get("NotShowIn").cloned(),
        })
    }
}
